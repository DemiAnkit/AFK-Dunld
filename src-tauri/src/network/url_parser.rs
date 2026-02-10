// src-tauri/src/network/url_parser.rs

use url::Url;
use regex::Regex;
use crate::utils::error::DownloadError;

#[derive(Debug, Clone)]
pub struct ParsedUrl {
    pub url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub filename: String,
    pub extension: Option<String>,
}

pub struct UrlParser;

impl UrlParser {
    /// Parse and validate a URL
    pub fn parse(raw_url: &str) -> Result<ParsedUrl, DownloadError> {
        let trimmed = raw_url.trim();

        // Add scheme if missing
        let url_str = if !trimmed.contains("://") {
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        let parsed = Url::parse(&url_str)
            .map_err(|e| DownloadError::InvalidUrl(
                format!("{}: {}", raw_url, e)
            ))?;

        // Validate scheme
        let scheme = parsed.scheme().to_lowercase();
        if !["http", "https", "ftp", "ftps"].contains(&scheme.as_str()) {
            return Err(DownloadError::InvalidUrl(
                format!("Unsupported scheme: {}", scheme)
            ));
        }

        // Validate host
        let host = parsed.host_str()
            .ok_or_else(|| DownloadError::InvalidUrl(
                "No host in URL".to_string()
            ))?
            .to_string();

        let path = parsed.path().to_string();

        // Extract filename
        let filename = Self::extract_filename_from_url(&parsed);

        // Extract extension
        let extension = filename
            .rsplit('.')
            .next()
            .filter(|ext| ext.len() <= 10 && !ext.contains('/'))
            .map(|s| s.to_lowercase());

        Ok(ParsedUrl {
            url: url_str,
            scheme,
            host,
            path,
            filename,
            extension,
        })
    }

    /// Extract filename from URL path
    fn extract_filename_from_url(url: &Url) -> String {
        url.path_segments()
            .and_then(|segments| segments.last())
            .filter(|s| !s.is_empty())
            .map(|s| {
                // URL decode
                urlencoding_decode(s)
            })
            .unwrap_or_else(|| "download".to_string())
    }

    /// Extract filename from Content-Disposition header
    pub fn extract_filename_from_header(header: &str) -> Option<String> {
        // Try filename*= (RFC 5987)
        let re_star = Regex::new(
            r#"filename\*\s*=\s*(?:UTF-8|utf-8)''(.+?)(?:;|$)"#
        ).ok()?;
        if let Some(captures) = re_star.captures(header) {
            if let Some(name) = captures.get(1) {
                return Some(urlencoding_decode(name.as_str()));
            }
        }

        // Try filename="..."
        let re_quoted = Regex::new(
            r#"filename\s*=\s*"([^"]+)""#
        ).ok()?;
        if let Some(captures) = re_quoted.captures(header) {
            if let Some(name) = captures.get(1) {
                return Some(name.as_str().to_string());
            }
        }

        // Try filename=...
        let re_unquoted = Regex::new(
            r#"filename\s*=\s*([^\s;]+)"#
        ).ok()?;
        if let Some(captures) = re_unquoted.captures(header) {
            if let Some(name) = captures.get(1) {
                return Some(name.as_str().to_string());
            }
        }

        None
    }

    /// Check if a string looks like a downloadable URL
    pub fn is_downloadable_url(text: &str) -> bool {
        let trimmed = text.trim();

        // Must start with http:// or https://
        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            return false;
        }

        // Must be a valid URL
        if Url::parse(trimmed).is_err() {
            return false;
        }

        // Check for common downloadable extensions
        let download_extensions = [
            "zip", "rar", "7z", "tar", "gz", "bz2", "xz",
            "exe", "msi", "dmg", "deb", "rpm", "appimage",
            "iso", "img",
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
            "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm",
            "mp3", "flac", "wav", "aac", "ogg", "wma",
            "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp",
            "apk", "ipa",
            "torrent",
        ];

        let lower = trimmed.to_lowercase();
        for ext in &download_extensions {
            if lower.ends_with(&format!(".{}", ext)) {
                return true;
            }
        }

        // Could still be a downloadable URL even without extension
        // (e.g., redirected URLs)
        true
    }
}

/// Simple URL decoding
fn urlencoding_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_url() {
        let result = UrlParser::parse(
            "https://example.com/files/test.zip"
        ).unwrap();
        assert_eq!(result.filename, "test.zip");
        assert_eq!(result.extension, Some("zip".to_string()));
        assert_eq!(result.scheme, "https");
    }

    #[test]
    fn test_parse_url_without_scheme() {
        let result = UrlParser::parse("example.com/file.zip").unwrap();
        assert_eq!(result.scheme, "https");
    }

    #[test]
    fn test_extract_content_disposition() {
        let header = r#"attachment; filename="my file.zip""#;
        let name = UrlParser::extract_filename_from_header(header);
        assert_eq!(name, Some("my file.zip".to_string()));
    }

    #[test]
    fn test_downloadable_url() {
        assert!(UrlParser::is_downloadable_url(
            "https://example.com/file.zip"
        ));
        assert!(!UrlParser::is_downloadable_url("not a url"));
    }
}