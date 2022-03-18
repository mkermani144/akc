fn print_help() {
  println!("akc suggest [<command>] <args>");
  println!("Available commands are: random, hangout, videocall, call, text");
}

pub fn handle(args: &[String]) {
    let command;

    if args.len() < 1 {
        command = "random"
    } else {
        command = &args[0];
    }

    match command.as_ref() {
        "" | "help" => print_help(),
        _ => println!("Invalid command: akc suggest {}", command)
    }
}