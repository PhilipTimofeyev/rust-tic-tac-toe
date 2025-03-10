use clearscreen::ClearScreen;
use std::io::{Read, Write, stdin, stdout};

pub fn clear_screen() {
    ClearScreen::default()
        .clear()
        .expect("Failed to clear screen");
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
