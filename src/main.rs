mod config;
mod jupiter;
mod binance;
mod utils;
mod price;
mod ws_binance;

use config::Config;
use jupiter::Jupiter;
use binance::Binance;
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use tracing_subscriber::fmt::init;
use ws_binance::binance_ws_price;
use tokio::sync::watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();
    let cfg = Config::from_env();

    // Solana Devnet RPC
    let rpc_client = Arc::new(RpcClient::new("https://api.devnet.solana.com"));

    // Load Solana wallet
    let user = Arc::new(read_keypair_file(&cfg.solana_keypair_path)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?);

    // Initialize clients
    let jupiter = Arc::new(Jupiter::new());
    let binance = Arc::new(Binance::new(cfg.binance_api_key.clone(), cfg.binance_secret.clone()));

    // Token Mints
    let sol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").expect("Invalid SOL mint");
    let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").expect("Invalid USDC mint");

    // Shared channel for Binance price updates
    let (tx, mut rx) = watch::channel(0.0f64);

    // Spawn WebSocket task
    tokio::spawn({
        let tx = tx.clone();
        async move {
            loop {
                match binance_ws_price("SOLUSDT").await {
                    Ok(price) => {
                        let _ = tx.send(price);
                    }
                    Err(e) => {
                        eprintln!("Binance WS error: {:?}", e);
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    });

    loop {
        // 1️⃣ Fetch Jupiter quote
        let quote = match jupiter.get_quote(sol_mint, usdc_mint, 1_000_000).await {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Failed to fetch Jupiter quote: {:?}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
    let j_price = quote.in_amount as f64 / 1e6;

        // 2️⃣ Get latest Binance price
        let b_price = *rx.borrow();

        let profit_percent = ((b_price - j_price) / j_price) * 100.0;
        println!("Jupiter: {:.4}, Binance: {:.4}, Profit: {:.2}%", j_price, b_price, profit_percent);

        if profit_percent > 0.5 {
            println!("Arbitrage opportunity detected!");

            // 3️⃣ Spawn async task for executing trades
            let jupiter_clone = Arc::clone(&jupiter);
            let binance_clone = Arc::clone(&binance);
            let user_clone = Arc::clone(&user);
            let rpc_client_clone = Arc::clone(&rpc_client);
            let quote_clone = quote.clone();

            tokio::spawn(async move {
                if let Err(e) = jupiter_clone.execute_swap(&user_clone, quote_clone, &rpc_client_clone).await {
                    eprintln!("Jupiter swap failed: {:?}", e);
                }

                if let Err(e) = binance_clone.place_order("SOLUSDT", "SELL", 0.01, b_price).await {
                    eprintln!("Binance order failed: {:?}", e);
                }
            });
        }

        sleep(Duration::from_millis(200)).await; // fast loop; trades run concurrently
    }
}

