use url::Url;
use regex::Regex;

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
        let source = crate::utils::seperate_links(source);
        let mut proxy_list: Vec<MTProxy> = Vec::new();
        let regex = Regex::new(
        r#"(\w+:\/\/.*\/proxy\?((server=.+)|(port=.+)|(secret=([A-Fa-f0-9]+|[A-Za-z0-9+\/]+))))+"#,
    )
    .unwrap();

        for captures in regex.captures_iter(&source) {
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
