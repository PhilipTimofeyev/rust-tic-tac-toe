use std::io::stdin;
mod config;
mod misc;

pub fn play() {
    welcome_message();

    let config = config::Config::build();
    let mut board = config.board;
    let mut players = config.players;

    loop {
        board.draw_board();

        loop {
            let current_player = &players[0];
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

enum Action {
    Replay,
    Quit,
}

fn welcome_message() {
    misc::clear_screen();
    println!("Welcome to Tic-Tac-Toe!");
    println!("The goal is to get three of the same marker across a row, column, or diagonally.");
    println!("First one to do so, wins!");
    println!();
    misc::pause();
}

fn get_player_action() -> Action {
    println!("Play again? Enter y for yes or n for no:");

    loop {
        let mut response = String::with_capacity(1);

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
