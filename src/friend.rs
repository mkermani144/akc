use structopt::StructOpt;

use crate::config;

#[derive(StructOpt)]
pub struct FriendCommandBase {
    name: String
}

#[derive(StructOpt)]
#[structopt(about = "Add a friend of specific type")]
pub enum FriendCommand {
    Aji(FriendCommandBase),
    Ki(FriendCommandBase),
    Chi(FriendCommandBase),
}

pub fn handle(args: FriendCommand) {
    match args {
        FriendCommand::Aji(name_wrapper) => config::add_aji(name_wrapper.name),
        FriendCommand::Ki(name_wrapper) => config::add_ki(name_wrapper.name),
        FriendCommand::Chi(name_wrapper) => config::add_chi(name_wrapper.name),
    }
}