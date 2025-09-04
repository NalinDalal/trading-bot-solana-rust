use solana_client::rpc_client::RpcClient;
use solana_sdk::{signer::keypair::Keypair, transaction::Transaction, signature::Signer};
use base64::{engine::general_purpose,Engine};
use bincode;

pub fn send_transaction(client: &RpcClient, tx_base64: &str, signer: &Keypair) -> anyhow::Result<()> {
    let tx_bytes = general_purpose::STANDARD.decode(tx_base64)?;
    let transaction: Transaction = bincode::deserialize(&tx_bytes)?;

    let recent_blockhash = client.get_latest_blockhash()?;
    client.send_and_confirm_transaction(&transaction)?;
    Ok(())
}

