use structopt::StructOpt;

use crate::config;

#[derive(StructOpt)]
#[structopt(about = "Suggests a friend to connect with randomly")]
pub struct SuggestCommand {}

pub fn handle() {
    config::suggest()
}