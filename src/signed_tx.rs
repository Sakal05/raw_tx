// use serde::de::Error as SerdeErr;

// use ethers::prelude::k256::SecretKey;
// use rand::rngs::OsRng;
// use rlp::{ Encodable, RlpStream };
// use secp256k1::{ Secp256k1, Message };
// use ethers::{ types::H256, signers::Signer };
// use ethers::prelude::*;
// use ethers::signers::LocalWallet;
// use ethereum_tx_sign::{ LegacyTransaction, Transaction, EcdsaSig };

// pub struct TransactionChainId<'a> {
//     pub nonce: &'a str,
//     pub gas_price: &'a str,
//     pub gas_limit: &'a str,
//     pub to: &'a str,
//     pub value: &'a str,
//     pub data: &'a str,
//     pub chain_id: u8,
// }

// impl Encodable for TransactionChainId<'_> {
//     fn rlp_append(&self, s: &mut RlpStream) {
//         s.begin_list(7)
//             .append(&self.nonce)
//             .append(&self.gas_price)
//             .append(&self.gas_limit)
//             .append(&self.to)
//             .append(&self.value)
//             .append(&self.data)
//             .append(&self.chain_id);
//     }
// }

// impl Transaction for TransactionChainId<'_> {
//     fn chain(&self) -> u64 {
//         println!("chain_id: {}", self.chain_id as u64);
//         self.chain_id as u64
//     }

    
//     fn sign(&self, ecdsa: &ethereum_tx_sign::EcdsaSig) -> Vec<u8> {
//         let sign_tx = ethereum_tx_sign::ecdsa(&self, private_key: &[u8]);
        
//     }

//     fn rlp_parts(&self) -> Vec<Box<dyn Encodable>> {
                
//         vec![
//             Box::new(self.nonce),
//             Box::new(self.gas_price),
//             Box::new(self.gas_limit),
//             Box::new(self.to),
//             Box::new(self.value),
//             Box::new(self.data.clone()),
//         ]
//     }

//     fn transaction_type() -> Option<u8> {
//         None
//     }
// }

// impl TransactionChainId<'_> {
//     pub fn encode_to_hex(&self) -> String {
//         let mut rng = OsRng;
//         let private_key = SecretKey::random(&mut rng);

//         let raw = rlp::encode(self);

//         // let hexa = hex::encode(&raw);

//         use sha3::{ Digest, Sha3_256 };

//         // create a SHA3-256 object
//         let mut hasher = Sha3_256::default();

//         // write input message
//         hasher.update(&raw);

//         let tx = LegacyTransaction {
//             chain: 1,
//             nonce: 0,
//             to: Some([0x45; 20]),
//             value: 1000,
//             gas_price: 20 * (10u128).pow(9),
//             gas: 21000,
//             data: vec![],
//         };

//         let ecdsa = tx.ecdsa(&vec![0x35; 32]).unwrap();
//         let tx_bytes = tx.sign(&ecdsa);
//         let tx_hash = hex::encode(&tx_bytes);

//         tx_hash

//         // println!("SIGN TX HASH: {:?}", &tx_hash);

//         // // read hash digest
//         // let hash = hasher.finalize();

//         // hex::encode(&hash)
//     }
// }
