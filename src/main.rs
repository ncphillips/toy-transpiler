extern crate regex;

use std::io::{self, Read};

mod transpiler;

fn main() {
    let mut code = String::new();
    io::stdin()
        .read_to_string(&mut code)
        .expect("No code provided");

    println!("{}", transpiler::transpile(&code));
}
