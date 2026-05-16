use std::env::current_dir;
use std::io::{self, Write};
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;

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
        Err(_) => search_path_directories(args),
    }
}

fn search_path_directories(args: &str) {
    let current_directory = current_dir().unwrap_or_default();
    let parent_directory = current_directory.parent().unwrap_or(Path::new(""));

    for entry in WalkDir::new(parent_directory) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.file_name() == args {
            println!("{} is {}", args, entry.path().display());
            return;
        }
    }

    println!("{}: not found", args)
}

/*
When type receives a command input, your shell must follow these steps:
Check if the command is a builtin command (like exit or echo). If it is, report it as a builtin (<command> is a shell builtin) and stop.
If the command is not a builtin, your shell must go through every directory in PATH. For each directory:
    Check if a file with the command name exists.
    Check if the file has execute permissions.
    If the file exists and has execute permissions, print <command> is <full_path> and stop.
    If the file exists but lacks execute permissions, skip it and continue to the next directory.
If no executable is found in any directory, print <command>: not found.
*/

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
