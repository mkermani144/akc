use clap::Parser;

use crate::config;

#[derive(Parser)]
#[command(about = "Suggests a friend to connect with randomly")]
/// Command wrapper for `akc suggest`.
pub struct SuggestCommand {}

/// Runs suggestion flow.
pub async fn handle() {
    config::suggest().await
}
