mod raw_tx;
mod signed_tx;
mod s_tx;
mod tpae;

use data_encoding::HEXLOWER_PERMISSIVE;
use ethereum_tx_sign::Transaction;
use s_tx::generate;
use rand::Rng;


use crate::raw_tx::Transactions;
// use crate::signed_tx::TransactionChainId;
use crate::s_tx::{LegacyTransaction};

fn main() {
    tpae::to_hex();
    
    // let unsigned_transaction = Transactions {
    //     nonce: "0x1e7",
    //     gas_price: "0x2e90edd000",
    //     gas_limit: "0x30d40",
    //     to: "0xbd064928cdd4fd67fb99917c880e6560978d7ca1",
    //     value: "0xde0b6b3a7640000",
    //     data: "0x",
    // };

    // let x = unsigned_transaction.encode_to_hex();

    // println!("\nUnsigned tx RLP encoded: {:?}", hex::encode(x));

    let receipt_address = LegacyTransaction::address_to_bytes("0x5852231D8a00306A67DfB128AEd50c1573411d60").unwrap();
    let pk = private_key_to_bytes("124ce2df311216d9c6f8c417ce2258ef45df6c6e2cb12b40762d1debc8a170e4");
    
    let private_key_str = "0x1234567890abcdef";
let private_key = split_private_key(private_key_str).unwrap();
    println!("private key: {:?}", &private_key);
    
    let fuji_value = 0.2;  // Value in Fuji
    let decimal_factor = 10u128.pow(9);  // Decimal factor for conversion to nanofuji
    let nanofuji_value = attach_fuji_value(fuji_value, decimal_factor);

    let tx = LegacyTransaction {
        nonce: 97,
        chain: 43113,
        to: Some(receipt_address),
        value: nanofuji_value as u128,
        gas_price: 20 * (10u128).pow(9),
        gas_limit: 21000,
        data: vec![0x1, 0x2],
    };
    

    let ecdsa = tx.ecdsa(&pk).unwrap();
    let tx_bytes = tx.sign(&ecdsa);
    println!("this is tx byte: {:?}", tx_bytes);

    //confirm to hexa
    let hex = hex::encode(&tx_bytes);
    println!("hex value: {:?}", hex);

    fn generate_private_key() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let mut private_key: [u8; 32] = [0; 32];
        rng.fill(&mut private_key);
        private_key
    }

    fn attach_fuji_value(value: f64, decimal_factor: u128) -> u64 {
        let fuji_value = value * decimal_factor as f64;
        let nanofuji_value = fuji_value.round() as u64;
    
        nanofuji_value
    }

    fn private_key_to_bytes(private_key: &str) -> [u8; 32] {
        // Remove the "0x" prefix if it exists
        let key = if private_key.starts_with("0x") {
            &private_key[2..]
        } else {
            private_key
        };
    
        // Ensure the key has a valid length
        if key.len() != 64 {
            return [0;32];
        }
    
        // Parse the hexadecimal key into a byte array
        let bytes = match hex::decode(key) {
            Ok(decoded) => decoded,
            Err(_) => return [0;32],
        };
    
        // Ensure the byte array has the expected length
        if bytes.len() != 32 {
            println!("Error");
            return [0;32];
        }
    
        // Convert the byte vector to a fixed-size array
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes);
    
        result
    }
    
    fn split_private_key(private_key: &str) -> Result<[u8; 32], &'static str> {
        // Remove the "0x" prefix if present
        let key = if private_key.starts_with("0x") {
            &private_key[2..]
        } else {
            private_key
        };
    
        // Parse the private key as a hexadecimal string
        let num = match u128::from_str_radix(key, 16) {
            Ok(parsed) => parsed,
            Err(_) => return Err("Failed to parse private key"),
        };
    
        // Convert the number into a byte array
        let mut bytes = [0u8; 32];
        let num_bytes = num.to_be_bytes();
        let offset = bytes.len().saturating_sub(num_bytes.len());
        bytes[offset..].copy_from_slice(&num_bytes);
    
        Ok(bytes)
    }
    // match generate(x, &generate_private_key(), Some(1)) {
    //     Ok(result) => {
    //         println!("Sign Tx: {:?}", &result);
            
       
    //     // let hex_string = hex::encode(result);
    //     // println!("Hex representation: {}", hex_string);


    //     }
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //     }
    // }


    // let signed_tx = TransactionChainId {
    //     nonce: "0x1e7",
    //     gas_price: "0x2e90edd000",
    //     gas_limit: "0x30d40",
    //     to: "0xbd064928cdd4fd67fb99917c880e6560978d7ca1",
    //     value: "0xde0b6b3a764000",
    //     data: "0x",
    //     chain_id: 1,
    // };

    // ethereum_tx_sign::Transaction::chain(&signed_tx);

    // println!("\nSigned Tx RLP encoded: {}", signed_tx.encode_to_hex());
}
