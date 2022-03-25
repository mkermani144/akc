use structopt::{StructOpt};

mod config;
mod friend;
mod suggest;

#[derive(StructOpt)]
#[structopt(about = "A command-line tool for managing connections with friends.")]
enum Akc {
    Friend(friend::FriendCommand),
    Suggest(suggest::SuggestCommand),
}

fn main() {
    let args = Akc::from_args();

    match args {
        Akc::Friend(friend_args) => friend::handle(friend_args),
        Akc::Suggest(_) => suggest::handle()
    }
}
