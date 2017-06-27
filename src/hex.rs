pub fn to_hex(value: &[u8]) -> String{
    let mut result = String::new();
    for v in value{
        result.push_str(format!("{:>02x}", v).as_str());
    }
    result
}
