use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "scraper")]

pub trait Proxy {}

/// Represents an MTProxy, a specific type of proxy.
#[derive(Debug)]
pub struct MTProxy {
    /// The host address of the MTProxy.
    pub host: String,
    /// The port number for the MTProxy.
    pub port: u32,
    /// The secret associated with the MTProxy.
    pub secret: String,
}

impl MTProxy {
    pub fn to_url(&self) -> String {
        format!(
            "https://t.me/proxy?server={}&port={}&secret={}",
            self.host, self.port, self.secret
        )
    }

    pub fn scrape(source: &str) -> Vec<Self> {
        Scraper::scrape_mtproxy(source)
    }
}

/// Represents a Shadowsocks proxy.
#[derive(Debug)]
pub struct Shadowsocks {
    /// The host address of the Shadowsocks proxy.
    pub host: String,
    /// The port number for the Shadowsocks proxy.
    pub port: u32,
    /// The password associated with the Shadowsocks proxy.
    pub password: String,
    /// The encryption method used by the Shadowsocks proxy.
    pub method: String,
}

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
        Scraper::scrape_vmess(source)
    }
}

impl Shadowsocks {
    pub fn to_url(&self) -> String {
        let base64_part = URL_SAFE.encode(format!("{}:{}", self.method, self.password));
        format!("ss://{}@{}:{}", base64_part, self.host, self.port)
    }

    pub fn scrape(source: &str) -> Vec<Self> {
        Scraper::scrape_shadowsocks(source)
    }
}

/// A scraper for extracting MTProxy information from a given source string.
pub struct Scraper();

impl Scraper {
    fn seperate_links(text: &str) -> String {
        let regex = Regex::new(r#"\b(https|ss|vmess)?://[^\s<>"']+[^.,;!?)"'\s]"#).unwrap();
        let mut links = String::new();
        for cap in regex.captures_iter(text) {
            links.push_str(&cap[0].replace("&amp;amp;", "&").replace("%3D", "="));
            links.push('\n');
        }
        links
    }
    /// Scrape MTProxy information from the provided source string.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing MTProxy information to be scraped.
    ///
    /// # Returns
    ///
    /// A vector of `MTProxy` instances parsed from the source string.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = "Hello https://t.me/proxy?server=proxy.example.com&port=8080&secret=mysecret";
    /// let proxies = proxy_scraper::Scraper::scrape_mtproxy(source);
    /// println!("{:?}", proxies);
    /// ```
    pub fn scrape_mtproxy(source: &str) -> Vec<MTProxy> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<MTProxy> = Vec::new();
        let regex = Regex::new(
        r#"(\w+:\/\/.*\/proxy\?((server=.+)|(port=.+)|(secret=([A-Fa-f0-9]+|[A-Za-z0-9+\/]+))))+"#,
    )
    .unwrap();

        for captures in regex.captures_iter(source) {
            let proxy = captures.get(1).map(|m| m.as_str()).unwrap_or("");

            if proxy.is_empty() {
                continue;
            }

            let proxy_url = Url::parse(proxy).unwrap();

            let server = proxy_url
                .query_pairs()
                .find(|(key, _)| key == "server")
                .map(|(_, value)| value);
            let port = proxy_url
                .query_pairs()
                .find(|(key, _)| key == "port")
                .map(|(_, value)| value);
            let secret = proxy_url
                .query_pairs()
                .find(|(key, _)| key == "secret")
                .map(|(_, value)| value);

            match (server, port, secret) {
                (Some(server), Some(port), Some(secret)) => proxy_list.push(MTProxy {
                    host: server.to_string(),
                    port: port.parse().unwrap_or(443),
                    secret: secret.to_string(),
                }),
                _ => continue,
            };
        }

        proxy_list
    }

    // TODO: Needs Error Handling
    pub fn scrape_shadowsocks(source: &str) -> Vec<Shadowsocks> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<Shadowsocks> = Vec::new();
        let regex = Regex::new(
            r#"ss://((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?)@((.+):(\d+))#"#,
        )
        .unwrap();

        for captures in regex.captures_iter(source) {
            let base64_part = captures.get(1).map(|b64| b64.as_str()).unwrap_or("");
            let host = captures.get(3).map(|h| h.as_str()).unwrap_or("");
            let port: u32 = captures
                .get(4)
                .map(|p| p.as_str())
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap();

            if base64_part.is_empty() || host.is_empty() || port == 0 {
                continue;
            }

            let decoded_base64_part =
                String::from_utf8(URL_SAFE.decode(&base64_part).unwrap()).unwrap();
            let parts: Vec<&str> = decoded_base64_part.split(":").collect();

            let method = parts[0].to_string();
            let password = parts[1].to_string();

            proxy_list.push(Shadowsocks {
                host: host.to_string(),
                port,
                password,
                method,
            });
        }
        proxy_list
    }

    pub fn scrape_vmess(source: &str) -> Vec<VMess> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<VMess> = Vec::new();
        let regex = Regex::new(
            r#"vmess://((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?)"#,
        )
        .unwrap();

        for captures in regex.captures_iter(source) {
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
