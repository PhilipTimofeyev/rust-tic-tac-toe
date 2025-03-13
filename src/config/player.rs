use crate::config::board::Board;
use std::collections::HashMap;
use std::io::stdin;

#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub marker: String,
}

impl Player {
    pub fn player_turn(&self, board: &mut Board) {
        println!("{}'s ({}) turn:", self.name, self.marker);
        let player_move = self.get_player_move(&board.squares);

        board.place_marker(player_move, self);
    }

    fn get_player_move(&self, board: &HashMap<u8, String>) -> u8 {
        loop {
            let mut square = String::with_capacity(1);

            stdin().read_line(&mut square).expect("Failed to read line");

            let square: u8 = match square.trim().parse() {
                Ok(num) => {
                    if num > 0 && num < 10 {
                        num
                    } else {
                        println!("Please enter a number between 1 and 9");
                        continue;
                    }
                }
                Err(_) => continue,
            };

            match board.get(&square) {
                Some(i) => {
                    if i == "X" || i == "O" {
                        println!("Please select an empty square");
                        continue;
                    } else {
                        return square;
                    }
                }
                None => return square,
            };
        }
    }
}
