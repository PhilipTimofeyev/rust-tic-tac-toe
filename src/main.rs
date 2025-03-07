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

fn main() {
    let mut board = initialize_board();
    let mut current_marker = String::from("X");

    loop {
        print!("{}[2J", 27 as char);
        let player_move = player_move();
        current_marker = alternate_marker(current_marker);

        board.insert(player_move, current_marker.to_string());
        draw_board(&board);
        if check_if_winner(&board, &current_marker) {
            println!("{} is the winner!", current_marker);
            break;
        }
    }
}

fn initialize_board() -> HashMap<u8, String> {
    let mut board = HashMap::new();

    board.insert(1, " ".to_string());
    board.insert(2, " ".to_string());
    board.insert(3, " ".to_string());

    board.insert(4, " ".to_string());
    board.insert(5, " ".to_string());
    board.insert(6, " ".to_string());

    board.insert(7, " ".to_string());
    board.insert(8, " ".to_string());
    board.insert(9, " ".to_string());

    board
}

fn check_if_winner(board: &HashMap<u8, String>, current_marker: &String) -> bool {
    for row in WINNING_SQUARES {
        let mut count: u8 = 0;

        for square in row {
            match board.get(&square) {
                Some(i) => {
                    if i == current_marker {
                        count += 1
                    }
                }
                None => (),
            };
            if count == 3 {
                return true;
            }
        }
    }
    false
}

fn alternate_marker(marker: String) -> String {
    if marker == "X" {
        String::from("O")
    } else {
        String::from("X")
    }
}

enum Marker {
    X,
    O,
    Empty,
}

fn player_move() -> u8 {
    loop {
        let mut square = String::new();

        io::stdin()
            .read_line(&mut square)
            .expect("Failed to read line");

        let square: u8 = match square.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return square;
    }
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
