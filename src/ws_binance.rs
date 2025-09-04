use tokio_tungstenite::connect_async;
use futures::{StreamExt};
use serde_json::Value;
use anyhow::Result;

pub async fn binance_ws_price(symbol: &str) -> Result<f64> {
    let url = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol.to_lowercase());
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to Binance WebSocket for {}", symbol);

    let mut ws_stream = ws_stream;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let json: Value = serde_json::from_str(msg.to_text()?)?;
            let price = json["p"].as_str().unwrap().parse::<f64>()?;
            return Ok(price);
        }
    }

    Ok(0.0)
}

