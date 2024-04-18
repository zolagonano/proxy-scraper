#[cfg(feature = "scraper")]
extern crate lib;

use regex::Regex;
use url::Url;

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

/// A scraper for extracting MTProxy information from a given source string.
pub struct Scraper();

impl Scraper {
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
    /// let proxies = lib::Scraper::scrape_mtproxy(source);
    /// println!("{:?}", proxies);
    /// ```
    pub fn scrape_mtproxy(source: &str) -> Vec<MTProxy> {
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
}
