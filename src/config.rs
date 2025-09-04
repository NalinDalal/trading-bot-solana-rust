use dotenv::dotenv;
use std::env;

pub struct Config {
    pub binance_api_key: String,
    pub binance_secret: String,
    pub solana_keypair_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            binance_api_key: env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY missing"),
            binance_secret: env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY missing"),
            solana_keypair_path: env::var("SOLANA_KEYPAIR_PATH").expect("SOLANA_KEYPAIR_PATH missing"),
        }
    }
}

