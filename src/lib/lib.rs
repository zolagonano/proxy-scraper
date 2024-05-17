#[cfg(feature = "scraper")]
pub mod hysteria;
pub mod mtproxy;
pub mod shadowsocks;
pub mod trojan;
pub mod tuic;
mod utils;
pub mod vless;
pub mod vmess;

use std::collections::HashSet;

pub trait Proxy {
    fn to_url(&self) -> String;
    fn to_url_pretty(&self) -> String {
        if self.get_type() == "MTPROXY" || self.get_type() == "VMESS" {
            return self.to_url();
        };

        format!(
            "{}#{}-{}-{}%20{}:{}",
            self.to_url(),
            self.get_type(),
            self.get_network(),
            self.get_security(),
            self.get_host(),
            self.get_port(),
        )
    }

    fn scrape(source: &str) -> Vec<impl Proxy>;
    fn scrape_pretty(source: &str) -> HashSet<String> {
        Self::scrape(source)
            .into_iter()
            .map(|proxy| proxy.to_url_pretty())
            .collect()
    }
    fn get_host(&self) -> &str;
    fn get_port(&self) -> u32;

    fn get_network(&self) -> String;
    fn get_security(&self) -> String;

    fn get_type(&self) -> &str;

    #[cfg(feature = "checking")]
    fn url_test(&self) -> Result<u128, String> {
        let url = format!("https://{}:{}", self.get_host(), self.get_port());

        let start = std::time::Instant::now();

        match reqwest::blocking::get(url) {
            Ok(_) => {
                let elapsed = start.elapsed().as_millis();
                Ok(elapsed)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    #[cfg(feature = "checking")]
    fn port_check(&self) -> bool {
        use std::net::TcpStream;
        // TODO: Change ports to u16 later.
        match TcpStream::connect((&self.get_host()[..], self.get_port() as u16)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
