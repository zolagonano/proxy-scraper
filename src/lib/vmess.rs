use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a VMess proxy.
#[derive(Debug, Serialize, Deserialize)]
pub struct VMess {
    /// The address of the VMess server.
    pub add: String,
    /// The optional host address of the VMess server.
    pub host: Option<String>,
    /// The UUID of the VMess server.
    pub id: String,
    /// The port number of the VMess server.
    pub port: serde_json::Value,
    /// The network type of the VMess server.
    pub net: String,
    /// The optional SNI (Server Name Indication) of the VMess server.
    pub sni: Option<String>,
    /// The optional TLS (Transport Layer Security) of the VMess server.
    pub tls: Option<String>,
    /// Additional metadata associated with the VMess server.
    #[serde(flatten)]
    pub metadata: Option<HashMap<String, String>>,
}

impl VMess {
    /// Converts the VMess proxy information into a VMess URL.
    ///
    /// # Example
    ///
    /// ```
    /// use proxy_scraper::vmess::VMess;
    /// let proxy = VMess {
    ///     add: "example.com".to_string(),
    ///     host: Some("www.example.com".to_string()),
    ///     id: "uuid".to_string(),
    ///     port: serde_json::json!(443),
    ///     net: "tcp".to_string(),
    ///     sni: Some("example.com".to_string()),
    ///     tls: Some("tls".to_string()),
    ///     metadata: None,
    /// };
    /// let url = proxy.to_url();
    /// assert_eq!(url, "vmess://ewogICJhZGQiOiAiZXhhbXBsZS5jb20iLAogICJob3N0IjogInd3dy5leGFtcGxlLmNvbSIsCiAgImlkIjogInV1aWQiLAogICJwb3J0IjogNDQzLAogICJuZXQiOiAidGNwIiwKICAic25pIjogImV4YW1wbGUuY29tIiwKICAidGxzIjogInRscyIKfQ==");
    /// ```
    pub fn to_url(&self) -> String {
        let base64_part = URL_SAFE.encode(serde_json::to_vec_pretty(&self).unwrap());
        format!("vmess://{}", base64_part)
    }

    /// Scrapes VMess proxy information from the provided source string and returns a vector of VMess instances.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the source code or text from which to extract VMess proxy information.
    ///
    /// # Returns
    ///
    /// A vector of `VMess` instances parsed from the source string.
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
