use clap::Parser;

mod commands;
mod config;
mod tui;

fn main() {
    let c = config::Config::parse();

    match c.command {
        config::Commands::Play => commands::play::play(),
    }
}
