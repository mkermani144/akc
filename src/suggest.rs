use clap::Parser;

use crate::config;

#[derive(Parser)]
#[command(about = "Suggests a friend to connect with randomly")]
pub struct SuggestCommand {}

pub async fn handle() {
    config::suggest().await
}
