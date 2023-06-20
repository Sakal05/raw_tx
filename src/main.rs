mod s_tx;
// mod fetch_gas_price;

use std::str;

// use data_encoding::HEXLOWER_PERMISSIVE;
use ethereum_tx_sign::Transaction;
use serde::Deserialize;
// use s_tx::generate;
// use rand::Rng;
use web3_rpc::web3::Web3;
// use crate::raw_tx::Transactions;
use crate::s_tx::{ LegacyTransaction };

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let pk = private_key_to_bytes(
        "124ce2df311216d9c6f8c417ce2258ef45df6c6e2cb12b40762d1debc8a170e4"
    );

    let tx = new_struct().await?;

    println!("Tx: {:#?}", &tx);

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
    Ok(())
}

async fn new_struct() -> anyhow::Result<LegacyTransaction> {
    let receipt_address = LegacyTransaction::address_to_bytes(
        "0x5852231D8a00306A67DfB128AEd50c1573411d60"
    ).unwrap();

    let fuji_value = 0.2; // Value in Fuji
    let decimal_factor = (10u128).pow(9); // Decimal factor for conversion to nanofuji
    let nanofuji_value = attach_fuji_value(fuji_value, decimal_factor);
    let gas_price = get_gas_price().await?;
    let current_nonce = get_nonce().await?;
 
    // println!("Current nonce: {:?}", current_nonce);
    // println!("Gas Price: {:?}", &gas_price);


    let mut tx = LegacyTransaction {
        nonce: current_nonce,
        chain: 43113,
        to: Some(receipt_address),
        value: nanofuji_value as u128,
        gas_price: gas_price,
        gas_limit: 0,   //will be updated later after gas estimatation
        data: vec![0x1, 0x2],
    };

    /* Estimate Gas Price to set gas limit */

    let receipt_address_string = format!("0x{}", hex::encode_upper(receipt_address));

    let encoded_value = format!("{:x?}", &tx.value);
    let gas_price_hex = format!("{:x?}", &tx.gas_price);
    let gas_limit_hex = format!("{:x?}", &tx.gas_limit);

    #[derive(Deserialize)]
    struct ApiResponse {
        jsonrpc: String,
        id: u32,
        // result: String,
        error: Option<ApiError>,
        result: String,
    }

    #[derive(Deserialize)]
    #[derive(Debug)]
    struct ApiError {
        code: i32,
        message: String,
    }
    //convert data into hexadecimal
    let data_h = hex::encode(&tx.data);

    let url = format!(
        "https://api-testnet.snowtrace.io/api?module=proxy&action=eth_estimateGas&data=0x{}&to={}&value=0x{}&gasPrice=0x{}&gas=0x{}&apikey=U5U1Q3YXX6BMNJ5DJVDK4EBYRUVZS3HBZI",
        data_h,
        receipt_address_string,
        encoded_value,
        gas_price_hex,
        gas_limit_hex
    );

    let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;

    match response.error {
        Some(error) => {
            println!("Error code: {}", error.code);
            println!("Error message: {:#?}", error.message);
        }
        None => {
            
        }
    }

    let temp = &response.result.trim_start_matches("0x");
    let gas_price_int = u128::from_str_radix(temp, 16)?;
    tx.gas_limit = gas_price_int;

    Ok(tx)

}

// async fn test_request() -> anyhow::Result<u128> {
//     #[derive(Deserialize)]
//     struct ApiResponse {
//         _jsonrpc: String,
//         _id: u32,
//         // result: String,
//         error: Option<ApiError>,
//         result: String,
//     }

//     #[derive(Deserialize)]
//     #[derive(Debug)]
//     struct ApiError {
//         code: i32,
//         message: String,
//     }

//     let tx = new_struct().await?;
//     let receipt_address = match &tx.to {
//         Some(address) => address,
//         None => {
//             return Err(anyhow::anyhow!("No recipient address provided"));
//         }
//     };

//     //convert to address to hex format with 0x upfront
//     let receipt_address_string = format!("0x{}", hex::encode_upper(receipt_address));

//     let encoded_value = format!("{:x?}", tx.value);
//     let gas_price_hex = format!("{:x?}", tx.gas_price);
//     let gas_limit_hex = format!("{:x?}", tx.gas_limit);

//     //convert data into hexadecimal
//     let d = tx.data.clone();
//     let data_h = hex::encode(&d);

//     let url = format!(
//         "https://api-testnet.snowtrace.io/api?module=proxy&action=eth_estimateGas&data=0x{}&to={}&value=0x{}&gasPrice=0x{}&gas=0x{}&apikey=U5U1Q3YXX6BMNJ5DJVDK4EBYRUVZS3HBZI",
//         data_h,
//         receipt_address_string,
//         encoded_value,
//         gas_price_hex,
//         gas_limit_hex
//     );

//     let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;

//     match response.error {
//         Some(error) => {
//             println!("Error code: {}", error.code);
//             println!("Error message: {:#?}", error.message);
//         }
//         None => {
//             println!("Success");
//         }
//     }

//     let temp = &response.result.trim_start_matches("0x");
//     let gas_price_int = u128::from_str_radix(temp, 16)?;

//     Ok(gas_price_int)
// }

fn attach_fuji_value(value: f64, decimal_factor: u128) -> u64 {
    let fuji_value = value * (decimal_factor as f64);
    let nanofuji_value = fuji_value.round() as u64;
    // println!("nano value: {}", nanofuji_value);

    nanofuji_value
}

async fn get_gas_price() -> anyhow::Result<u128> {
    let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());
    let r = rpc.eth_gas_price().await?;

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
