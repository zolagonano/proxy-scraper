use argh::FromArgs;
use std::str::FromStr;

enum ProxyType {
    MTProxy,
}

impl FromStr for ProxyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mtproxy" => Ok(Self::MTProxy),
            _ => Err(()),
        }
    }
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
        Ok(ProxyType::MTProxy) => {
            let context = fetch_url(&cli.source).await?;
            let result = proxy_scraper::Scraper::scrape_mtproxy(&context);

            println!("{:#?}", result);
        }
        Err(_) => eprintln!("Error: Invalid Proxy Type"),
    }

    Ok(())
}
