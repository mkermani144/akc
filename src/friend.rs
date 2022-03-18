fn print_help() {
    println!("akc friend <command> <args>");
    println!("Available commands are: help, aji, ki, chi");
}

pub fn handle(args: &[String]) {
    if args.len() < 1 {
        print_help();
        return
    }

    let command = &args[0];

    match command.as_ref() {
        "" | "help" => print_help(),
        _ => println!("Invalid command: akc friend {}", command)
    }
}