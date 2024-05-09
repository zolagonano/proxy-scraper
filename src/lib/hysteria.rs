use crate::Proxy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Hysteria proxy.
#[derive(Debug, Serialize, Deserialize)]
pub struct Hysteria {
    /// The version of the Hysteria protocol.
    pub version: u8,
    /// The host address of the Hysteria proxy.
    pub host: String,
    /// The port number for the Hysteria proxy.
    pub port: u32,
    /// The authentication string associated with the Hysteria proxy.
    pub auth: String,
    /// Additional parameters associated with the Hysteria proxy.
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl Proxy for Hysteria {
    /// Converts the Hysteria proxy information into a Hysteria URL.
    ///
    /// # Example
    ///
    /// ```
    /// use proxy_scraper::Proxy;
    /// use proxy_scraper::hysteria::Hysteria;
    /// use std::collections::HashMap;
    /// let proxy = Hysteria {
    ///     version: 1,
    ///     host: "example.com".to_string(),
    ///     port: 443,
    ///     auth: "auth123".to_string(),
    ///     parameters: Some(HashMap::new()), // Insert additional parameters here
    /// };
    /// let url = proxy.to_url();
    /// assert_eq!(url, "hysteria://auth123@example.com:443?");
    /// ```
    fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        let hysteria_version = match self.version {
            1 => "hysteria",
            _ => "hy2",
        };

        format!(
            "{}://{}@{}:{}?{}",
            hysteria_version, self.auth, self.host, self.port, url_encoded_parameters
        )
    }

    /// Scrapes Hysteria proxy information from the provided source string and returns a vector of Hysteria instances.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the source code or text from which to extract Hysteria proxy information.
    ///
    /// # Returns
    ///
    /// A vector of `Hysteria` instances parsed from the source string.
    fn scrape(source: &str) -> Vec<impl Proxy> {
        let source = crate::utils::seperate_links(source);
        let mut proxy_list: Vec<Hysteria> = Vec::new();
        let regex =
            Regex::new(r#"(hy2|hysteria2|hysteria)://([A-Za-z0-9\-._~]+)@((.+):(\d+))\?(.+)#"#)
                .unwrap();

        for captures in regex.captures_iter(&source) {
            let version = captures.get(1).map(|ver| ver.as_str()).unwrap_or("");
            let auth = captures.get(2).map(|auth| auth.as_str()).unwrap_or("");
            let host = captures.get(4).map(|host| host.as_str()).unwrap_or("");
            let port = captures.get(5).map(|port| port.as_str()).unwrap_or("");
            let url_parameters = captures.get(6).map(|params| params.as_str()).unwrap_or("");

            if version.is_empty()
                || auth.is_empty()
                || host.is_empty()
                || port.is_empty()
                || url_parameters.is_empty()
            {
                continue;
            }

            let parameters: HashMap<String, String> =
                serde_urlencoded::from_str(&url_parameters).unwrap();

            let hysteria_version = match version {
                "hy2" | "hysteria2" => 2,
                _ => 1,
            };

            let hysteria_proxy = Hysteria {
                version: hysteria_version,
                host: host.to_string(),
                port: port.parse::<u32>().unwrap_or(0),
                auth: auth.to_string(),
                parameters: Some(parameters),
            };

            proxy_list.push(hysteria_proxy);
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
        "TCP".to_string()
    }

    fn get_security(&self) -> String {
        "TLS".to_string()
    }

    fn get_type(&self) -> &str {
        match self.version {
            1 => "HYSTERIA1",
            _ => "HYSTERIA2",
        }
    }
}
