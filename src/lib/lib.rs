use regex::Regex;
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

/// A scraper for extracting MTProxy information from a given source string.
pub struct Scraper();

impl Scraper {
    fn seperate_links(text: &str) -> String {
        let regex = Regex::new(r#"\b(https|ss)?://[^\s<>"']+[^.,;!?)"'\s]"#).unwrap();
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
            r#"ss://((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?)@((.+):\d+)#"#,
        )
        .unwrap();

        for captures in regex.captures_iter(source) {
            let base64_part = captures.get(1).unwrap().as_str();
            let host = captures.get(3).unwrap().as_str().to_string();
            let port: u32 = captures
                .get(4)
                .unwrap()
                .as_str()
                .to_string()
                .parse::<u32>()
                .unwrap();

            let decoded_base64_part =
                String::from_utf8(base64::decode(&base64_part).unwrap()).unwrap();
            let parts: Vec<&str> = decoded_base64_part.split(":").collect();

            let method = parts[0].to_string();
            let password = parts[1].to_string();

            proxy_list.push(Shadowsocks {
                host,
                port,
                password,
                method,
            });
        }
        proxy_list
    }
}
