use clap::{Args, Parser, Subcommand};

use crate::config;

#[derive(Args)]
pub struct MemoryCommandBase {
    names: Vec<String>,
}

#[derive(Subcommand)]
#[command(about = "Add a memory with one or more friends")]
pub enum MemoryCommand {
    Hangout(MemoryCommandBase),
    VideoCall(MemoryCommandBase),
    Call(MemoryCommandBase),
    Text(MemoryCommandBase),
    Suggest,
}

#[derive(Parser)]
pub struct Memory {
    #[command(subcommand)]
    command: MemoryCommand,
}

pub async fn handle(args: Memory) {
    match args.command {
        MemoryCommand::Hangout(names_wrapper) => config::add_hangout(&names_wrapper.names).await,
        MemoryCommand::VideoCall(names_wrapper) => {
            config::add_video_call(&names_wrapper.names).await
        }
        MemoryCommand::Call(names_wrapper) => config::add_call(&names_wrapper.names).await,
        MemoryCommand::Text(names_wrapper) => config::add_text(&names_wrapper.names).await,
        MemoryCommand::Suggest => config::suggest().await,
    }
}
