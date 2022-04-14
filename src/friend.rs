use clap::{Args, Parser, Subcommand};

use crate::config;

#[derive(Args)]
pub struct FriendCommandBase {
    name: String
}

#[derive(Subcommand)]
#[clap(about = "Add a friend of specific type")]
pub enum FriendCommand {
    Aji(FriendCommandBase),
    Ki(FriendCommandBase),
    Chi(FriendCommandBase),
}

#[derive(Parser)]
pub struct Friend {
    #[clap(subcommand)]
    command: FriendCommand,
}

pub fn handle(args: Friend) {
    match args.command {
        FriendCommand::Aji(name_wrapper) => config::add_aji(name_wrapper.name),
        FriendCommand::Ki(name_wrapper) => config::add_ki(name_wrapper.name),
        FriendCommand::Chi(name_wrapper) => config::add_chi(name_wrapper.name),
    }
}