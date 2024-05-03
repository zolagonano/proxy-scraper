use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VLess {
    pub host: String,
    pub port: u32,
    pub id: String,
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl VLess {
    pub fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        format!(
            "vless://{}@{}:{}?{}",
            self.id, self.host, self.port, url_encoded_parameters
        )
    }

    pub fn scrape(source: &str) -> Vec<Self> {
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
