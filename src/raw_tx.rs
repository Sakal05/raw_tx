use ethereum_tx_sign::{Transaction, EcdsaSig};
use rlp::{ Encodable, RlpStream };
use data_encoding::HEXLOWER_PERMISSIVE;

#[derive(Debug)]
pub struct Transactions<'a> {
    pub nonce: &'a str,
    pub gas_price: &'a str,
    pub gas_limit: &'a str,
    pub to: &'a str,
    pub value: &'a str,
    pub data: &'a str,
}

// impl Transaction<'_> {
//     pub fn encode_to_hex(&self) -> [u8; 32] {
//         let encode = rlp::encode(self);

//         let fields = [
//             &self.nonce,
//             &self.gas_price,
//             &self.gas_limit,
//             &self.to,
//             &self.value,
//             &self.data,
//         ];

        

//         // let hexa = hex::encode(&encode);

//         let mut result: [u8; 32] = [0; 32];
//         result.copy_from_slice(&encode[..32]);

//         // let hex = hex::encode(&result);
//         let hex = HEXLOWER_PERMISSIVE.encode(&encode);

//         println!("Hexa value: {}", &hex);

//         result
//     }
// }

impl Transactions<'_> {
    pub fn encode_to_hex(&self) -> [u8; 32] {
        let mut rlp_stream = RlpStream::new();

        rlp_stream.begin_list(6)
            .append(&self.nonce.strip_prefix("0x").unwrap())
            .append(&self.gas_price.strip_prefix("0x").unwrap())
            .append(&self.gas_limit.strip_prefix("0x").unwrap())
            .append(&self.to.strip_prefix("0x").unwrap())
            .append(&self.value.strip_prefix("0x").unwrap())
            .append(&self.data.strip_prefix("0x").unwrap());

    let encode = rlp_stream.out();
    println!("encoded rlp: {:?}", &encode);

    let mut result: [u8; 32] = [0; 32];
    result.copy_from_slice(&encode[0..32]);

        let hex = HEXLOWER_PERMISSIVE.encode(&result);
        println!("Hexa value: {}", hex);

    result
    }

    // fn sign(&self, ecdsa: &EcdsaSig) -> Vec<u8> {
    //     sign_bytes(None, ecdsa, self)
    // }

    
}

fn sign_bytes<T: Transaction>(tx_type: Option<u8>, ecdsa: &EcdsaSig, t: &T) -> Vec<u8> {
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
        vec.insert(0usize, b)
    }
    vec
}

impl Encodable for Transactions<'_> {
    fn rlp_append(&self, s: &mut RlpStream) {
        // s.begin_list(6)
        //     .append(&self.nonce)
        //     .append(&self.gas_price)
        //     .append(&self.gas_limit)
        //     .append(&self.to)
        //     .append(&self.value)
        //     .append(&self.data);
    }

    // fn rlp_bytes(&self) -> bytes::BytesMut {
	// 	    let mut s = RlpStream::new();
	// 	    self.rlp_append(&mut s);
	// 	    s.out()
	//     }
}
