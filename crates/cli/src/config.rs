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
    Play {
        /// Seed for the random number generator.
        #[arg(long)]
        seed: Option<u64>,
        /// Width of the map.
        #[arg(long)]
        map_width: Option<usize>,
        /// Height of the map.
        #[arg(long)]
        map_height: Option<usize>,
        /// Maximum number of tiles per floor.
        #[arg(long)]
        floor_tiles: Option<usize>,
        /// Maximum number of enemies.
        #[arg(long)]
        enemies: Option<usize>,
    },
}
