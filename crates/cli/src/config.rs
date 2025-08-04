use clap::{Parser, Subcommand};

/// Configures the dungeon game CLI
#[derive(Parser)]
#[command(name = "dungeon", about = "CLI for Dungeon Crawler")]
pub(crate) struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

/// Allowed commands for the dungeon game CLI
#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Play game in terminal UI (TUI)
    Play,
}
