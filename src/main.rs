#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let input = read();
        println!("{}: command not found", input.trim())
    }
}

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}


