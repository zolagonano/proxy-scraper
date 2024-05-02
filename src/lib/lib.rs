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

impl Shadowsocks {
    pub fn to_url(&self) -> String {
        let base64_part = URL_SAFE.encode(format!("{}:{}", self.method, self.password));
        format!("ss://{}@{}:{}", base64_part, self.host, self.port)
    }

    pub fn scrape(source: &str) -> Vec<Self> {
        Scraper::scrape_shadowsocks(source)
    }
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
        Scraper::scrape_vless(source)
    }
}

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
        Scraper::scrape_trojan(source)
    }
}

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
        Scraper::scrape_hysteria(source)
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

    pub fn scrape_vless(source: &str) -> Vec<VLess> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<VLess> = Vec::new();
        let regex = Regex::new(r#"vless://([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})@((.+):(\d+))\?(.+)#"#).unwrap();

        for captures in regex.captures_iter(source) {
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

    pub fn scrape_trojan(source: &str) -> Vec<Trojan> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<Trojan> = Vec::new();
        let regex = Regex::new(r#"trojan://([A-Za-z0-9\-._~]+)@((.+):(\d+))\?(.+)#"#).unwrap();

        for captures in regex.captures_iter(source) {
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

    pub fn scrape_hysteria(source: &str) -> Vec<Hysteria> {
        let source = &Self::seperate_links(source);
        let mut proxy_list: Vec<Hysteria> = Vec::new();
        let regex =
            Regex::new(r#"(hy2|hysteria2|hysteria)://([A-Za-z0-9\-._~]+)@((.+):(\d+))\?(.+)#"#)
                .unwrap();

        for captures in regex.captures_iter(source) {
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
