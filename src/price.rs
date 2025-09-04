use reqwest::Client;
use serde_json::Value;

pub async fn fetch_binance_price(symbol: &str) -> anyhow::Result<f64> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let client = Client::new();
    let resp: Value = client.get(&url).send().await?.json().await?;
    Ok(resp["price"].as_str().unwrap().parse()?)
}

