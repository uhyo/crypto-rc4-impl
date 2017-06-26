use generator::Generator;

pub fn rc4(key: &[u8], data: &[u8]) -> Vec<u8>{
    // generate a cipher stream from key
    let gen = Generator::new(key);
    // xor of cipher stream and data stream
    Iterator::zip(gen, data)
        .map(|(k, d)| k ^ d)
        .collect()
}
