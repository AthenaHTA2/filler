//main.rs is the entry point of the program
use std::io::{self, BufRead, Write};
use filler_lib::{Tokens, expand_right, check_right, expand_left, check_left, find_opponent};

fn main() {

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read my player number
    let mut p_info = String::new();
    stdin.lock().read_line(&mut p_info).unwrap();

    //split_whitespace returns an iterator over a slice of sub-strings;
    //nth returns the specific substring, indexes start from 0;
    //unwrap() is used to get the value from the Option
    let me usize = p_info
        .split_whitespace()
        .nth(3)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    // Assign symbols to self and opponent
    let tokens = Tokens::new(me);   

    // Read the Anfield dimensions and state
    let mut anfield_info = String::new();
    stdin.lock().read_line(&mut anfield_info).unwrap();
    let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
    //e.g. 15 rows
    //parse() is used to convert the string to integer
    let rows: usize = dimensions[1].parse().unwrap();
    let cols: usize = dimensions[2].parse().unwrap();

    let mut anfield = Vec::new();
    //Note: 0 to 15 gives 16 rows
    for _ in 0..rows {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        anfield.push(line.trim().to_string());
    }

    //now remove the slice at index '0' 
    //because don't need column headings
    anfield.pop_front();

    // Read the piece dimensions and shape
    let mut piece_info = String::new();
    stdin.lock().read_line(&mut piece_info).unwrap();
    let piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
    let piece_width: usize = piece_dimensions[1].parse().unwrap();
    let piece_height: usize = piece_dimensions[2].parse().unwrap();

    let mut piece = Vec::new();
    for _ in 0..piece_height {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        piece.push(line.trim().to_string());
    }

    // Determine the next move
    // Simple approach: place the piece so its first cell
    // will sit on my previous piece's last cell, if possible
    let (x, y) = expand_right(&anfield, &piece, &tokens);
    //check if last character can be used as anchor for next piece
    let (available) = check_right(&anfield, &piece, &tokens);

    let(x, y) = if available {
        (x, y)
    } else {
        expand_left(&anfield, &piece, &tokens)
    };

    let (x_opp, y_opp) = find_opponent(&anfield, &tokens);

    // Output the coordinates to the game engine
    //println!("{} {}", x, y);
    writeln!(stdout, "{} {}", x, y).unwrap();
    stdout.flush().unwrap();

}
