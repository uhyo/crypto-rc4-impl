extern crate rc4impl;

use std::env;
use std::io;
use std::io::Read;

use rc4impl::{rc4, to_hex};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Key is not provided");
    }
    let key = args[1].clone().into_bytes();
    let stream = io::stdin().bytes().map(|res| res.unwrap());

    let result = rc4(&key[..], stream);
    let hex = to_hex(&result);
    println!("{}", hex);
}
