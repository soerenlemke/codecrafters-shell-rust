#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let input = read();
        eval(input);
    }
}

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn eval(input: String) {
    let command = input.trim();
    if command == "exit" {
        std::process::exit(0);
    }

    match command {
        "exit" => std::process::exit(0),
        &_ => println!("{}: command not found", input.trim()),
    }
}
