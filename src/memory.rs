use clap::{Args, Parser, Subcommand};

use crate::config;

#[derive(Args)]
pub struct MemoryCommandBase {
    names: Vec<String>
}

#[derive(Subcommand)]
#[clap(about = "Add a memory with one or more friends")]
pub enum MemoryCommand {
    Hangout(MemoryCommandBase),
    VideoCall(MemoryCommandBase),
    Call(MemoryCommandBase),
    Text(MemoryCommandBase),
}

#[derive(Parser)]
pub struct Memory {
    #[clap(subcommand)]
    command: MemoryCommand,
}

pub fn handle(args: Memory) {
    match args.command {
        MemoryCommand::Hangout(names_wrapper) => config::add_hangout(&names_wrapper.names),
        MemoryCommand::VideoCall(names_wrapper) => config::add_video_call(&names_wrapper.names),
        MemoryCommand::Call(names_wrapper) => config::add_call(&names_wrapper.names),
        MemoryCommand::Text(names_wrapper) => config::add_text(&names_wrapper.names),
    }
}