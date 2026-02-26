use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::config;

#[derive(Args)]
pub struct FriendCommandBase {
    name: String,
}

#[derive(ValueEnum, Clone)]
pub enum FriendLevel {
    Aji,
    Ki,
    Chi,
}

#[derive(Args)]
pub struct EditFriendCommand {
    name: String,
    #[arg(long)]
    new_name: Option<String>,
    #[arg(long)]
    level: Option<FriendLevel>,
}

#[derive(Args)]
pub struct ListFriendsCommand {
    #[arg(long = "type")]
    friend_type: Option<FriendLevel>,
    #[arg(long = "sort-chance")]
    sort_chance: bool,
}

#[derive(Subcommand)]
#[command(about = "Add or list friends")]
pub enum FriendCommand {
    Aji(FriendCommandBase),
    Ki(FriendCommandBase),
    Chi(FriendCommandBase),
    Rm(FriendCommandBase),
    Edit(EditFriendCommand),
    #[command(alias = "ls")]
    List(ListFriendsCommand),
}

#[derive(Parser)]
pub struct Friend {
    #[command(subcommand)]
    command: FriendCommand,
}

pub async fn handle(args: Friend) {
    match args.command {
        FriendCommand::Aji(name_wrapper) => config::add_aji(name_wrapper.name).await,
        FriendCommand::Ki(name_wrapper) => config::add_ki(name_wrapper.name).await,
        FriendCommand::Chi(name_wrapper) => config::add_chi(name_wrapper.name).await,
        FriendCommand::Rm(name_wrapper) => config::remove_friend(name_wrapper.name).await,
        FriendCommand::Edit(args) => {
            let level = args.level.map(|level| match level {
                FriendLevel::Aji => "aji".to_owned(),
                FriendLevel::Ki => "ki".to_owned(),
                FriendLevel::Chi => "chi".to_owned(),
            });
            config::edit_friend(args.name, args.new_name, level).await
        }
        FriendCommand::List(args) => {
            let friend_type = args.friend_type.map(|level| match level {
                FriendLevel::Aji => "aji".to_owned(),
                FriendLevel::Ki => "ki".to_owned(),
                FriendLevel::Chi => "chi".to_owned(),
            });
            config::list_friends(friend_type, args.sort_chance).await
        }
    }
}
