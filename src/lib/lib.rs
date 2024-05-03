pub mod mtproxy;
pub mod shadowsocks;
pub mod vmess;
pub mod vless;
pub mod trojan;
pub mod hysteria;
pub mod tuic;
pub mod utils;

#[cfg(feature = "scraper")]

pub trait Proxy {}
