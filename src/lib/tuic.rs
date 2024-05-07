use crate::Proxy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a TUIC proxy.
#[derive(Debug, Serialize, Deserialize)]
pub struct TUIC {
    /// The host address of the TUIC proxy.
    pub host: String,
    /// The port number for the TUIC proxy.
    pub port: u32,
    /// The authentication string associated with the TUIC proxy.
    pub auth: String,
    /// Additional parameters associated with the TUIC proxy.
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl Proxy for TUIC {
    /// Converts the TUIC proxy information into a TUIC URL.
    ///
    /// # Example
    ///
    /// ```
    /// use proxy_scraper::tuic::TUIC;
    /// use std::collections::HashMap;
    /// use proxy_scraper::Proxy;
    /// let proxy = TUIC {
    ///     host: "example.com".to_string(),
    ///     port: 443,
    ///     auth: "auth123".to_string(),
    ///     parameters: Some(HashMap::new()), // Insert additional parameters here
    /// };
    /// let url = proxy.to_url();
    /// assert_eq!(url, "tuic://auth123@example.com:443?");
    /// ```
    fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        format!(
            "tuic://{}@{}:{}?{}",
            self.auth, self.host, self.port, url_encoded_parameters
        )
    }

    /// Scrapes TUIC proxy information from the provided source string and returns a vector of TUIC instances.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the source code or text from which to extract TUIC proxy information.
    ///
    /// # Returns
    ///
    /// A vector of `TUIC` instances parsed from the source string.
    fn scrape(source: &str) -> Vec<impl Proxy> {
        let source = crate::utils::seperate_links(source);
        let mut proxy_list: Vec<TUIC> = Vec::new();
        let regex = Regex::new(r#"tuic://([A-Za-z0-9\-._~]+)@((.+):(\d+))\?(.+)#"#).unwrap();

        for captures in regex.captures_iter(&source) {
            let auth = captures.get(1).map(|auth| auth.as_str()).unwrap_or("");
            let host = captures.get(3).map(|host| host.as_str()).unwrap_or("");
            let port = captures.get(4).map(|port| port.as_str()).unwrap_or("");
            let url_parameters = captures.get(5).map(|params| params.as_str()).unwrap_or("");

            if auth.is_empty() || host.is_empty() || port.is_empty() || url_parameters.is_empty() {
                continue;
            }

            let parameters: HashMap<String, String> =
                serde_urlencoded::from_str(&url_parameters).unwrap();

            let tuic_proxy = TUIC {
                host: host.to_string(),
                port: port.parse::<u32>().unwrap_or(0),
                auth: auth.to_string(),
                parameters: Some(parameters),
            };

            proxy_list.push(tuic_proxy);
        }

        proxy_list
    }
}
