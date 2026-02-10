// src-tauri/src/network/proxy_manager.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyType {
    Http,
    Https,
    Socks5,
}

impl ProxyConfig {
    pub fn to_url(&self) -> Option<String> {
        if !self.enabled {
            return None;
        }

        let scheme = match self.proxy_type {
            ProxyType::Http => "http",
            ProxyType::Https => "https",
            ProxyType::Socks5 => "socks5",
        };

        let auth = match (&self.username, &self.password) {
            (Some(user), Some(pass)) => format!("{}:{}@", user, pass),
            (Some(user), None) => format!("{}@", user),
            _ => String::new(),
        };

        Some(format!("{}://{}{}:{}", scheme, auth, self.host, self.port))
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_type: ProxyType::Http,
            host: String::new(),
            port: 8080,
            username: None,
            password: None,
        }
    }
}