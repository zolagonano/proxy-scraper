
async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let body = response.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let context = fetch_url("https://raw.githubusercontent.com/ALIILAPRO/MTProtoProxy/main/mtproto.txt").await?;
    let result = lib::Scraper::scrape_mtproxy(&context);

    println!("{:#?}", result);
    Ok(())
}
