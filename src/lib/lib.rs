#[cfg(feature = "scraper")]
pub mod hysteria;
pub mod mtproxy;
pub mod shadowsocks;
pub mod trojan;
pub mod tuic;
mod utils;
pub mod vless;
pub mod vmess;

pub trait Proxy {
    fn to_url(&self) -> String;
    fn scrape(source: &str) -> Vec<impl Proxy>;
}
