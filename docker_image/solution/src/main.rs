//main.rs is the entry point of the program
use std::io::{self, BufRead, Write};
use std::collections::VecDeque;
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
    let me: usize = p_info
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
    
    //continuously read Anfield and piece data
    //and print the coordinates of the next move
    loop {

        // Read the Anfield dimensions and state
        //e.g.: Anfield 20 15:
        let mut anfield_info = String::new();
        stdin.lock().read_line(&mut anfield_info).unwrap();
        let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
        // Remove the colon at the end and parse the number of rows
        let rows: usize = dimensions[2].trim_end_matches(':').parse().unwrap();
        //not used: let cols: usize = dimensions[2].parse().unwrap();
        //I am using VecDeque because I need to remove the first element
        //and the VecDeque has the pop_front() method
        let mut anfield = VecDeque::new();
        //Note: 0 to 15 gives 16 rows
        for _ in 0..rows {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        anfield.push_back(line.trim().to_string());
        }

        //now remove the slice at index '0' 
        //because don't need column headings
        anfield.pop_front();

        // Read the piece dimensions and shape
        let mut piece_info = String::new();
        stdin.lock().read_line(&mut piece_info).unwrap();
        let piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
        //not used: let piece_width: usize = piece_dimensions[1].parse().unwrap();
        //remove the colon at the end and parse the number of rows
        let piece_height: usize =  piece_dimensions[2].trim_end_matches(':').parse().unwrap();
        let mut piece = Vec::new();
        for _ in 0..piece_height {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        piece.push(line.trim().to_string());
        }

        // Determine the next move
        // Simple approach: place the piece so its first cell
        // will sit on my previous piece's last cell, if possible
        let (right_x, right_y) = expand_right(&anfield, &tokens);
        //check if last character to the right can be used as anchor for next piece
        let available_right = check_right(&anfield, &piece, &tokens);
        let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
        let available_left = check_left(&anfield, &piece, &tokens);
        // Find the opponent's last cell
        //let(foe_x, foe_y) = find_opponent(&anfield, &tokens);

        let (mut x, mut y): (Option<usize>, Option<usize>) = (None, None);

        if available_right {
        (x, y) = (Some(right_x), Some(right_y));
        } else if available_left {
        (x, y) = (Some(left_x), Some(left_y));
        }else{
        //kill the game
        (x, y) = (Some(0), Some(0));
        };

        // Output the coordinates to the game engine
        //println!("{} {}", x, y);
        // Unwrap the Option values or handle the None case
        let x_value = x.unwrap_or(0); // Default to 0 if None
        let y_value = y.unwrap_or(0); // Default to 0 if None
        // Output the coordinates to the game engine
        write!(stdout, "{} {}\n", x_value, y_value).unwrap();
        stdout.flush().unwrap();
        //println!("{} {}", x_value, y_value);
    }

}
