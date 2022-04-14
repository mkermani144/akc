use clap::Parser;

mod config;
mod friend;
mod memory;
mod suggest;

#[derive(Parser)]
#[clap(about, version)]
enum AkcCommand {
    Friend(friend::Friend),
    Suggest(suggest::SuggestCommand),
    Memory(memory::Memory)
}

fn main() {
    let args = AkcCommand::parse();

    match args {
        AkcCommand::Friend(friend_args) => friend::handle(friend_args),
        AkcCommand::Suggest(_) => suggest::handle(),
        AkcCommand::Memory(memory_args) => memory::handle(memory_args)
    }
}
