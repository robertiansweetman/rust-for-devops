// hi.rs

use std::env;

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => panic!("No name given")
    }
}