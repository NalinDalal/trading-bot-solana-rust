use jupiter_swap_api_client::{quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig, JupiterSwapApiClient};
use solana_sdk::{signer::keypair::Keypair, pubkey::Pubkey};
use solana_client::rpc_client::RpcClient;

pub struct Jupiter {
    client: JupiterSwapApiClient,
}

impl Jupiter {
    pub fn new() -> Self {
        Self {
            client: JupiterSwapApiClient::new("https://quote-api.jup.ag/v6"),
        }
    }

    pub async fn get_quote(&self, input_mint: Pubkey, output_mint: Pubkey, amount: u64) -> anyhow::Result<QuoteRequest> {
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

    pub async fn execute_swap(&self, user: &Keypair, quote_response: QuoteRequest, rpc_client: &RpcClient) -> anyhow::Result<()> {
        let swap_response = self.client.swap(&SwapRequest {
            user_public_key: user.pubkey(),
            quote_response,
            config: TransactionConfig::default(),
        }).await?;

        // send tx to solana
        crate::utils::send_transaction(rpc_client, &swap_response.swap_transaction, user)?;
        Ok(())
    }
}

