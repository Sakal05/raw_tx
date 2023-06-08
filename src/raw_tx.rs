use rlp::{ Encodable, RlpStream };

#[derive(Debug)]
pub struct Transaction<'a> {
    pub nonce: &'a str,
    pub gas_price: &'a str,
    pub gas_limit: &'a str,
    pub to: &'a str,
    pub value: &'a str,
    pub data: &'a str,
}

impl Transaction<'_> {
    pub fn encode_to_hex(&self) -> [u8; 32] {
        let encode = rlp::encode(self);

        let hexa = hex::encode(&encode);

        let mut result: [u8; 32] = [0; 32];
        result.copy_from_slice(&encode[..32]);
        result
    }
}

impl Encodable for Transaction<'_> {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(6)
            .append(&self.nonce)
            .append(&self.gas_price)
            .append(&self.gas_limit)
            .append(&self.to)
            .append(&self.value)
            .append(&self.data);
    }
}
