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
        if let Some(square) = self.squares.get_mut(&player_move) {
            *square = current_player.marker.to_string()
        }
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
        self.turn >= 9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_board() {
        let board = Board::new();

        assert_eq!(board.turn, 0); // Starts with 0 turns

        let num_of_squares = board.squares.iter().count();
        assert_eq!(num_of_squares, 9) // Contains 9 squares
    }

    #[test]
    fn adding_turn() {
        let mut board = Board::new();

        board.add_turn();
        assert_eq!(board.turn, 1);
    }

    #[test]
    fn placing_marker() {
        let mut board = Board::new();
        let player_1 = Player {
            name: String::from("Player 1"),
            marker: String::from("X"),
        };
        let player_2 = Player {
            name: String::from("Player 2"),
            marker: String::from("O"),
        };

        board.place_marker(1, &player_1);
        assert_eq!(board.squares.get(&1).unwrap(), "X");

        board.place_marker(9, &player_2);
        assert_eq!(board.squares.get(&9).unwrap(), "O");

        board.place_marker(12, &player_1); //Returns None if square outside hash range
        assert_eq!(board.squares.get(&12), None);
    }

    #[test]
    fn verify_draw() {
        let mut board = Board::new();

        board.turn = 8;
        assert!(!board.check_if_draw());

        board.turn = 9;
        assert!(board.check_if_draw());
    }

    #[test]
    fn verify_row_winning_squares() {
        let mut board = Board::new();
        let player_1 = Player {
            name: String::from("Player 1"),
            marker: String::from("X"),
        };
        board.turn = 5; // Prevent automatic false from too few turns

        let mut result = board.check_if_winner(&player_1);
        assert!(!result);

        // Check row
        board.squares.insert(1, "X".to_string());
        board.squares.insert(2, "X".to_string());
        board.squares.insert(3, "X".to_string());

        result = board.check_if_winner(&player_1);

        println!("{:#?}", board.squares);
        assert!(result)
    }

    #[test]
    fn verify_column_winning_squares() {
        let mut board = Board::new();
        let player_1 = Player {
            name: String::from("Player 1"),
            marker: String::from("X"),
        };
        board.turn = 5;

        // Check column
        board.squares.insert(1, "X".to_string());
        board.squares.insert(4, "X".to_string());
        board.squares.insert(7, "X".to_string());

        let result = board.check_if_winner(&player_1);

        println!("{:#?}", board.squares);
        assert!(result)
    }

    #[test]
    fn verify_diagonal_winning_squares() {
        let mut board = Board::new();
        let player_1 = Player {
            name: String::from("Player 1"),
            marker: String::from("X"),
        };
        board.turn = 5;

        // Check diagonal
        board.squares.insert(1, "X".to_string());
        board.squares.insert(5, "X".to_string());
        board.squares.insert(9, "X".to_string());

        let result = board.check_if_winner(&player_1);

        println!("{:#?}", board.squares);
        assert!(result)
    }
}
