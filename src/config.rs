use serde::{Serialize, Deserialize};
use serde_yaml;
use clap::Subcommand;

#[derive(Serialize, Deserialize)]
struct Settings {
    endpoint: String,
    region: String
}

#[derive(Subcommand)]
/// Alter configuration settings
pub enum Commands {
    // Settings' subcommands
    Get,
    Set
}

