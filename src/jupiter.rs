use jupiter_swap_api_client::{quote::{QuoteRequest, QuoteResponse}, swap::SwapRequest, transaction_config::TransactionConfig, JupiterSwapApiClient};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::{signer::keypair::Keypair, pubkey::Pubkey, signature::Signer};

use solana_client::rpc_client::RpcClient;


#[derive(Clone)]
pub struct Jupiter {
    client: JupiterSwapApiClient
}

impl Jupiter {
    pub fn new() -> Self {
        Self {
            client: JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".to_string()),
        }
    }

    pub async fn get_quote(&self, input_mint: Pubkey, output_mint: Pubkey, amount: u64) -> anyhow::Result<QuoteResponse> {
        let quote_request = QuoteRequest {
            amount,
            input_mint,
            output_mint,
            slippage_bps: 50,
            ..QuoteRequest::default()
        };

        let response = self.client.quote(&quote_request).await?;
        Ok(response)
    }

    pub async fn execute_swap(&self, user: &Keypair, quote_response: QuoteResponse, rpc_client: &RpcClient) -> anyhow::Result<()> {
        let swap_response = self.client.swap(&SwapRequest {
            user_public_key: user.pubkey(),
            quote_response,
            config: TransactionConfig::default(),
        }, None).await?;

        // send tx to solana
    let tx_base64 = general_purpose::STANDARD.encode(&swap_response.swap_transaction);
    crate::utils::send_transaction(rpc_client, &tx_base64, user)?;
        Ok(())
    }
}

