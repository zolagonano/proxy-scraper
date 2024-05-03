use argh::FromArgs;
use proxy_scraper::*;
use std::str::FromStr;

enum ProxyType {
    MTProxy,
    Shadowsocks,
    VMess,
    VLess,
    Trojan,
    Hysteria,
    TUIC,
}

impl FromStr for ProxyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mtproxy" => Ok(Self::MTProxy),
            "ss" | "shadowsocks" => Ok(Self::Shadowsocks),
            "vmess" => Ok(Self::VMess),
            "vless" => Ok(Self::VLess),
            "trojan" => Ok(Self::Trojan),
            "hysteria" => Ok(Self::Hysteria),
            "tuic" => Ok(Self::TUIC),
            _ => Err(()),
        }
    }
}

macro_rules! scrape_proxy {
    ($module:ident::$struct:ident, $context:expr) => {{
        let context = fetch_url(&$context).await?;
        let result = proxy_scraper::$module::$struct::scrape(&context)
            .into_iter()
            .map(|proxy| proxy.to_url())
            .collect::<Vec<String>>();

        println!("{}", result.join("\n"));
    }};
}

#[derive(Debug, FromArgs)]
/// Scrap Proxies from URLs
struct ProxyScraper {
    #[argh(option)]
    #[argh(description = "proxies source url")]
    source: String,

    #[argh(option, default = "String::from(\"mtproxy\")")]
    #[argh(description = "proxy type")]
    proxy_type: String,
}

async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let body = response.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: ProxyScraper = argh::from_env();

    match ProxyType::from_str(&cli.proxy_type) {
        Ok(ProxyType::MTProxy) => scrape_proxy!(mtproxy::MTProxy, cli.source),

        Ok(ProxyType::Shadowsocks) => scrape_proxy!(shadowsocks::Shadowsocks, cli.source),
        Ok(ProxyType::VMess) => scrape_proxy!(vmess::VMess, cli.source),
        Ok(ProxyType::VLess) => scrape_proxy!(vless::VLess, cli.source),
        Ok(ProxyType::Trojan) => scrape_proxy!(trojan::Trojan, cli.source),
        Ok(ProxyType::Hysteria) => scrape_proxy!(hysteria::Hysteria, cli.source),
        Ok(ProxyType::TUIC) => scrape_proxy!(tuic::TUIC, cli.source),
        Err(_) => eprintln!("Error: Invalid Proxy Type"),
    }

    Ok(())
}
