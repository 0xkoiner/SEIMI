use alloy::primitives::Address;

pub async fn vec_addr_to_string(v: &Vec<Address>) -> String {
    v.iter()
        .map(Address::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}
