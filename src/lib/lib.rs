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
    fn scrape(source: &str) -> Vec<impl Proxy>;
    fn scrape_pretty(source: &str) -> HashSet<String> {
        Self::scrape(source)
            .into_iter()
            .map(|proxy| proxy.to_url())
            .collect()
    }
    fn get_host(&self) -> &str;
    fn get_port(&self) -> u32;

    #[cfg(feature = "scraper")]
    fn port_check(&self) -> bool {
        #[cfg(feature = "scraper")]    
        use std::net::TcpStream;
        // TODO: Change ports to u16 later.
        match TcpStream::connect((&self.get_host()[..], self.get_port() as u16)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
