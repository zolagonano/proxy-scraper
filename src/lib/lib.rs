pub mod mtproxy;
pub mod shadowsocks;
pub mod vmess;
pub mod vless;
pub mod trojan;
pub mod hysteria;
pub mod utils;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "scraper")]

pub trait Proxy {}


#[derive(Debug, Serialize, Deserialize)]
pub struct TUIC {
    pub host: String,
    pub port: u32,
    pub auth: String,
    #[serde(flatten)]
    pub parameters: Option<HashMap<String, String>>,
}

impl TUIC {
    pub fn to_url(&self) -> String {
        let url_encoded_parameters = serde_urlencoded::to_string(&self.parameters).unwrap();
        format!(
            "tuic://{}@{}:{}?{}",
            self.auth, self.host, self.port, url_encoded_parameters
        )
    }

    pub fn scrape(source: &str) -> Vec<Self> {
        Scraper::scrape_tuic(source)
    }
}

/// A scraper for extracting MTProxy information from a given source string.
pub struct Scraper();

impl Scraper {
    fn seperate_links(text: &str) -> String {
        let regex = Regex::new(
            r#"\b(https|ss|vmess|vless|trojan|hysteria2|hy2|hysteria)?://[^\s<>"']+[^.,;!?)"'\s]"#,
        )
        .unwrap();
        let mut links = String::new();
        for cap in regex.captures_iter(text) {
            links.push_str(&cap[0].replace("&amp;amp;", "&").replace("%3D", "="));
            links.push('\n');
        }
        links
    }

    pub fn scrape_tuic(source: &str) -> Vec<TUIC> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<TUIC> = Vec::new();
        let regex = Regex::new(r#"tuic://([A-Za-z0-9\-._~]+)@((.+):(\d+))\?(.+)#"#).unwrap();

        for captures in regex.captures_iter(source) {
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
