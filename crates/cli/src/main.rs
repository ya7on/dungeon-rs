use clap::{Error, Parser, error::ErrorKind};

mod commands;
mod config;
mod tui;

const DEFAULT_SEED: u64 = 0;
const DEFAULT_MAP_WIDTH: usize = 101;
const DEFAULT_MAP_HEIGHT: usize = 101;

fn main() {
    let c = config::Config::parse();

    match c.command {
        config::Commands::Play {
            seed,
            map_width,
            map_height,
            enemies,
            floor_tiles,
        } => {
            let seed = seed.unwrap_or(DEFAULT_SEED);
            let map_width = map_width.unwrap_or(DEFAULT_MAP_WIDTH);
            let map_height = map_height.unwrap_or(DEFAULT_MAP_HEIGHT);
            let floor_tiles = floor_tiles
                .unwrap_or(DEFAULT_MAP_WIDTH * DEFAULT_MAP_HEIGHT / 3);
            let enemies = enemies.unwrap_or(floor_tiles / 500);

            if floor_tiles > map_width * map_height {
                Error::raw(
                    ErrorKind::ValueValidation,
                    "floor_tiles must not exceed map area (width × height)\n",
                )
                .exit();
            }

            if enemies > floor_tiles - 1 {
                Error::raw(
                    ErrorKind::ValueValidation,
                    "Too many enemies: must leave at least one floor tile free\n",
                )
                .exit();
            }

            commands::play::play(
                seed,
                map_width,
                map_height,
                floor_tiles,
                enemies,
            );
        },
    }
}
