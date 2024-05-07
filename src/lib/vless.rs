use crate::Proxy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a VLess proxy.
#[derive(Debug, Serialize, Deserialize)]
pub struct VLess {
    /// The host address of the VLess proxy.
    pub host: String,
    /// The port number for the VLess proxy.
    pub port: u32,
    /// The UUID of the VLess proxy.
    pub id: String,
    /// Additional parameters associated with the VLess proxy.
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl Proxy for VLess {
    /// Converts the VLess proxy information into a VLess URL.
    ///
    /// # Example
    ///
    /// ```
    /// use proxy_scraper::vless::VLess;
    /// use proxy_scraper::Proxy;
    /// let proxy = VLess {
    ///     host: "example.com".to_string(),
    ///     port: 443,
    ///     id: "00000000-0000-0000-0000-000000000000".to_string(),
    ///     parameters: None,
    /// };
    /// let url = proxy.to_url();
    /// assert_eq!(url, "vless://00000000-0000-0000-0000-000000000000@example.com:443?");
    /// ```
    fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        format!(
            "vless://{}@{}:{}?{}",
            self.id, self.host, self.port, url_encoded_parameters
        )
    }

    /// Scrapes VLess proxy information from the provided source string and returns a vector of VLess instances.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the source code or text from which to extract VLess proxy information.
    ///
    /// # Returns
    ///
    /// A vector of `VLess` instances parsed from the source string.
    fn scrape(source: &str) -> Vec<impl Proxy> {
        let source = crate::utils::seperate_links(source);
        let mut proxy_list: Vec<VLess> = Vec::new();
        let regex = Regex::new(r#"vless://([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})@((.+):(\d+))\?(.+)#"#).unwrap();

        for captures in regex.captures_iter(&source) {
            let uuid = captures.get(1).map(|id| id.as_str()).unwrap_or("");
            let host = captures.get(3).map(|host| host.as_str()).unwrap_or("");
            let port = captures.get(4).map(|port| port.as_str()).unwrap_or("");
            let url_parameters = captures.get(5).map(|params| params.as_str()).unwrap_or("");

            if uuid.is_empty() || host.is_empty() || port.is_empty() || url_parameters.is_empty() {
                continue;
            }

            let parameters: HashMap<String, String> =
                serde_urlencoded::from_str(&url_parameters).unwrap();

            let vless_proxy = VLess {
                host: host.to_string(),
                port: port.parse::<u32>().unwrap_or(0),
                id: uuid.to_string(),
                parameters: Some(parameters),
            };

            proxy_list.push(vless_proxy);
        }

        proxy_list
    }
}
