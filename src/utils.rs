use solana_client::rpc_client::RpcClient;
use solana_sdk::{signer::keypair::Keypair, transaction::Transaction, message::Message};
use base64::decode;
use bincode;

pub fn send_transaction(client: &RpcClient, tx_base64: &str, signer: &Keypair) -> anyhow::Result<()> {
    let tx_bytes = decode(tx_base64)?;
    let transaction: Transaction = bincode::deserialize(&tx_bytes)?;

    let recent_blockhash = client.get_recent_blockhash()?.0;
    let message = Message::new(&transaction.instructions, Some(&signer.pubkey()));
    let tx = Transaction::new_signed_with_payer(&message, Some(&signer.pubkey()), &[signer], recent_blockhash);

    client.send_and_confirm_transaction(&tx)?;
    Ok(())
}

