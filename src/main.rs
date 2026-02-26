use clap::Parser;

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
}

#[tokio::main]
async fn main() {
    let args = AkcCommand::parse();

    match args {
        AkcCommand::Friend(friend_args) => friend::handle(friend_args).await,
        AkcCommand::Suggest(_) => suggest::handle().await,
        AkcCommand::Memory(memory_args) => memory::handle(memory_args).await,
    }
}
