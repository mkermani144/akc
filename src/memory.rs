use structopt::StructOpt;

use crate::config;

#[derive(StructOpt)]
pub struct MemoryCommandBase {
    names: Vec<String>
}

#[derive(StructOpt)]
#[structopt(about = "Add a memory with one or more friends")]
pub enum MemoryCommand {
    Hangout(MemoryCommandBase),
    VideoCall(MemoryCommandBase),
    Call(MemoryCommandBase),
    Text(MemoryCommandBase),
}

pub fn handle(args: MemoryCommand) {
    match args {
        MemoryCommand::Hangout(names_wrapper) => config::add_hangout(names_wrapper.names),
        MemoryCommand::VideoCall(names_wrapper) => config::add_video_call(names_wrapper.names),
        MemoryCommand::Call(names_wrapper) => config::add_call(names_wrapper.names),
        MemoryCommand::Text(names_wrapper) => config::add_text(names_wrapper.names),
    }
}