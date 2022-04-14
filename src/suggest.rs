use clap::Parser;

use crate::config;

#[derive(Parser)]
#[clap(about = "Suggests a friend to connect with randomly")]
pub struct SuggestCommand {}

pub fn handle() {
    config::suggest()
}