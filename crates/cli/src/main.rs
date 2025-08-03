use colored::Colorize;
use std::{
    fmt::{Debug, Display},
    io::{Write, stdin, stdout},
};

use corelib::{Direction, GameState, PlayerAction, Position, Tile, new_game};

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
    Green,
}

#[derive(Debug, Clone, Copy)]
enum Symbol {
    Empty,
    Floor,
    Player,
}

#[derive(Debug, Clone, Copy)]
struct RenderItem {
    color: Color,
    symbol: Symbol,
}

impl Display for RenderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = match self.symbol {
            Symbol::Empty => '.',
            Symbol::Floor => '_',
            Symbol::Player => '@',
        }
        .to_string();
        let colored_sym = match self.color {
            Color::Black => sym.black(),
            Color::White => sym.white(),
            Color::Green => sym.green(),
        };
        write!(f, "{}", colored_sym)
    }
}

fn render(state: &GameState) {
    let mut field = [[RenderItem {
        color: Color::Black,
        symbol: Symbol::Empty,
    }; 9]; 9];

    let player = state.player();

    for row in -4..5 {
        for col in -4..5 {
            let tile = state
                .dungeon()
                .get_tile(&(player.position().clone() + Position::new(row, col)));

            let x_index = (col + 4) as usize;
            let y_index = (row + 4) as usize;
            match tile {
                Tile::Floor => {
                    field[x_index][y_index] = RenderItem {
                        color: Color::White,
                        symbol: Symbol::Floor,
                    };
                }
                Tile::Empty => {
                    field[x_index][y_index] = RenderItem {
                        color: Color::Black,
                        symbol: Symbol::Empty,
                    };
                }
            }
        }
    }

    field[4][4] = RenderItem {
        color: Color::Green,
        symbol: Symbol::Player,
    };

    for row in field {
        for cell in row {
            print!("{cell} ");
        }
        println!();
    }
}

fn main() {
    let mut state = new_game();

    loop {
        render(&state);
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "s" => state.apply_player_action(PlayerAction::Skip),
            "mn" => state.apply_player_action(PlayerAction::Move(Direction::North)),
            "ms" => state.apply_player_action(PlayerAction::Move(Direction::South)),
            "me" => state.apply_player_action(PlayerAction::Move(Direction::East)),
            "mw" => state.apply_player_action(PlayerAction::Move(Direction::West)),
            "q" => break,
            _ => println!("Unknown command"),
        }
    }
}
