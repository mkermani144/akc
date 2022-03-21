use structopt::{StructOpt};

mod friend;
mod config;

#[derive(StructOpt)]
#[structopt(about = "A command-line tool for managing connections with friends.")]
enum Akc {
    Friend(friend::FriendCommand)
}

fn main() {
    let args = Akc::from_args();

    match args {
        Akc::Friend(friend_args) => friend::handle(friend_args)
    }
}
