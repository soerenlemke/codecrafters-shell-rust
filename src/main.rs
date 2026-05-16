use std::io::{self, Write};
use std::str::FromStr;

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
    let input = input.trim();
    let (command, args) = input.split_once(" ").unwrap_or((input, ""));

    match command {
        "exit" => std::process::exit(0),
        "echo" => echo_command(args),
        "type" => type_command(args),
        _ => println!("{}: command not found", input.trim()),
    }
}

fn echo_command(args: &str){
    println!("{}", args);
}

fn type_command(args: &str) {
    match BuiltinCommand::from_str(args) {
        Ok(_) => println!("{} is a shell builtin", args),
        Err(_) => println!("{}: not found", args)
    }
}

enum BuiltinCommand {
    Exit,
    Echo,
    Type,
}

impl FromStr for BuiltinCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(BuiltinCommand::Exit),
            "echo" => Ok(BuiltinCommand::Echo),
            "type" => Ok(BuiltinCommand::Type),
            _ => Err(()),
        }
    }
}
