use num_bigint::BigUint;
use sha3::digest::typenum::private::PrivateLogarithm2;
// use serde_json::json;
// use web3_rpc::model::Tag;
use web3_rpc::web3::Web3;
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;
// #[derive(Debug, Serialize, Deserialize)]
// struct JsonRpcResult {
//     jsonrpc: String,
//     id: String,
//     result: Option<String>,
// }
#[tokio::main]
pub async fn get_gas_price() -> anyhow::Result<u128> {
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

// #[tokio::main]
// pub async fn main() -> anyhow::Result<()> {
//     let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());
//     let r = rpc.eth_gas_price().await?;

//     match &r.result {
//         Some(gas_price) => {
//             // Decode gas price from hexadecimal to bytes
//             let temp = &gas_price.trim_start_matches("0x");
//             let gas_price_int = i64::from_str_radix(temp, 16);
           

//             // Print the gas price as a string
//             println!("Gas Price: {:?}", &gas_price_int);
//         }
//         None => {
//             println!("Gas price is not available in the JSON result.");
//         }
//     }

//     println!("R is: {:?}", &r);
//     Ok(())
//     // println!("{:?}", r);
// }