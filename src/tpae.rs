use ethereum_tx_sign::{LegacyTransaction, Transaction};
use rand::Rng;

pub fn to_hex() {
    
    let new_transaction = LegacyTransaction {
        chain: 1,
        nonce: 0,
        to: Some([0; 20]),
        value: 1675538,
        gas_price: 250,
        gas: 21000,
        data: vec![],
    };

    let private_key_32_bytes = generate_private_key();
    
    let ecdsa = new_transaction.ecdsa(&private_key_32_bytes);
    let mut ecsdsa_temp;
    match &ecdsa {
        Ok(result) => {
            ecsdsa_temp = result.clone();
        },
        Err(e) => {
            println!("error hz");
        }
    }
    println!("ecdsa: {:?}", &ecdsa);
    // let transaction_bytes = new_transaction.sign(&ecsdsa_temp);
    
    
}

fn generate_private_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut private_key: [u8; 32] = [0; 32];
    rng.fill(&mut private_key);
    private_key
}