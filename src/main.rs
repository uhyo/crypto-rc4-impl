extern crate rc4impl;

use rc4impl::Generator;

fn main() {
    let key = [0u8; 10];
    let foo = Generator::new(&key);

}
