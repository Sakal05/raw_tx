mod raw_tx;
mod signed_tx;
mod s_tx;

use s_tx::generate;
use rand::Rng;


use crate::raw_tx::Transaction;
// use crate::signed_tx::TransactionChainId;
use crate::s_tx::{LegacyTransaction};

fn main() {
    let unsigned_transaction = Transaction {
        nonce: "0x1e7",
        gas_price: "0x2e90edd000",
        gas_limit: "0x30d40",
        to: "0xbd064928cdd4fd67fb99917c880e6560978d7ca1",
        value: "0xde0b6b3a7640000",
        data: "0x",
    };

    let x = unsigned_transaction.encode_to_hex();

    // println!("\nUnsigned tx RLP encoded: {}", unsigned_transaction.encode_to_hex());

    let legacy_transaction = LegacyTransaction {
        nonce: 0,
        chain: 1,
        to: Some([0x45; 20]),
        value: 1000,
        gas_price: 20 * (10u128).pow(9),
        gas_limit: 21000,
        data: vec![],
    };

    fn generate_private_key() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let mut private_key: [u8; 32] = [0; 32];
        rng.fill(&mut private_key);
        private_key
    }
    

    generate(x, &generate_private_key(), Some(1));


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
