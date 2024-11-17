//lib.rs handles the logic of the program

//use std::io::{self, Write};
//use std::env;
use::std::collections::VecDeque;

pub struct Tokens {
    pub my_last: char,
    pub my_territory: char,
    pub opponent_last: char,
    pub opponent_territory: char,
    pub anfield_empty: char,
}

impl Tokens {
    pub fn new(my_number: usize) -> Self {
        if my_number == 1 {
            Tokens {
                my_last: 'a',
                my_territory: '@',
                opponent_last: 's',
                opponent_territory: '$',
                anfield_empty: '.',
            }
        } else {
            Tokens {
                my_last: 's',
                my_territory: '$',
                opponent_last: 'a',
                opponent_territory: '@',
                anfield_empty: '.',
            }
        }
    }
}

//get coordinates of my last piece's final character
//in order to put the new piece to the right of it
pub fn expand_right(
    anfield: &VecDeque<String>,
    tokens: &Tokens,
) -> (usize, usize) {

    // Get my symbol's last character's row
    //to remove the warning message use let mut y: Option<usize> = None;
    let mut y: usize = 0;
    let mut find_symbol = tokens.my_territory;

    //Anfield contains only one my_territory character when the game starts
    //Hence need to check if my_last character is present in the Anfield
    let contains_my_last = anfield.iter().any(|line| line.contains(tokens.my_last));
    if contains_my_last == false {
        //find_symbol = tokens.my_territory;
    }else{
        find_symbol = tokens.my_last;
    }
    for (i, line) in anfield.iter().enumerate() {
        if line.contains(&format!("{}{}", find_symbol, tokens.anfield_empty)) {
            // The line contains '.' followed by '@' or 'a'     
            //y = Some(i);
            y = i;
            break;
        }
    }

    // Get find_symbol's last character's column
    let mut x = 0;
    for (i, ch) in anfield[y].chars().enumerate() {
        if ch == find_symbol && 
        i + 1 < anfield[y].len() && 
        anfield[y].chars().nth(i + 1) == Some(tokens.anfield_empty) {
            x = i;
            break;
        }
    }

(x, y)

}

//check if last character can be used as anchor for next piece
pub fn check_right(
    anfield: &VecDeque<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> bool {

    // Find the last cell of my previous piece
    let (last_x, last_y) = expand_right(&anfield, &tokens);

    // Determine if there is sufficient space to place the piece
    for x in last_x + 1..=last_x + piece[0].len() - 1 {
        for y in last_y..=last_y + piece.len() - 1 {
            if x >= anfield[0].len() || y >= anfield.len() || anfield[y].chars().nth(x) != Some(tokens.anfield_empty) {
                return false;
            }
        }
    }

    true
}

//get coordinates to anchor the new piece
//to the left of my territory or my last piece
pub fn expand_left(
    anfield: &VecDeque<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> (usize, usize) {

    // Find row that contains anfield_empty and my_territory characters, e.g. '.@'
    let mut y: usize = 0;
    let mut find_symbol = tokens.my_territory;

    //Anfield contains only one my_territory character when the game starts
    //Hence need to check if my_last character is present in the Anfield
    let contains_my_last = anfield.iter().any(|line| line.contains(tokens.my_last));
    if contains_my_last == false {
        //find_symbol = tokens.my_territory;
    }else{
        find_symbol = tokens.my_last;
    }
    for (i, line) in anfield.iter().enumerate() {
        if line.contains(&format!("{}{}", tokens.anfield_empty, find_symbol)) {
            // The line contains '.' followed by '@' or 'a'      
            y = i;
            break;
        }
    }

    // Find column so that new piece can overlap one symbol '@' or 'a'
    let mut x = 0;
    for (i, ch) in anfield[y].chars().enumerate() {
        if ch == find_symbol &&
        i >= piece[0].len() &&
        anfield[y].chars().nth(i - piece[0].len() -1) == Some(tokens.anfield_empty) {
            x = i - piece[0].len() -1;
            break;
        }
    }


(x, y)

}

//check if cell at the left of my territory 
//can be used as anchor for my next piece
pub fn check_left(
    anfield: &VecDeque<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> bool {

    // Find coordinates for my next piece
    let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);

    //determine if there is sufficient space to place piece
    for x in left_x - 1..=left_x - piece[0].len()-1 {
        for y in left_y..=left_y + piece.len()-1 {
            if x <= 0 || y >= anfield.len() || anfield[y].chars().nth(x) != Some(tokens.anfield_empty) {
                return false;
            }
        }
    }

    true

}

pub fn find_opponent(
    anfield: &VecDeque<String>,
    tokens: &Tokens,
) -> (usize, usize) {

    // Find opponent's last character's row
    let mut y: usize = 0;
    let mut x: usize = 0;
    let mut y_a: usize = 0;
    let mut y_b: usize = 0;
    let mut y_me: usize = 0;

    let mut opponent_symbol = tokens.opponent_territory;
    // Check if Anfield contains opponent's last character
    let contains_opponent_last = anfield.iter().any(|line| line.contains(tokens.opponent_last));
    if contains_opponent_last == false {
        // opponent_symbol = tokens.opponent_territory;
    } else {
        opponent_symbol = tokens.opponent_last;
    }

    let mut my_symbol = tokens.my_territory;
    // Check if Anfield contains my_last character
    let contains_my_last = anfield.iter().any(|line| line.contains(tokens.my_last));
    if contains_my_last == false {
        // my_symbol = tokens.my_territory;
    } else {
        my_symbol = tokens.my_last;
    }

    for (i, line) in anfield.iter().enumerate() {
        if line.contains(&format!("{}{}", opponent_symbol, tokens.anfield_empty)) {
        // The line contains opponent symbol followed by '.'    
            y_a = i;
        }
        if line.contains(&format!("{}{}", tokens.anfield_empty, opponent_symbol)) {
            // The line contains '.' followed by opponent symbol
            y_b = i;
        }
        if line.contains(&format!("{}{}", my_symbol, tokens.anfield_empty)) {
            // The line contains my symbol followed by '.'
            //to do: I could check if Anfield contains '.' followed by my symbol
            
            y_me = i;
        }
    }

    // Find the row where the opponent is closest to my last character
    
    if (y_me as isize - y_a as isize).abs() < (y_me as isize - y_b as isize).abs() {
        y = y_a;
        for (i, ch) in anfield[y].chars().enumerate() {
            if ch == opponent_symbol &&
               i + 1 < anfield[y].len() &&
               anfield[y].chars().nth(i + 1) == Some(tokens.anfield_empty) {
                x = i;
                break;
            }
        }
    } else {
        y = y_b;
        for (i, ch) in anfield[y].chars().enumerate() {
            if ch == opponent_symbol &&
               i >= 1 &&
               anfield[y].chars().nth(i - 1) == Some(tokens.anfield_empty) {
                x = i;
                break;
            }
        }
    }
    
    (x, y)
}

    
