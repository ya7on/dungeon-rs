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
    Red,
}

#[derive(Debug, Clone, Copy)]
enum Symbol {
    Empty,
    Floor,
    Player,
    Enemy,
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
            Symbol::Enemy => 'E',
        }
        .to_string();
        let colored_sym = match self.color {
            Color::Black => sym.black(),
            Color::White => sym.white(),
            Color::Green => sym.green(),
            Color::Red => sym.red(),
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
    println!("HP: {}", player.stats().hp());

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

    for entity in state.entities() {
        let position = entity.position();
        let relative_position = position.clone() - player.position().clone();

        if relative_position.x <= 4
            && relative_position.y <= 4
            && relative_position.x >= -4
            && relative_position.y >= -4
        {
            field[(relative_position.y + 4) as usize][(relative_position.x + 4) as usize] =
                RenderItem {
                    color: Color::Red,
                    symbol: Symbol::Enemy,
                };
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
            "an" => state.apply_player_action(PlayerAction::Attack(Direction::North)),
            "as" => state.apply_player_action(PlayerAction::Attack(Direction::South)),
            "ae" => state.apply_player_action(PlayerAction::Attack(Direction::East)),
            "aw" => state.apply_player_action(PlayerAction::Attack(Direction::West)),
            "q" => break,
            _ => println!("Unknown command"),
        }
    }
}
