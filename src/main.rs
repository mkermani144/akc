use std::env;

mod friend;
mod suggest;

fn print_help() {
    println!("Usage: akc <command> <args>");
    println!("Available commands are: help, friend, suggest");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return
    }

    let command = &args[1];
    let child_commands = &args[2..];

    match command.as_ref() {
        "" | "help" => print_help(),
        "friend" => friend::handle(child_commands),
        "suggest" => suggest::handle(child_commands),
        _ => println!("Invalid command: {}", command)
    }
}
