#[cfg(feature = "scraper")]
pub mod hysteria;
pub mod mtproxy;
pub mod shadowsocks;
pub mod trojan;
pub mod tuic;
mod utils;
pub mod vless;
pub mod vmess;

use ping::ping;
use std::net::TcpStream;

pub trait Proxy {
    fn to_url(&self) -> String;
    fn scrape(source: &str) -> Vec<impl Proxy>;
    fn get_host(&self) -> &str;
    fn get_port(&self) -> u32;
    fn port_check(&self) -> bool {
        // TODO: Change ports to u16 later.
        match TcpStream::connect((&self.get_host()[..], self.get_port() as u16)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
