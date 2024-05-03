use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hysteria {
    pub version: u8,
    pub host: String,
    pub port: u32,
    pub auth: String,
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl Hysteria {
    pub fn to_url(&self) -> String {
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

    pub fn scrape(source: &str) -> Vec<Self> {
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
}