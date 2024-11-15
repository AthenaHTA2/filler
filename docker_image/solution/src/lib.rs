//lib.rs handles the logic of the program

use std::io::{self, Write};
use std::env;

pub struct Tokens {
    pub my_last: char,
    pub my_terrytory: char,
    pub opponent_last: char,
    pub opponent_terrytory: char,
}

pub impl Tokens {
    pub fn new(my_number: usize) -> Self {
        if my_number == 1 {
            Tokens {
                my_last: 'a',
                my_terrytory: '@',
                opponent_last: 's',
                opponent_terrytory: '$',
                anfield_empty: '.',
            }
        } else {
            Tokens {
                my_last: 's',
                my_terrytory: '$',
                opponent_last: 'a',
                opponent_terrytory: '@',
                anfield_empty: '.',
            }
        }
    }
}

//get coordinates of my last piece's final character
//in order to put the new piece to the right of it
pub fn expand_right(
    anfield: &Vec<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> (usize, usize) {

    // Find piece 'a' last character's row
    let mut y usize = 0;
    for (i, line) in anfield.iter().enumerate() {
        if line.contains(&format!("{}{}", tokens.my_last, tokens.anfield_empty)) {
            // The line contains '.' followed by '@'       
            y = i;
            break;
        }
    }

    // Find piece 'a' last character's column
    let mut x = 0;
    for (i, ch) in anfield[row].chars().enumerate() {
        if ch == tokens.my_last && i + 1 < anfield[row].len() && anfield[row].chars().nth(i + 1) == Some(tokens.anfield_empty) {
            x = i;
            break;
        }
    }

(x, y)

}

//check if last character can be used as anchor for next piece
pub fn check_right(
    anfield: &Vec<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> bool {

    // Find the last cell of my previous piece
    let (last_x, last_y) = expand_right(&anfield, &piece, &tokens);

    //determine if there is sufficient space to place piece
    (last_x + 1..=last_x + piece[0].len()-1).for_each(|x| {
        (last_y..=last_y + piece.len()-1).for_each(|y| {
            if anfield[y][x] == tokens.anfield_empty &&
            x < anfield[0].len() && y < anfield.len() {
                continue;
            }else{
                return false;
            }
        });
    });

    return true;

}

//get coordinates to anchor the new piece
//to the left of my territory
pub fn expand_left(
    anfield: &Vec<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> (usize, usize) {

    // Find row that contains anfield_empty and my_terrytory characters, e.g. '.@'
    let mut y: usize = 0;
    for (i, line) in anfield.iter().enumerate() {
        if line.contains(&format!("{}{}", tokens.anfield_empty, tokens.my_territory)) {
            // The line contains '.' followed by '@'       
            y = i;
            break;
        }
    }

    // Find column for a symbol '@' that is preceeded by symbol '.'
    let mut x = 0;
    for (i, ch) in anfield[y].chars().enumerate() {
        if ch == tokens.my_territory &&
        i >= piece[0].len() &&
        anfield[y].chars().nth(i - piece[0].len() -1) == Some(tokens.anfield_empty) {
            x = i - piece[0].len() -1;
            break;
        }
    }


(x, y)

}

//check if my territory can be used as anchor for next piece
pub fn check_left(
    anfield: &Vec<String>,
    piece: &Vec<String>,
    tokens: &Tokens,
) -> bool {

    // Find the last cell of my previous piece
    let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);

    //determine if there is sufficient space to place piece
    (left_x..=left_x + piece[0].len()-1).for_each(|x| {
        (left_y..=left_y + piece.len()-1).for_each(|y| {
            if anfield[y][x] == tokens.anfield_empty &&
            x >= 0 && y < anfield.len() {
                continue;
            }else{
                return false;
            }
        });
    });

    return true;

}
