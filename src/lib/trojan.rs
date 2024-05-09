use crate::Proxy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Trojan proxy.
#[derive(Debug, Serialize, Deserialize)]
pub struct Trojan {
    /// The host address of the Trojan proxy.
    pub host: String,
    /// The port number for the Trojan proxy.
    pub port: u32,
    /// The password associated with the Trojan proxy.
    pub password: String,
    /// Additional parameters associated with the Trojan proxy.
    #[serde(flatten)]
    pub parameters: HashMap<String, String>,
}

impl Proxy for Trojan {
    /// Converts the Trojan proxy information into a Trojan URL.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use proxy_scraper::trojan::Trojan;
    /// use proxy_scraper::Proxy;
    /// let proxy = Trojan {
    ///     host: "example.com".to_string(),
    ///     port: 443,
    ///     password: "password123".to_string(),
    ///     parameters: HashMap::new(), // Insert additional parameters here
    /// };
    /// let url = proxy.to_url();
    /// assert_eq!(url, "trojan://password123@example.com:443?");
    /// ```
    fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        format!(
            "trojan://{}@{}:{}?{}",
            self.password, self.host, self.port, url_encoded_parameters
        )
    }

    /// Scrapes Trojan proxy information from the provided source string and returns a vector of Trojan instances.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the source code or text from which to extract Trojan proxy information.
    ///
    /// # Returns
    ///
    /// A vector of `Trojan` instances parsed from the source string.
    fn scrape(source: &str) -> Vec<impl Proxy> {
        let source = crate::utils::seperate_links(source);
        let mut proxy_list: Vec<Trojan> = Vec::new();
        let regex = Regex::new(r#"trojan://([A-Za-z0-9\-._~]+)@((.+):(\d+))\?(.+)#"#).unwrap();

        for captures in regex.captures_iter(&source) {
            let password = captures.get(1).map(|pass| pass.as_str()).unwrap_or("");
            let host = captures.get(3).map(|host| host.as_str()).unwrap_or("");
            let port = captures.get(4).map(|port| port.as_str()).unwrap_or("");
            let url_parameters = captures.get(5).map(|params| params.as_str()).unwrap_or("");

            if password.is_empty()
                || host.is_empty()
                || port.is_empty()
                || url_parameters.is_empty()
            {
                continue;
            }

            let parameters: HashMap<String, String> =
                serde_urlencoded::from_str(&url_parameters).unwrap();

            let trojan_proxy = Trojan {
                host: host.to_string(),
                port: port.parse::<u32>().unwrap_or(0),
                password: password.to_string(),
                parameters: parameters,
            };

            proxy_list.push(trojan_proxy);
        }

        proxy_list
    }
    fn get_host(&self) -> &str {
        &self.host
    }

    fn get_port(&self) -> u32 {
        self.port
    }

    fn get_network(&self) -> String {
        self.parameters
            .get("type")
            .unwrap_or(&"TCP".to_string())
            .to_uppercase()
    }

    fn get_security(&self) -> String {
        self.parameters
            .get("security")
            .unwrap_or(&"NONE".to_string())
            .to_uppercase()
    }

    fn get_type(&self) -> &str {
        "TR"
    }
}
