use clap::{Args, Parser, Subcommand};

use crate::config;

#[derive(Args)]
pub struct MemoryCommandBase {
    names: Vec<String>,
}

#[derive(Args)]
pub struct MemoryIdCommandBase {
    id: i64,
}

#[derive(Subcommand)]
#[command(about = "Add a memory with one or more friends")]
/// Memory-related subcommands.
pub enum MemoryCommand {
    Hangout(MemoryCommandBase),
    VideoCall(MemoryCommandBase),
    Call(MemoryCommandBase),
    Text(MemoryCommandBase),
    Suggest,
    Undo,
    Remove(MemoryIdCommandBase),
}

#[derive(Parser)]
/// Root wrapper for `akc memory ...`.
pub struct Memory {
    #[command(subcommand)]
    command: MemoryCommand,
}

/// Executes parsed memory commands.
pub async fn handle(args: Memory) {
    match args.command {
        MemoryCommand::Hangout(names_wrapper) => config::add_hangout(&names_wrapper.names).await,
        MemoryCommand::VideoCall(names_wrapper) => {
            config::add_video_call(&names_wrapper.names).await
        }
        MemoryCommand::Call(names_wrapper) => config::add_call(&names_wrapper.names).await,
        MemoryCommand::Text(names_wrapper) => config::add_text(&names_wrapper.names).await,
        MemoryCommand::Suggest => config::suggest().await,
        MemoryCommand::Undo => config::undo_memory().await,
        MemoryCommand::Remove(args) => config::remove_memory(args.id).await,
    }
}
