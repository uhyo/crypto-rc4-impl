mod generator;
mod rc4;

pub use generator::Generator;
pub use rc4::rc4;

#[cfg(test)]
mod test{
    use super::rc4;

    #[test]
    fn crypto(){
        let key = String::from("foobar");
        let plaintext = String::from("abcdefghijklmnopqrstuvwxyz");

        let key = key.into_bytes();
        let plaintext = plaintext.into_bytes();

        let message = rc4(&key[..], &plaintext[..]);

        let hex = to_hex(&message);
        assert_eq!(hex, "0ecb738a9325d40ce79c91344f83e11c3870d1caf4527b6a61fd");
    }

    fn to_hex(value: &[u8]) -> String{
        let mut result = String::new();
        for v in value{
            result.push_str(format!("{:>02x}", v).as_str());
        }
        result
    }
}
