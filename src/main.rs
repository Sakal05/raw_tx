mod raw_tx;
mod signed_tx;
mod s_tx;
mod tpae;
// mod fetch_gas_price;

use std::str;

// use data_encoding::HEXLOWER_PERMISSIVE;
use ethereum_tx_sign::Transaction;
use serde::Deserialize;
use serde_json::json;
// use s_tx::generate;
// use rand::Rng;
use web3_rpc::web3::Web3;

// use crate::raw_tx::Transactions;
// use crate::signed_tx::TransactionChainId;
use crate::s_tx::{ LegacyTransaction };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let receipt_address = LegacyTransaction::address_to_bytes(
        "0x5852231D8a00306A67DfB128AEd50c1573411d60"
    ).unwrap();
    let pk = private_key_to_bytes(
        "124ce2df311216d9c6f8c417ce2258ef45df6c6e2cb12b40762d1debc8a170e4"
    );

    let fuji_value = 0.2; // Value in Fuji
    let decimal_factor = (10u128).pow(9); // Decimal factor for conversion to nanofuji
    let nanofuji_value = attach_fuji_value(fuji_value, decimal_factor);
    let gas_price = get_gas_price().await?;
    let current_nonce = get_nonce().await?;
    println!("Current nonce: {:?}", current_nonce);
    println!("Gas Price: {:?}", &gas_price);
    // let gas_est = estimate_gas().await?;

    //test request
    test_request().await;

    let tx = LegacyTransaction {
        nonce: current_nonce,
        chain: 43113,
        to: Some(receipt_address),
        value: nanofuji_value as u128,
        gas_price: gas_price,
        gas_limit: 15000000,
        data: vec![0x1, 0x2],
    };

    println!("Tx: {:?}", &tx);

    let ecdsa = tx.ecdsa(&pk).unwrap();
    let tx_bytes = tx.sign(&ecdsa);
    // println!("this is tx byte: {:?}", tx_bytes);

    //confirm to hexa
    let hex = hex::encode(&tx_bytes);
    println!("hex value: {:?}", hex);

    /* For testing purpose */
    // fn generate_private_key() -> [u8; 32] {
    //     let mut rng = rand::thread_rng();
    //     let mut private_key: [u8; 32] = [0; 32];
    //     rng.fill(&mut private_key);
    //     private_key
    // }

    // async fn get_current_gas_price() -> anyhow::Result<u128> {
    //     let gas_price = fetch_gas_price::get_gas_price()?;

    //     Ok(gas_price)
    // }
    async fn test_request() -> Result<u128, Box<dyn std::error::Error>> {
        
        #[derive(Deserialize)]
        struct ApiResponse {
            jsonrpc: String,
            id: u32,
            result: String,
        }

        let receipt_address = LegacyTransaction::address_to_bytes(
            "0x5852231D8a00306A67DfB128AEd50c1573411d60"
        ).unwrap();

        let receipt_address_string = format!("0x{}", hex::encode_upper(receipt_address));

        let url =
            format!("https://api-goerli.etherscan.io/api?module=proxy&action=eth_estimateGas&data=0x&to={}&value=0x1&gasPrice=0x51da038cc&gas=0x186A0&apikey=U5U1Q3YXX6BMNJ5DJVDK4EBYRUVZS3HBZI", receipt_address_string);

        // let resp = reqwest
        //     ::get("https://api-goerli.etherscan.io/api
        //         ?module=proxy
        //         &action=eth_estimateGas
        //         &data=0x
        //         &to=0xe84d601e5d945031129a83e5602be0cc7f182cf3
        //         &value=0x1
        //         &gasPrice=0x51da038cc
        //         &gas=0x186A0
        //         &apikey=U5U1Q3YXX6BMNJ5DJVDK4EBYRUVZS3HBZI").await?
        //     .json::<HashMap<String, String>>().await?;

        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        println!("jsonrpc: {}", response.jsonrpc);
        println!("id: {}", response.id);
        println!("result: {}", response.result);
        let temp = &response.result.trim_start_matches("0x");
        let gas_price_int = u128::from_str_radix(temp, 16)?;
        // let est_gas = hex::decode(&response.result[2..])?;
        // let gas_price_int = u128::from_str_radix(est_gas, 16)?;
        // let text = &response.text().await?;
        // println!("Response body: {}", text);

        Ok(gas_price_int)
    }

    fn attach_fuji_value(value: f64, decimal_factor: u128) -> u64 {
        let fuji_value = value * (decimal_factor as f64);
        let nanofuji_value = fuji_value.round() as u64;
        println!("nano value: {}", nanofuji_value);

        nanofuji_value
    }

    async fn get_gas_price() -> anyhow::Result<u128> {
        let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());
        let r = rpc.eth_gas_price().await?;

        match r.result {
            Some(gas_price) => {
                // Decode gas price from hexadecimal to bytes
                let temp = &gas_price.trim_start_matches("0x");
                println!("Temp: {}", temp);
                let gas_price_int = u128::from_str_radix(temp, 16)?;

                Ok(gas_price_int)
            }
            None => Err(anyhow::anyhow!("Gas price is not available in the JSON result.")),
        }
    }

    async fn get_nonce() -> anyhow::Result<u128> {
        let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());
        let r = rpc.eth_get_transaction_count(
            "0xCF6F0d155989B11Ba3882e99c72f609f0C06e086",
            None
        ).await?;

        match r.result {
            Some(nonce_now) => {
                let temp = &nonce_now.trim_start_matches("0x");
                let nonce = u128::from_str_radix(temp, 16)?;
                Ok(nonce)
            }
            None => Err(anyhow::anyhow!("Gas price is not available in the JSON result.")),
        }
    }

    async fn estimate_gas() -> anyhow::Result<u128> {
        let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());
        let receipt_address = LegacyTransaction::address_to_bytes(
            "0x5852231D8a00306A67DfB128AEd50c1573411d60"
        ).unwrap();
        let tx_json = LegacyTransaction {
            nonce: 12,
            chain: 43113,
            to: Some(receipt_address),
            value: 12,
            gas_price: 12,
            gas_limit: 12,
            data: vec![0x1, 0x2],
        };
        let tx_json = serde_json::to_value(&tx_json)?;
        println!("Tx json: {:?}", &tx_json);
        let r = rpc
            .eth_estimate_gas(json!(&tx_json)).await
            .map_err(|err| anyhow::Error::msg(err.to_string()))?;
        println!("json result: {:?}", &r);
        match r.result {
            Some(gas_price) => {
                // Decode gas price from hexadecimal to bytes
                let temp = &gas_price.trim_start_matches("0x");
                let gas_price_int = u128::from_str_radix(temp, 16)?;

                Ok(gas_price_int)
            }
            None => Err(anyhow::anyhow!("Gas price is not available in the JSON result.")),
        }
    }

    fn private_key_to_bytes(private_key: &str) -> [u8; 32] {
        // Remove the "0x" prefix if it exists
        let key = if private_key.starts_with("0x") { &private_key[2..] } else { private_key };

        // Ensure the key has a valid length
        if key.len() != 64 {
            return [0; 32];
        }

        // Parse the hexadecimal key into a byte array
        let bytes = match hex::decode(key) {
            Ok(decoded) => decoded,
            Err(_) => {
                return [0; 32];
            }
        };

        // Ensure the byte array has the expected length
        if bytes.len() != 32 {
            println!("Error");
            return [0; 32];
        }

        // Convert the byte vector to a fixed-size array
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes);

        result
    }

    Ok(())
}
