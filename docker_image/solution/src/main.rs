//main.rs is the entry point of the program

use std::io::{self, BufRead, Write};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use filler_lib::{Tokens, expand_right, check_right, expand_left, check_left, find_opponent};

fn main() {

/*alternative reading of input

use std::io;

let mut input = String::new();

io::stdin().read_line(&mut input).unwrap();

*/

    let stdin = io::stdin();
    //The println! macro will lock the standard output on each call. 
    //If you call println! within a hot loop, this behavior may be the bottleneck of the loop. 
    //To avoid this, lock stdout with io::stdout().lock():
    let mut stdout = io::stdout();
    //let mut stdou = stdout.lock();
/*
    // Simulate an "Enter" key press after a delay
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        let mut stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "").unwrap();
    });
*/
    // Read my player number
    let mut p1_info = String::new();
    stdin.lock().read_line(&mut p1_info).unwrap();
    //stdin.read_line(&me_info).unwrap();
    println!("p1_info:{:?}", p1_info);
    //split_whitespace returns an iterator over a slice of sub-strings;
    //nth returns the specific substring, indexes start from 0;
    //unwrap() is used to get the value from the Option
    let p1 = p1_info
        .split_whitespace()
        .nth(2)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
        
    println!("p1:{:?}", p1);
    // Assign symbols to self and opponent 
    
    //read foe info
    let mut p2_info = String::new();
    stdin.lock().read_line(&mut p2_info).unwrap();
    //split_whitespace returns an iterator over a slice of sub-strings;
    //nth returns the specific substring, indexes start from 0;
    let p2 = p2_info
        .split_whitespace()
        .nth(2)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    
    println!("p2:{:?}", p2);

    let mut me: usize = 0;

    //detect which player I am
    if p1_info == "$$$ exec p1 : [solution/target/release/filler]\n"{
        me = p1;
    } else {
        me = p2;
    }

    println!("me:{:?}", me);
    
    let tokens = Tokens::new(me);

    //continuously read Anfield and piece data
    //and print the coordinates of the next move
    
    loop {

        // Read the Anfield dimensions and state
        //e.g.: Anfield 20 15:
        let mut anfield_info = String::new();
        stdin.lock().read_line(&mut anfield_info).unwrap();
        let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
        println!("dimensions:{:?}", dimensions);
        // Remove the colon at the end and parse the number of rows
        let rows: usize = dimensions[2].trim_end_matches(':').parse().unwrap();
        //let anf_rows = dimensions[2].trim_end_matches(':');
        //println!("anf_rows:{}", anf_rows);
        //let rows: usize = anf_rows.parse().unwrap();
        println!("rows:{}", rows);
        //not used: let cols: usize = dimensions[2].parse().unwrap();
        //I am using VecDeque because I need to remove the first element
        //and the VecDeque has the pop_front() method
        
        let mut anfield = VecDeque::new();
        //Note: 0 to 15 gives 16 rows
        for _ in 0..rows {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        //remove row number and space at the beginning
        let off_with_head: Vec<&str> = line.split_whitespace().collect();
        if off_with_head.len() > 1 {
        line = off_with_head[1].to_string();
        anfield.push_back(line.trim().to_string());
        } else {
        anfield.push_back(line.trim().to_string());
        }
    }
        println!("anfield after removing row headers:{:?}", anfield);
        //now remove the slice at index '0' 
        //because don't need column headings
        anfield.pop_front();
        println!("anfield after removing column headers:{:?}", anfield);
        //This is the last Anfield row that is stored twice and I don't need it
        let mut extra_line = String::new();
        let _throw_away = stdin.lock().read_line(&mut extra_line).unwrap();
        // Read the piece dimensions and shape
        let mut piece_info = String::new();
        stdin.lock().read_line(&mut piece_info).unwrap();
        let mut piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
        println!("piece_dimensions:{:?}", piece_dimensions);
        //not used: let piece_width: usize = piece_dimensions[1].parse().unwrap();
        //remove the colon at the end and parse the number of rows
        let mut piece_height: usize =  piece_dimensions[2].trim_end_matches(':').parse().unwrap();
        println!("piece_height:{}", piece_height);
        let mut piece = Vec::new();
        //looping from 0 up to excluding height    
        for _ in 0..piece_height {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            piece.push(line.trim().to_string());
        }
        println!("piece:{:?}", piece);
        //if !piece.is_empty() {
            //println!("piece length:{}", piece[0].len());
        //}
        // Determine the next move
        // Simple approach: place the piece so its first cell
        // will sit on my previous piece's last cell, if possible
        let (right_x, right_y) = expand_right(&anfield, &tokens);
        println!("right_x:{} right_y:{}", right_x, right_y);
        //check if last character to the right can be used as anchor for next piece
        let available_right = check_right(&anfield, &piece, &tokens);
        println!("available_right:{}", available_right);
        let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
        println!("left_x:{} left_y:{}", left_x, left_y);
        let available_left = check_left(&anfield, &piece, &tokens);
        println!("available_left:{}", available_left);
        // Find the opponent's last cell
        let(foe_x, foe_y) = find_opponent(&anfield, &tokens);
        println!("foe_x:{} foe_y:{}", foe_x, foe_y);
        let (mut x, mut y): (usize, usize) = (0, 0);

        if available_right {
            (x, y) = (right_x, right_y);
        } else if available_left {
            (x, y) = (left_x, left_y);
        }else{
            //kill the game
            (x, y) = (foe_x, foe_y);
        };

        println!("my move is: {} {}", x, y);
        //io::stdout().lock().write_all(format!("{} {}\n", x, y).as_bytes()).unwrap();
        //writeln!(stdou, " {} {}", x, y).unwrap();
        //stdou.flush().unwrap();
        
        // Debugging statement to verify output
        //eprintln!("Outputting coordinates: {} {}", x, y);

        // Output the coordinates to the game engine
        //println!("{} {}", x, y);
        // Unwrap the Option values or handle the None case
        //let x_value = x.unwrap_or(0); // Default to 0 if None
        //let y_value = y.unwrap_or(0); // Default to 0 if None
        // Output the coordinates to the game engine
        //writeln!(stdout.lock(), " {} {}", x, y).unwrap();
        let mut lok = stdout.lock();
        writeln!(lok, "{} {}", x, y).unwrap();
        lok.flush().unwrap();
    }//end of loop
}
    
