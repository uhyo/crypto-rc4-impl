use generator::Generator;

pub fn rc4<'a, T>(key: &[u8], data: T) -> Vec<u8> where T: Iterator<Item=u8>{
    // generate a cipher stream from key
    let gen = Generator::new(key);
    // xor of cipher stream and data stream
    Iterator::zip(gen, data)
        .map(|(k, d)| k ^ d)
        .collect()
}
