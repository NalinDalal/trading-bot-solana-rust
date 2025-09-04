use reqwest::Client;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex::encode;
use chrono::Utc;

#[derive(Clone)]
pub struct Binance {
    api_key: String,
    secret_key: String,
    client: Client,
}

impl Binance {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
            client: Client::new(),
        }
    }

    pub async fn place_order(&self, symbol: &str, side: &str, quantity: f64, price: f64) -> anyhow::Result<()> {
        let timestamp = Utc::now().timestamp_millis();
        let query = format!(
            "symbol={}&side={}&type=LIMIT&timeInForce=GTC&quantity={}&price={}&timestamp={}",
            symbol, side, quantity, price, timestamp
        );

        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())?;
        mac.update(query.as_bytes());
        let signature = encode(mac.finalize().into_bytes());

        let url = format!("https://testnet.binance.vision/api/v3/order?{}&signature={}", query, signature);

        let resp = self.client.post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        println!("Binance response: {:?}", resp.text().await?);
        Ok(())
    }
}

