use std::collections::HashMap;

use crate::config::player::Player;
use crate::misc::clear_screen;

const WINNING_SQUARES: [[u8; 3]; 8] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9],
    [1, 5, 9],
    [3, 5, 7],
];

pub struct Board {
    pub squares: HashMap<u8, String>,
    pub turn: u8,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = HashMap::new();
        for i in 1..=9 {
            squares.insert(i, i.to_string());
        }
        Self { squares, turn: 0 }
    }

    pub fn draw_board(&self) {
        clear_screen();

        let a = self.squares.get(&1).unwrap();
        let b = self.squares.get(&2).unwrap();
        let c = self.squares.get(&3).unwrap();

        let d = self.squares.get(&4).unwrap();
        let e = self.squares.get(&5).unwrap();
        let f = self.squares.get(&6).unwrap();

        let g = self.squares.get(&7).unwrap();
        let h = self.squares.get(&8).unwrap();
        let i = self.squares.get(&9).unwrap();

        println!("-------------");
        println!("| {} | {} | {} |", a, b, c);
        println!("-------------");
        println!("| {} | {} | {} |", d, e, f);
        println!("-------------");
        println!("| {} | {} | {} |", g, h, i);
        println!("-------------");
    }

    pub fn place_marker(&mut self, player_move: u8, current_player: &Player) {
        self.squares
            .insert(player_move, current_player.marker.to_owned());
    }

    pub fn add_turn(&mut self) {
        self.turn += 1
    }

    pub fn check_if_winner(&self, current_player: &Player) -> bool {
        if self.turn < 5 {
            return false;
        }

        for row in WINNING_SQUARES {
            let mut count: u8 = 0;

            for square in row {
                if let Some(i) = self.squares.get(&square) {
                    if *i == current_player.marker {
                        count += 1
                    }
                };
                if count == 3 {
                    return true;
                }
            }
        }
        false
    }

    pub fn check_if_draw(&self) -> bool {
        self.turn == 9
    }
}
