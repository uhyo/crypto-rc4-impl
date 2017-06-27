# RC4 implementation in Rust

*Warning: this implementation is not intended for practical use.*

## How to build
This implementation is written in the Rust language.
So you need to have Rust installed.
If not, first install Rust from [the official site](https://www.rust-lang.org/).

Once you have Rust, the following command at the project repository
should build and run the implementation.

```sh
cargo run
```

## Usage
By just doing `cargo run`, it repeatedly generates RC4 cipher streams for different randonly-generated keys,
and outputs their distribution to standard output.
