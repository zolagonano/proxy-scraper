use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trojan {
    pub host: String,
    pub port: u32,
    pub password: String,
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl Trojan {
    pub fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        format!(
            "trojan://{}@{}:{}?{}",
            self.password, self.host, self.port, url_encoded_parameters
        )
    }

    pub fn scrape(source: &str) -> Vec<Self> {
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
                parameters: Some(parameters),
            };

            proxy_list.push(trojan_proxy);
        }

        proxy_list
    }
}
