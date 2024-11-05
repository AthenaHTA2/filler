//main.rs is the entry point of the program
use std::io::{self, BufRead, Write};
use filler_lib::{run};

fn main() {
    println!("Hello, world!");
    run();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the player number
    let mut player_info = String::new();
    stdin.lock().read_line(&mut player_info).unwrap();

    // Read the Anfield dimensions and state
    let mut anfield_info = String::new();
    stdin.lock().read_line(&mut anfield_info).unwrap();
    let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
    let rows: usize = dimensions[1].parse().unwrap();
    let cols: usize = dimensions[2].parse().unwrap();

    let mut anfield = Vec::new();
    for _ in 0..rows {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        anfield.push(line.trim().to_string());
    }

    // Read the piece dimensions and shape
    let mut piece_info = String::new();
    stdin.lock().read_line(&mut piece_info).unwrap();
    let piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
    let piece_rows: usize = piece_dimensions[1].parse().unwrap();
    let piece_cols: usize = piece_dimensions[2].parse().unwrap();

    let mut piece = Vec::new();
    for _ in 0..piece_rows {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        piece.push(line.trim().to_string());
    }

    // Process the input to determine the best move
    // For simplicity, let's assume we always place the piece at (0, 0)
    let x = 0;
    let y = 0;

    // Output the coordinates to the game engine
    writeln!(stdout, "{} {}", x, y).unwrap();
    stdout.flush().unwrap();

}
