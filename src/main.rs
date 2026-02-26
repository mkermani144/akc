use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use std::io;

mod config;
mod friend;
mod memory;
mod suggest;

#[derive(Parser)]
#[command(about, version)]
enum AkcCommand {
    Friend(friend::Friend),
    Suggest(suggest::SuggestCommand),
    Memory(memory::Memory),
    DbPath,
    Completion(CompletionCommand),
}

#[derive(Parser)]
struct CompletionCommand {
    shell: Shell,
}

#[tokio::main]
async fn main() {
    let args = AkcCommand::parse();

    match args {
        AkcCommand::Friend(friend_args) => friend::handle(friend_args).await,
        AkcCommand::Suggest(_) => suggest::handle().await,
        AkcCommand::Memory(memory_args) => memory::handle(memory_args).await,
        AkcCommand::DbPath => config::print_db_path(),
        AkcCommand::Completion(args) => {
            generate(args.shell, &mut AkcCommand::command(), "akc", &mut io::stdout())
        }
    }
}
