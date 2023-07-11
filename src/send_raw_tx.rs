use web3_rpc::web3::Web3;

pub async fn send_raw_tx(hash: String) -> anyhow::Result<String> {
  
    let rpc = Web3::new("https://ava-testnet.public.blastapi.io/ext/bc/C/rpc".to_string());

    /* Send Raw Transaction */
    let raw_tx_hash = format!("0x{}", hash);

    let send_raw = rpc.eth_send_raw_transaction(raw_tx_hash.as_str()).await?;

    let tx_hash = match send_raw.result {
        Some(res) => {
            println!("Result: {:?}", res);
            res
        }
        None => {
            println!("Send Failed");
            "0x".to_string()
        }
    };

    println!("Tx hash: {}", &tx_hash);

    Ok(tx_hash)
}
