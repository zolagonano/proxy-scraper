use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a VMess proxy.
#[derive(Debug, Serialize, Deserialize)]
pub struct VMess {
    pub add: String,
    pub host: Option<String>,
    pub id: String,
    pub port: serde_json::Value,
    pub net: String,
    pub sni: Option<String>,
    pub tls: Option<String>,
    #[serde(flatten)]
    pub metadata: Option<HashMap<String, String>>,
}

impl VMess {
    pub fn to_url(&self) -> String {
        let base64_part = URL_SAFE.encode(serde_json::to_vec_pretty(&self).unwrap());
        format!("vmess://{}", base64_part)
    }

    pub fn scrape(source: &str) -> Vec<Self> {
        let source = crate::utils::seperate_links(source);
        let mut proxy_list: Vec<VMess> = Vec::new();
        let regex = Regex::new(
            r#"vmess://((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?)"#,
        )
        .unwrap();

        for captures in regex.captures_iter(&source) {
            let base64_part = captures.get(1).map(|b64| b64.as_str()).unwrap_or("");

            if base64_part.is_empty() {
                continue;
            }

            if let Ok(decoded_base64_part) = URL_SAFE.decode(&base64_part) {
                let json_string = String::from_utf8(decoded_base64_part).unwrap();

                let deserialized_vmess: VMess = serde_json::from_str(&json_string).unwrap();

                proxy_list.push(deserialized_vmess);
            }
        }

        proxy_list
    }
}
