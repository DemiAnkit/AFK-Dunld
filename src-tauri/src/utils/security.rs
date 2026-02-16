// Security utilities for credential encryption and input validation
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng as Argon2OsRng};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Encrypted credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredential {
    pub encrypted_data: String, // Base64 encoded
    pub nonce: String,           // Base64 encoded
}

/// Credential encryption service
pub struct CredentialVault {
    key: Arc<[u8; 32]>,
}

impl CredentialVault {
    /// Create a new vault with a derived key from password
    pub fn new(master_password: &str) -> Result<Self, String> {
        // Derive a key from the master password
        let salt = SaltString::generate(&mut Argon2OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(master_password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        // Extract the hash bytes (first 32 bytes for AES-256)
        let hash_option = password_hash.hash
            .ok_or("No hash produced")?;
        let hash_bytes = hash_option.as_bytes();
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash_bytes[..32]);

        Ok(Self {
            key: Arc::new(key),
        })
    }

    /// Create from existing key (for testing)
    pub fn from_key(key: [u8; 32]) -> Self {
        Self {
            key: Arc::new(key),
        }
    }

    /// Encrypt a credential
    pub fn encrypt(&self, plaintext: &str) -> Result<EncryptedCredential, String> {
        use aes_gcm::aead::rand_core::RngCore;
        
        let cipher = Aes256Gcm::new(self.key.as_ref().into());
        
        // Generate a random nonce manually
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok(EncryptedCredential {
            encrypted_data: general_purpose::STANDARD.encode(&ciphertext),
            nonce: general_purpose::STANDARD.encode(&nonce_bytes),
        })
    }

    /// Decrypt a credential
    pub fn decrypt(&self, encrypted: &EncryptedCredential) -> Result<String, String> {
        let cipher = Aes256Gcm::new(self.key.as_ref().into());
        
        let ciphertext = general_purpose::STANDARD
            .decode(&encrypted.encrypted_data)
            .map_err(|e| format!("Failed to decode ciphertext: {}", e))?;
        
        let nonce_bytes = general_purpose::STANDARD
            .decode(&encrypted.nonce)
            .map_err(|e| format!("Failed to decode nonce: {}", e))?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext)
            .map_err(|e| format!("Invalid UTF-8: {}", e))
    }
}

/// Input validator
pub struct InputValidator;

impl InputValidator {
    /// Validate URL
    pub fn validate_url(url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Check length
        if url.len() > 2048 {
            return Err("URL is too long (max 2048 characters)".to_string());
        }

        // Parse URL
        url::Url::parse(url)
            .map_err(|e| format!("Invalid URL: {}", e))?;

        Ok(())
    }

    /// Validate file path
    pub fn validate_file_path(path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }

        // Check for path traversal attempts
        if path.contains("..") {
            return Err("Path traversal not allowed".to_string());
        }

        // Check for null bytes
        if path.contains('\0') {
            return Err("Null bytes not allowed in path".to_string());
        }

        Ok(())
    }

    /// Validate category name
    pub fn validate_category_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Category name cannot be empty".to_string());
        }

        if name.len() > 100 {
            return Err("Category name is too long (max 100 characters)".to_string());
        }

        // Check for invalid characters
        if name.chars().any(|c| c.is_control()) {
            return Err("Category name contains invalid characters".to_string());
        }

        Ok(())
    }

    /// Validate color hex code
    pub fn validate_color(color: &str) -> Result<(), String> {
        if !color.starts_with('#') {
            return Err("Color must start with #".to_string());
        }

        let hex = &color[1..];
        if hex.len() != 6 {
            return Err("Color must be in #RRGGBB format".to_string());
        }

        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Color contains invalid hex digits".to_string());
        }

        Ok(())
    }

    /// Sanitize user input (remove dangerous characters)
    pub fn sanitize_input(input: &str) -> String {
        input
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect()
    }
}

/// Rate limiter using token bucket algorithm
pub struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    max_requests: usize,
    window_duration: Duration,
}

struct TokenBucket {
    tokens: usize,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_duration: Duration) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window_duration,
        }
    }

    /// Check if a request is allowed
    pub async fn check_rate_limit(&self, key: &str) -> bool {
        let mut buckets = self.buckets.write().await;
        let now = Instant::now();

        let bucket = buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
            tokens: self.max_requests,
            last_refill: now,
        });

        // Refill tokens based on elapsed time
        let elapsed = now.duration_since(bucket.last_refill);
        if elapsed >= self.window_duration {
            bucket.tokens = self.max_requests;
            bucket.last_refill = now;
        }

        // Check if we have tokens
        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Reset rate limit for a key
    pub async fn reset(&self, key: &str) {
        let mut buckets = self.buckets.write().await;
        buckets.remove(key);
    }

    /// Clear all rate limits
    pub async fn clear_all(&self) {
        let mut buckets = self.buckets.write().await;
        buckets.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let vault = CredentialVault::from_key([42u8; 32]);
        let plaintext = "my_secret_password";

        let encrypted = vault.encrypt(plaintext).unwrap();
        let decrypted = vault.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_url_validation() {
        assert!(InputValidator::validate_url("https://example.com").is_ok());
        assert!(InputValidator::validate_url("ftp://ftp.example.com/file.zip").is_ok());
        assert!(InputValidator::validate_url("").is_err());
        assert!(InputValidator::validate_url("not a url").is_err());
    }

    #[test]
    fn test_path_validation() {
        assert!(InputValidator::validate_file_path("/home/user/file.txt").is_ok());
        assert!(InputValidator::validate_file_path("../etc/passwd").is_err());
        assert!(InputValidator::validate_file_path("file\0name").is_err());
    }

    #[test]
    fn test_category_name_validation() {
        assert!(InputValidator::validate_category_name("My Category").is_ok());
        assert!(InputValidator::validate_category_name("").is_err());
        assert!(InputValidator::validate_category_name(&"a".repeat(101)).is_err());
    }

    #[test]
    fn test_color_validation() {
        assert!(InputValidator::validate_color("#FF0000").is_ok());
        assert!(InputValidator::validate_color("#123ABC").is_ok());
        assert!(InputValidator::validate_color("FF0000").is_err());
        assert!(InputValidator::validate_color("#GGGGGG").is_err());
        assert!(InputValidator::validate_color("#FFF").is_err());
    }

    #[test]
    fn test_sanitize_input() {
        let input = "Hello\x00World\x01Test";
        let sanitized = InputValidator::sanitize_input(input);
        assert_eq!(sanitized, "HelloWorldTest");
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, Duration::from_secs(1));

        assert!(limiter.check_rate_limit("user1").await);
        assert!(limiter.check_rate_limit("user1").await);
        assert!(limiter.check_rate_limit("user1").await);
        assert!(!limiter.check_rate_limit("user1").await); // Should be blocked

        // Different user should have separate limit
        assert!(limiter.check_rate_limit("user2").await);
    }

    #[tokio::test]
    async fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(2, Duration::from_millis(100));

        assert!(limiter.check_rate_limit("user1").await);
        assert!(limiter.check_rate_limit("user1").await);
        assert!(!limiter.check_rate_limit("user1").await);

        // Wait for refill
        tokio::time::sleep(Duration::from_millis(150)).await;

        assert!(limiter.check_rate_limit("user1").await); // Should work again
    }
}
