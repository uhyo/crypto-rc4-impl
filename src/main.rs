extern crate rand;
extern crate getopts;
extern crate rc4impl;

use std::env;
use std::io;
use std::io::{Read, Write};
use std::fs::File;

use getopts::Options;
use rand::thread_rng;

use rc4impl::{rc4, to_hex, one_dist};

fn main() {
    let args: Vec<String> = env::args().collect();
    // parse options
    let mut options = Options::new();

    // define options
    options.optopt("k", "key", "Key of RC4 encryption", "");

    let matches = options.parse(&args[1..]).unwrap();

    match matches.opt_str("key") {
        Some(key) => {
            encrypt_mode(key);
        }
        None => {
            distribution_mode();
        }
    }
}

fn encrypt_mode(key: String) {
    let key = key.into_bytes();
    let stream = io::stdin().bytes().map(|res| res.unwrap());

    let result = rc4(&key[..], stream);
    let hex = to_hex(&result);
    println!("{}", hex);
}

const ITER_COUNT: i32 = 16777216;

fn distribution_mode() {
    let mut stderr = io::stderr();
    writeln!(stderr, "Distribution mode").unwrap();

    let mut dist = vec![];
    let mut rng = thread_rng();
    for _ in 0..20 {
        dist.push(Box::new([0; 256]));
    }
    for _ in 0..ITER_COUNT {
        one_dist(&mut rng, &mut dist);
    }

    // write each distribution to file
    for i in 0..20 {
        let filename = format!("./data/dist-{}.csv", i);
        let mut file = File::create(filename).unwrap();
        print_dist(&mut file, &dist[i]);
    }

    // take the second byte
    // let ref second = dist[1];
    // print_dist(second);
}

fn print_dist<T>(target: &mut T, dist: &[i32; 256])
    where T: Write
{
    for i in 0..256 {
        writeln!(
            target,
            "{}, {}",
            i,
            dist[i]).unwrap();
    }
}
