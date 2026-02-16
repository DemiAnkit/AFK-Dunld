// Commands for security features
use tauri::State;
use crate::state::app_state::AppState;
use crate::utils::security::{EncryptedCredential, InputValidator};

#[tauri::command]
pub async fn encrypt_credential(
    state: State<'_, AppState>,
    plaintext: String,
) -> Result<EncryptedCredential, String> {
    state.credential_vault
        .encrypt(&plaintext)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn decrypt_credential(
    state: State<'_, AppState>,
    encrypted: EncryptedCredential,
) -> Result<String, String> {
    state.credential_vault
        .decrypt(&encrypted)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_url(
    url: String,
) -> Result<(), String> {
    InputValidator::validate_url(&url)
}

#[tauri::command]
pub async fn validate_file_path(
    path: String,
) -> Result<(), String> {
    InputValidator::validate_file_path(&path)
}

#[tauri::command]
pub async fn validate_category_name(
    name: String,
) -> Result<(), String> {
    InputValidator::validate_category_name(&name)
}

#[tauri::command]
pub async fn validate_color(
    color: String,
) -> Result<(), String> {
    InputValidator::validate_color(&color)
}

#[tauri::command]
pub async fn sanitize_input(
    input: String,
) -> Result<String, String> {
    Ok(InputValidator::sanitize_input(&input))
}

#[tauri::command]
pub async fn check_rate_limit(
    state: State<'_, AppState>,
    key: String,
) -> Result<bool, String> {
    Ok(state.rate_limiter.check_rate_limit(&key).await)
}
