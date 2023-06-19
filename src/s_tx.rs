// #![deny(warnings)]
#![deny(clippy::all)]
extern crate serde;
extern crate rlp;
extern crate secp256k1;


use rlp::{ Encodable, RlpStream };
use tiny_keccak::{Hasher, Keccak};
// use serde::de::Error as SerdeErr;
// use serde::ser::SerializeSeq;
use serde::{Serialize, Deserialize};

// use serde_derive::Deserialize;
// use serde_derive::Serialize;

use secp256k1::{ Message, Secp256k1, SecretKey };
// use rand::rngs::OsRng;
// use secp256k1::{ Secp256k1, Message };
// use ethers::prelude::*;

use ethereum_tx_sign::{ Transaction, EcdsaSig };
// use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    Secp256k1(secp256k1::Error),
}

impl From<secp256k1::Error> for Error {
    fn from(error: secp256k1::Error) -> Self {
        Error::Secp256k1(error)
    }
}

/// Internal function that avoids duplicating a lot of signing code
pub fn sign_bytes<T: Transaction>(tx_type: Option<u8>, ecdsa: &EcdsaSig, t: &T) -> Vec<u8> {
    let mut rlp_stream = RlpStream::new();
    let rlp = t.rlp_parts();
    rlp_stream.begin_unbounded_list();
    for r in rlp.iter() {
        rlp_stream.append(r);
    }
    let EcdsaSig { v, s, r } = ecdsa;

    // removes leading zeroes
    let mut r_n = r.clone();
    let mut s_n = s.clone();
    while r_n[0] == 0 {
        r_n.remove(0);
    }
    while s_n[0] == 0 {
        s_n.remove(0);
    }

    rlp_stream.append(v);
    rlp_stream.append(&r_n);
    rlp_stream.append(&s_n);

    rlp_stream.finalize_unbounded_list();

    let mut vec = rlp_stream.out().to_vec();
    if let Some(b) = tx_type {
        vec.insert(0usize, b);
    }
    vec
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LegacyTransaction {
    /// Chain ID
    pub chain: u64,
    /// Nonce
    pub nonce: u128,
    /// Recipient (None when contract creation)
    // #[serde(serialize_with = "option_array_u8_serialize")]
    // #[serde(deserialize_with = "option_array_u8_deserialize")]
    #[serde(default)]
    pub to: Option<[u8; 20]>,
    /// Transfered value
    pub value: u128,
    /// Gas price
    #[serde(rename = "gasPrice")]
    pub gas_price: u128,
    /// Gas limit
    #[serde(alias = "gasLimit")]
    pub gas_limit: u128,
    /// Input data
    // #[serde(serialize_with = "slice_u8_serialize")]
    // #[serde(deserialize_with = "slice_u8_deserialize")]
    #[serde(default)]
    pub data: Vec<u8>,
}

// impl LegacyTransaction {
//     fn ecdsa(&self, private_key: &[u8]) -> Result<EcdsaSig, Error> {
//         let hash = self.hash();

//         let chain = match Self::transaction_type() {
//             Some(_) => None,
//             None => Some(self.chain()),
//         };

//         EcdsaSig::generate(hash, private_key, chain)
//     }

// }

impl Transaction for LegacyTransaction {
    fn chain(&self) -> u64 {
        self.chain
    }

    fn rlp_parts(&self) -> Vec<Box<dyn Encodable>> {
        let to: Vec<u8> = match self.to {
            Some(ref to) => to.to_vec(),
            None => vec![],
        };
        vec![
            Box::new(self.nonce),
            Box::new(self.gas_price),
            Box::new(self.gas_limit),
            Box::new(to),
            Box::new(self.value),
            Box::new(self.data.clone())
        ]
    }

    fn sign(&self, ecdsa: &EcdsaSig) -> Vec<u8> {
        sign_bytes(None, ecdsa, self)
    }

    fn transaction_type() -> Option<u8> {
        None
    }

    fn ecdsa(&self, private_key: &[u8]) -> Result<EcdsaSig, ethereum_tx_sign::Error> {
        let hash = self.hash();

        let chain = match Self::transaction_type() {
            Some(_) => None,
            None => Some(self.chain()),
        };

        generate(hash, private_key, chain)
    }

    fn hash(&self) -> [u8; 32] {
        let rlp = self.rlp_parts();
        let mut rlp_stream = RlpStream::new();
        rlp_stream.begin_unbounded_list();
        for r in rlp.iter() {
            rlp_stream.append(r);
        }

        // `None` means it is legacy
        if Self::transaction_type().is_none() {
            rlp_stream.append(&self.chain());
            rlp_stream.append_raw(&[0x80], 1);
            rlp_stream.append_raw(&[0x80], 1);
        }

        rlp_stream.finalize_unbounded_list();
        let mut rlp_bytes = rlp_stream.out().to_vec();

        if let Some(tt) = Self::transaction_type() {
            rlp_bytes.insert(0usize, tt);
        }

        keccak256_hash(&rlp_bytes)
    }
}

impl LegacyTransaction {
    pub fn address_to_bytes(address: &str) -> Result<[u8; 20], hex::FromHexError> {
        let address = address.trim_start_matches("0x");
        let bytes = hex::decode(address)?;
        let mut result = [0u8; 20];
        result.copy_from_slice(&bytes[..20]);
        Ok(result)
    }
}


fn keccak256_hash(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    let mut resp: [u8; 32] = Default::default();
    hasher.finalize(&mut resp);
    resp
}

pub fn generate(
    hash: [u8; 32],
    private_key: &[u8],
    chain_id: Option<u64>
) -> Result<EcdsaSig, ethereum_tx_sign::Error> {
    let s = Secp256k1::signing_only();
    let msg = Message::from_slice(&hash)?;
    let key = SecretKey::from_slice(private_key)?;
    let (v, sig_bytes) = s.sign_ecdsa_recoverable(&msg, &key).serialize_compact();

    let v =
        (v.to_i32() as u64) +
        (match chain_id {
            Some(c) => c * 2 + 35,
            None => 0,
        });

    let ecdsa_sig = EcdsaSig {
        v,
        r: sig_bytes[0..32].to_vec(),
        s: sig_bytes[32..64].to_vec(),
    };

    // let tx = LegacyTransaction {
    //   chain: 1,
    //   nonce: 0,
    //   to: Some([0x45; 20]),
    //   value: 1000,
    //    gas_price: 20 * 10u128.pow(9),
    //    gas_limit: 21000,
    //   data: vec![]
    //  };



    // let t = sign_bytes(None, &ecdsa_sig, &tx);
    // let h = hex::encode(&t);
    // println!("hex of sign tx: {:?}", &h);
    //  println!("Raw sign tx: {:?}", &t);

    Ok(EcdsaSig {
        v,
        r: sig_bytes[0..32].to_vec(),
        s: sig_bytes[32..64].to_vec(),
    })

    
}

// fn slice_u8_deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: String = String::deserialize(deserializer)?;
//     let s = if s.starts_with(HEX_PREFIX) {
//         s.replace(HEX_PREFIX, "")
//     } else {
//         s
//     };
//     match hex::decode(&s) {
//         Ok(s) => Ok(s),
//         Err(err) => Err(derr::<D>(&s, err)),
//     }
// }

// fn slice_u8_serialize<S>(slice: &[u8], s: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     s.serialize_str(&hex::encode(slice))
// }

// fn derr<'de, D: serde::Deserializer<'de>>(s: &str, err: hex::FromHexError) -> D::Error {
//     match err {
//         hex::FromHexError::InvalidHexCharacter { c, .. } => {
//             D::Error::invalid_value(serde::de::Unexpected::Char(c), &"a valid hex character")
//         }
//         hex::FromHexError::OddLength => {
//             D::Error::invalid_length(s.len(), &"a hex string of even length")
//         }
//         hex::FromHexError::InvalidStringLength => {
//             D::Error::invalid_length(s.len(), &"a hex string that matches container length")
//         }
//     }
// }
