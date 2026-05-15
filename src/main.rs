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
    let (command, args) = input.split_once(" ").unwrap_or(("", ""));

    match command {
        "exit" => std::process::exit(0),
        "echo" => echo_command(args),
        &_ => println!("{}: command not found", input.trim()),
    }
}

fn echo_command(args: &str){
    println!("{}", args);
}