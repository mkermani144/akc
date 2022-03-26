use structopt::{StructOpt};

mod config;
mod friend;
mod memory;
mod suggest;

#[derive(StructOpt)]
#[structopt(about = "A command-line tool for managing connections with friends.")]
enum Akc {
    Friend(friend::FriendCommand),
    Suggest(suggest::SuggestCommand),
    Memory(memory::MemoryCommand)
}

fn main() {
    let args = Akc::from_args();

    match args {
        Akc::Friend(friend_args) => friend::handle(friend_args),
        Akc::Suggest(_) => suggest::handle(),
        Akc::Memory(memory_args) => memory::handle(memory_args)
    }
}
