use std::collections::HashMap;
use std::io;

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

struct Player {
    name: String,
    marker: String,
}
enum Action {
    Replay,
    Quit,
}

fn main() {
    welcome_message();

    loop {
        let mut board = initialize_board();
        let mut turn: u8 = 0;

        let player1 = Player {
            name: String::from("Player 1"),
            marker: String::from("X"),
        };
        let player2 = Player {
            name: String::from("Player 2"),
            marker: String::from("O"),
        };

        let mut players = [player1, player2];

        draw_board(&board);

        loop {
            clear_screen();

            player_turn(&mut board, &players[0]);
            draw_board(&board);

            turn += 1;

            if check_if_winner(&board, &players[0], turn) {
                break;
            } else if check_if_draw(turn) {
                break;
            }

            players.reverse();
        }

        let player_action = get_player_action();
        match player_action {
            Action::Replay => {
                println!("Starting new game");
                continue;
            }
            Action::Quit => {
                println!("Thanks for playing!");
                break;
            }
        }
    }
}

fn welcome_message() {
    println!("Welcome to Tic-Tac-Toe!");
    println!("The goal is to get three of the same marker across a row, column, or diagonally.");
    println!("First one to do so, wins!");
    println!("");
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn initialize_board() -> HashMap<u8, String> {
    let mut board = HashMap::new();
    for i in 1..=9 {
        board.insert(i, i.to_string());
    }
    board
}

fn draw_board(board: &HashMap<u8, String>) {
    let a = board.get(&1).unwrap();
    let b = board.get(&2).unwrap();
    let c = board.get(&3).unwrap();

    let d = board.get(&4).unwrap();
    let e = board.get(&5).unwrap();
    let f = board.get(&6).unwrap();

    let g = board.get(&7).unwrap();
    let h = board.get(&8).unwrap();
    let i = board.get(&9).unwrap();

    println!("-------------");
    println!("| {} | {} | {} |", a, b, c);
    println!("-------------");
    println!("| {} | {} | {} |", d, e, f);
    println!("-------------");
    println!("| {} | {} | {} |", g, h, i);
    println!("-------------");
}

fn player_turn(board: &mut HashMap<u8, String>, current_player: &Player) {
    let player_move = get_player_move(&board);

    place_marker(board, player_move, current_player);
}

fn get_player_move(board: &HashMap<u8, String>) -> u8 {
    loop {
        let mut square = String::new();

        io::stdin()
            .read_line(&mut square)
            .expect("Failed to read line");

        let square: u8 = match square.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match board.get(&square) {
            Some(i) => {
                if i == "X" || i == "O" {
                    continue;
                } else {
                    return square;
                }
            }
            None => return square,
        };
    }
}

fn place_marker(board: &mut HashMap<u8, String>, player_move: u8, current_player: &Player) {
    board.insert(player_move, current_player.marker.to_owned());
}

fn check_if_winner(board: &HashMap<u8, String>, current_player: &Player, turn: u8) -> bool {
    if turn < 5 {
        return false;
    }

    for row in WINNING_SQUARES {
        let mut count: u8 = 0;

        for square in row {
            match board.get(&square) {
                Some(i) => {
                    if *i == current_player.marker {
                        count += 1
                    }
                }
                None => (),
            };
            if count == 3 {
                println!("{} is the winner!", current_player.name);
                return true;
            }
        }
    }
    false
}

fn check_if_draw(turn: u8) -> bool {
    if turn == 9 {
        println!("Draw!");
        true
    } else {
        false
    }
}

fn get_player_action() -> Action {
    println!("Play again? Enter y for yes or n for no:");

    loop {
        let mut response = String::new();

        io::stdin()
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