/*
mod player_symbols;

use std::{ io::{ self, Read, BufRead }, collections::VecDeque };

//use player_symbols::GameSymbols;

pub struct GameSymbols {
    pub my_recent_symbol: char,
    pub my_territory_symbol: char,
    pub opponent_recent_symbol: char,
    pub opponent_territory_symbol: char,
}

impl GameSymbols {
    pub fn new(player_number: usize) -> Self {
        if player_number == 1 {
            return GameSymbols {
                my_recent_symbol: 'a',
                my_territory_symbol: '@',
                opponent_recent_symbol: 's',
                opponent_territory_symbol: '$',
            };
        } else {
            return GameSymbols {
                my_recent_symbol: 's',
                my_territory_symbol: '$',
                opponent_recent_symbol: 'a',
                opponent_territory_symbol: '@',
            };
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap());

    // Read player information
    // that is sent just once at the start
    let player_info = lines.next().unwrap();
    let player_number: usize = player_info
        .split_whitespace()
        .nth(3)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    // Define player symbols
    let symbols = GameSymbols::new(player_number);

    // Process game turns
    loop {
        // Read Anfield and piece information
        //split_whitespace returns an iterator over a slice of sub-strings;
        //nth returns the specific substring, indexes start from 0;
        //unwrap() is used to get the value from the Option
        //parse() is used to convert the string to integer
        let anfield_size_info = lines.next().unwrap();
        let vertical_anfield_size = anfield_size_info
            .split_whitespace()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        //map() maps the values of iter with the function 
        //in the closure. Noter that 0 to 15 gives 16 lines
        let mut anfield: VecDeque<String> = (0..=vertical_anfield_size)
            .map(|_| lines.next().unwrap())
            .collect();

        //remove x axis number labels
        //i.e. "01234567890123456789"
        anfield.pop_front();

        let piece_info = lines.next().unwrap();
        let piece_y_size: usize = piece_info.split_whitespace().nth(2).unwrap().parse().unwrap();

        let piece: VecDeque<String> = (0..piece_y_size).map(|_| lines.next().unwrap()).collect();

        // Your game logic goes here to determine the next move
        let (next_x, next_y) = find_next_move(&anfield, &piece, &symbols);

        // Output the move
        //println!("{} {}", next_x, next_y);
    }
}

// Example logic to find the next move
fn find_next_move(
    anfield: &VecDeque<String>,
    piece: &VecDeque<String>,
    symbols: &GameSymbols
) -> (usize, usize) {
    // Your logic to determine the next move based on the Anfield goes here
    
    for y in 0..anfield.len() {
        for x in 0..anfield[0].len() {
            // Check if the piece can be placed at (x, y)
            if can_place_piece(&anfield, &piece, x, y, &symbols) {
                return (x, y);
            }
        }
    }

    // Else place the piece in the center of the Anfield 
    let center_x = anfield[0].len() / 2;
    let center_y = anfield.len() / 2;

    (center_x, center_y)
}

// Function to check if the piece can be placed at a specific position
fn can_place_piece(
    anfield: &VecDeque<String>,
    piece: &VecDeque<String>,
    x: usize,
    y: usize,
    symbols: &GameSymbols
) -> bool {
    // Check if the piece can be placed at (x, y) without overlapping opponent's territory
    for (i, row) in piece.iter().enumerate() {
        for (j, piece_cell) in row.chars().enumerate() {
            //method get() returns a reference to the element at that position 
            // or None if out of bounds;
            // and_then() calls its function input (= Option) 
            // with the wrapped value and returns the result.
            let anfield_cell = anfield.get(y + i).and_then(|r| r.chars().nth(x + j));
            if piece_cell == '0' {
                // Check if the piece cell is on a valid position 
                // (within bounds, not overlapping opponent territory)
                if let Some(a) = anfield_cell {
                    if
                        a != symbols.opponent_recent_symbol &&
                        a != symbols.opponent_territory_symbol
                    {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    // Out of Anfield bounds
                    return false;
                }
            }
        }
    }

    true
}
*/
/*
use std::io::{self, BufRead, Write};
use std::collections::VecDeque;
use filler_lib::{Tokens, expand_right, check_right, expand_left, check_left, find_opponent};

fn main() {
    let stdin = io::stdin();
    let mut lock = io::stdout().lock();

    // Read my player number
    let mut me_info = String::new();
    stdin.lock().read_line(&mut me_info).unwrap();

    // Extract player number
    let me: usize = me_info
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

    // Read foe info
    let mut foe_info = String::new();
    stdin.lock().read_line(&mut foe_info).unwrap();

    // Extract foe number (not used in this example)
    let _foe = foe_info
        .split_whitespace()
        .nth(3)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    // Continuously read Anfield and piece data and print the coordinates of the next move
    loop {
        // Read the Anfield dimensions and state
        let mut anfield_info = String::new();
        stdin.lock().read_line(&mut anfield_info).unwrap();
        let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
        let rows: usize = dimensions[2].trim_end_matches(':').parse().unwrap();

        let mut anfield = Vec::new();
        for _ in 0..rows {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            let off_with_head: Vec<&str> = line.split_whitespace().collect();
            if off_with_head.len() > 1 {
                line = off_with_head[1].to_string();
            }
            anfield.push(line.trim().to_string());
        }

        // Remove the slice at index '0' because we don't need column headings
        anfield.remove(0);

        // Read the piece dimensions and shape
        let mut piece_info = String::new();
        stdin.lock().read_line(&mut piece_info).unwrap();
        let piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
        let piece_height: usize = piece_dimensions[2].trim_end_matches(':').parse().unwrap();
        let mut piece = Vec::new();
        for _ in 0..piece_height {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            piece.push(line.trim().to_string());
        }

        // Determine the next move
        let (right_x, right_y) = expand_right(&anfield, &tokens);
        let available_right = check_right(&anfield, &piece, &tokens);
        let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
        let available_left = check_left(&anfield, &piece, &tokens);
        let (foe_x, foe_y) = find_opponent(&anfield, &tokens);

        let (mut x, mut y): (usize, usize) = (0, 0);

        if available_right {
            (x, y) = (right_x, right_y);
        } else if available_left {
            (x, y) = (left_x, left_y);
        } else {
            // Kill the game
            (x, y) = (foe_x, foe_y);
        }

        // Debugging statement to verify output
        eprintln!("Outputting coordinates: {} {}", x, y);

        // Output the coordinates to the game engine
        writeln!(lock, "{} {}", x, y).unwrap();
        lock.flush().unwrap();
    }
}
*/