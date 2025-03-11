mod board;
pub mod player;

pub struct Config {
    pub board: board::Board,
    pub players: [player::Player; 2],
}

impl Config {
    pub fn build() -> Config {
        let board = board::Board::new();

        let player1 = player::Player {
            name: String::from("Player 1"),
            marker: String::from("X"),
        };

        let player2 = player::Player {
            name: String::from("Player 2"),
            marker: String::from("O"),
        };

        let players = [player1, player2];

        Config { board, players }
    }
}
