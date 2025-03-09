use clearscreen::ClearScreen;
use std::collections::HashMap;
use std::io::{Read, Write, stdin, stdout};

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

fn main() {
    welcome_message();

    let mut board = Board::new();

    let player1 = Player {
        name: String::from("Player 1"),
        marker: String::from("X"),
    };

    let player2 = Player {
        name: String::from("Player 2"),
        marker: String::from("O"),
    };

    let mut players = [&player1, &player2];

    loop {
        board.draw_board();

        loop {
            let current_player = players[0];
            current_player.player_turn(&mut board);
            board.draw_board();

            board.add_turn();

            if board.check_if_winner(current_player) {
                println!(
                    "{} ({}) is the winner!",
                    current_player.name, current_player.marker
                );
                break;
            } else if board.check_if_draw() {
                println!("Draw!");
                break;
            }

            players.reverse();
        }

        let player_action = get_player_action();
        match player_action {
            Action::Replay => {
                continue;
            }
            Action::Quit => {
                println!("Thanks for playing!");
                break;
            }
        }
    }
}

struct Board {
    squares: HashMap<u8, String>,
    turn: u8,
}

impl Board {
    fn new() -> Self {
        let mut squares = HashMap::new();
        for i in 1..=9 {
            squares.insert(i, i.to_string());
        }
        Self { squares, turn: 0 }
    }

    fn draw_board(&self) {
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

    fn place_marker(&mut self, player_move: u8, current_player: &Player) {
        self.squares
            .insert(player_move, current_player.marker.to_owned());
    }

    fn add_turn(&mut self) {
        self.turn += 1
    }

    fn check_if_winner(&self, current_player: &Player) -> bool {
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

    fn check_if_draw(&self) -> bool {
        self.turn == 9
    }
}
struct Player {
    name: String,
    marker: String,
}

impl Player {
    fn player_turn(&self, board: &mut Board) {
        println!("{}'s ({}) turn:", self.name, self.marker);
        let player_move = self.get_player_move(&board.squares);

        board.place_marker(player_move, self);
    }

    fn get_player_move(&self, board: &HashMap<u8, String>) -> u8 {
        loop {
            let mut square = String::new();

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
enum Action {
    Replay,
    Quit,
}

fn welcome_message() {
    clear_screen();
    println!("Welcome to Tic-Tac-Toe!");
    println!("The goal is to get three of the same marker across a row, column, or diagonally.");
    println!("First one to do so, wins!");
    println!();
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

fn clear_screen() {
    ClearScreen::default()
        .clear()
        .expect("Failed to clear screen");
}

fn get_player_action() -> Action {
    println!("Play again? Enter y for yes or n for no:");

    loop {
        let mut response = String::new();

        stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.to_lowercase().trim() {
            "y" => return Action::Replay,
            "n" => return Action::Quit,
            _ => {
                println!("Please enter y or n:")
            }
        }
    }
}
