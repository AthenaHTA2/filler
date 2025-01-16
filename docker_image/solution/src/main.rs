//main.rs is the entry point of the program

use filler_lib::{check_left, check_right, expand_left, expand_right, find_opponent, Tokens};
use std::collections::VecDeque;
use std::io::{self, BufRead, Read, Write};

//Import the Enigo trait used for simulating the Enter key press
use enigo::{
    Direction::Click,
    Enigo, Key, Keyboard, Settings,
};



//==============================================================================
//START OF VERSION 1: Code that takes in all input at once but returns EOF error
//==============================================================================
fn main() {
    let stdin = io::stdin();  
    //Test to see the actual game engine input
    // Read all input lines at once
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    //The println! macro will lock the standard output on each call.
    //If you call println! within a hot loop, this behavior may be the bottleneck of the loop.
    //To avoid this, lock stdout with io::stdout().lock():
    let stdout = io::stdout();

    // Read player information
    // that is sent just once at the start
    let p1_info = lines.next().unwrap();

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
    
    //read player 2 info
    let p2_info = lines.next().unwrap();
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

    let mut me: usize = p2;

    //detect which player I am
    if p1_info == "$$$ exec p1 : [target/release/filler]\n" {
        me = p1;
    } else {
        //me = p2;
    }

    //println!("me:{:?}", me);

    let tokens = Tokens::new(me);

    // Continuously read Anfield and piece data and print the coordinates of the next move
    loop {
        // Read the Anfield dimensions and state
        //e.g.: Anfield 20 15:
        let anfield_info = lines.next().unwrap();
        let dimensions: Vec<&str> = anfield_info
        .split_whitespace()
        .collect();
        //println!("dimensions:{:?}", dimensions);
        // Remove the colon at the end and parse the number of rows
        let rows: usize = dimensions[2]
        .trim_end_matches(':')
        .parse()
        .unwrap();
        //not used: let cols: usize = dimensions[2].parse().unwrap();
        //I am using VecDeque because I need to remove the first element
        //and the VecDeque has the pop_front() method

        let mut anfield = VecDeque::new();
        //I need 0 to 15 which gives 16 rows
        //note that 0..rows yields values from 0 (inclusive) to rows (exclusive)
        //I add 1 to account for column headings which which are later removed
        for _ in 0..rows + 1 {
            let line = lines.next().unwrap();
            //remove row number and space at the beginning
            let off_with_head: Vec<&str> = line.split_whitespace().collect();
            if off_with_head.len() > 1 {
                anfield.push_back(off_with_head[1].trim().to_string());
            } else {
                anfield.push_back(line.trim().to_string());
            }
        }
        //now remove the slice at index '0'
        //because don't need column headings
        anfield.pop_front();
        //println!("anfield: {:?}", anfield); 

        // Read the piece dimensions and shape
        let piece_info = lines.next().unwrap();
        let piece_dimensions: Vec<&str> = piece_info
        .split_whitespace()
        .collect();
        //println!("piece_dimensions:{:?}", piece_dimensions);
        //not used: let piece_width: usize = piece_dimensions[1].parse().unwrap();
        //remove the colon at the end and parse the number of rows
        let piece_height: usize = piece_dimensions[2]
        .trim_end_matches(':')
        .parse()
        .unwrap();
        //println!("piece_height:{}", piece_height);

        //My code stops at this point and completes after I press the Enter key
        let mut piece = Vec::new();
        //below loop is equivalent to line below:
        //let piece: VecDeque<String> = (0..piece_height).map(|_| lines.next().unwrap()).collect();
        // Create an instance of Enigo
        //let mut enigo = Enigo::new(&Settings::default()).unwrap();
        
        for i in 0..piece_height {
            
            if let Some(piece_line) = lines.next() {
                piece.push(piece_line.trim().to_string());
               // if i == piece_height - 1 {
                    //piece.push(piece_line.trim().to_string());
                 //   println!("inside piece height: {} {:?}", i, piece);
                    
                    //does not work: simulating pressing the Enter key
                    //let _ = Keyboard::key(/* value */,Key::Return, Click);
                    //let _ = enigo.key(Key::Return, Click);
                    // break
               // } else if i < piece_height - 1 {
                    
                //    piece.push(piece_line.trim().to_string());
                 //   println!("No more input or error reading at line {}", i);
                    //break;
                //}
            } else{
                //println!("inside piece None at line: {}", i);
                //let _ = enigo.key(Key::Return, Click);
                break;
            }
        }
        
        //println!("out of piece loop:{:?}", piece);

        //eprintln!("piece:{:?}", piece);
        // Determine the next move
        // Simple approach: place the piece so its first cell
        // will sit on my previous piece's last cell, if possible
        let (right_x, right_y) = expand_right(&anfield, &tokens);
        //println!("right_x:{} right_y:{}", right_x, right_y);
        //check if last character to the right can be used as anchor for next piece
        let available_right = check_right(&anfield, &piece, &tokens);
        //println!("available_right:{}", available_right);
        let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
        //println!("left_x:{} left_y:{}", left_x, left_y);
        let available_left = check_left(&anfield, &piece, &tokens);
        //println!("available_left:{}", available_left);
        // Find the opponent's last cell
        let (foe_x, foe_y) = find_opponent(&anfield, &tokens);
        //println!("foe_x:{} foe_y:{}", foe_x, foe_y);
        let (mut x, mut y): (usize, usize) = (foe_x, foe_y);

        if available_right {
            (x, y) = (right_x, right_y);
        } else if available_left {
            (x, y) = (left_x, left_y);
        } else {
            //kill the game
            //(x, y) = (foe_x, foe_y);
        };

        // Debugging statement to verify output
        //eprintln!("Outputting coordinates: {} {}", x, y);

        let mut lock_out = stdout.lock();
        write!(lock_out, "{} {}\n", x, y).unwrap();
        lock_out.flush().unwrap();
    } //end of loop
  
}



//============================================================================
//END OF VERSION 1: Code that takes in all input at once but returns EOF error
//============================================================================

/*
//======================================================================
//START OF VERSION 2: Code that takes in all input at once but times out
//======================================================================
//No error but code does not print nor complete
fn main() {
    let mut input = String::new();
    //println!("Reading input...");
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();
    //println!("Input read successfully");
    let stdout = io::stdout();

    // Read player information
    // that is sent just once at the start
    let p1_info = lines.next().expect("Failed to read player 1 info");
    //println!("p1_info: {}", p1_info);
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
        .expect("Failed to convert player 1 number to digit") as usize;

    //println!("p1: {}", p1);
    
    //I am removing player 2 info in case game engine does not send it
    //read player 2 info
    let p2_info = lines.next().unwrap();
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

    let mut me: usize = p2;

    //detect which player I am
    if p1_info == "$$$ exec p1 : [target/release/filler]\n" {
        me = p1;
    } else {
        //me = p2;
    }
    //println!("me:{:?}",me);
    
    let tokens = Tokens::new(me);
    
    //let tokens = Tokens::new(p1);

    loop {
        //println!("Reading anfield info...");
        let anfield_info = lines.next().expect("Failed to read anfield info");
        //println!("anfield_info: {}", anfield_info);
        let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
        // Remove the colon at the end and parse the number of rows
        let rows: usize = dimensions[2].trim_end_matches(':').parse().expect("Failed to parse rows");
        //println!("rows: {}", rows);
        //I am using VecDeque because I need to remove the first element
        //and the VecDeque has the pop_front() method

        let mut anfield = VecDeque::new();
        //I need 0 to 15 which gives 16 rows
        //note that 0..rows yields values from 0 (inclusive) to rows (exclusive)
        //I add 1 to account for column headings which which are later removed
        for _ in 0..rows + 1 {
            let line = lines.next().expect("Failed to read anfield line");
            //remove row number and space at the beginning
            let off_with_head: Vec<&str> = line.split_whitespace().collect();
            if off_with_head.len() > 1 {
                anfield.push_back(off_with_head[1].trim().to_string());
            } else {
                anfield.push_back(line.trim().to_string());
            }
        }
        //now remove the slice at index '0'
        //because don't need column headings
        anfield.pop_front();
        //println!("anfield: {:?}", anfield);

        // Read the piece dimensions and shape
        //println!("Reading piece info...");
        let piece_info = lines.next().expect("Failed to read piece info");
        let piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();

        //remove the colon at the end and parse the number of rows
        let piece_height: usize = piece_dimensions[2].trim_end_matches(':').parse().expect("Failed to parse piece height");
        //println!("piece_height:{}", piece_height);

        let mut piece = Vec::new();
        //below loop is equivalent to line below:
        //let piece: VecDeque<String> = (0..piece_height).map(|_| lines.next().unwrap()).collect();
        // Create an instance of Enigo
        //let mut enigo = Enigo::new(&Settings::default()).unwrap();
        
        for _i in 0..piece_height {        
            if let Some(piece_line) = lines.next() {
                    piece.push(piece_line.trim().to_string());
                    println!("piece_line: {}", piece_line);
                } else {
                    println!("no more piece lines");
                }
        }
        //println!("piece: {:?}", piece);

        // Determine the next move
        // Simple approach: place the piece so its first cell
        // will sit on my previous piece's last cell, if possible
        let (right_x, right_y) = expand_right(&anfield, &tokens);
        //println!("right_x:{} right_y:{}", right_x, right_y);
        //check if last character to the right can be used as anchor for next piece
        let available_right = check_right(&anfield, &piece, &tokens);
        //println!("available_right:{}", available_right);
        let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
        //println!("left_x:{} left_y:{}", left_x, left_y);
        let available_left = check_left(&anfield, &piece, &tokens);
        //println!("available_left: {}", available_left);
        // Find the opponent's last cell
        let (foe_x, foe_y) = find_opponent(&anfield, &tokens);
        //println!("foe_x: {} foe_y: {}", foe_x, foe_y);
        let (mut x, mut y): (usize, usize) = (foe_x, foe_y);

        if available_right {
            (x, y) = (right_x, right_y);
        } else if available_left {
            (x, y) = (left_x, left_y);
        }

        let mut lock_out = stdout.lock();
        write!(lock_out, "{} {}\n", x, y).expect("Failed to write output");
        lock_out.flush().expect("Failed to flush output");
    }
}


//====================================================================
//END OF VERSION 2: Code that takes in all input at once but times out
//====================================================================
*/
//End of my code that takes input at once





//Below is prior version of my code, that takes input line by line.
//First, have changed so it uses read instead of read_line
//This did not work as I still need to press enter to make my code run.
//Also added a channel to simulate the "Enter" key press, which did not work either.
/*
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

         //changing approach to gathering terminal input:
        //now I read all information at once
        // Read all input lines at once
        /*
        let input: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();
        println!("input all at once: {:?}", input);
        */
        /*
        // Create a channel to signal when to simulate the "Enter" key press
        let (tx, rx) = mpsc::channel::<()>();

        // Spawn a thread to simulate the "Enter" key press when signaled
        thread::spawn(move || {
            // Wait for the signal to simulate the "Enter" key press
            rx.recv().unwrap();
            let mut stdout = io::stdout();
            let mut handle = stdout.lock();
            //writeln!(handle, "ENTER").unwrap();
            writeln!(handle, "{} {}", x, y).unwrap();
        });
       */
    /*
        // Simulate an "Enter" key press after a delay
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(2));
            let mut stdout = io::stdout();
            let mut handle = stdout.lock();
            writeln!(handle, "").unwrap();
        });
    */
        // Read player 1 information
        let mut p1_info = String::new();
        stdin.lock().read_line(&mut p1_info).unwrap();
        //stdin.read_line(&me_info).unwrap();
        //println!("p1_info:{:?}", p1_info);
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

        //println!("p1:{:?}", p1);
        // Assign symbols to self and opponent

        //read player 2 info
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

        //println!("p2:{:?}", p2);

        let mut me: usize = 0;

        //detect which player I am
        if p1_info == "$$$ exec p1 : [target/release/filler]\n"{
            me = p1;
        } else {
            me = p2;
        }

        println!("me:{:?}", me);

        let tokens = Tokens::new(me);

        //continuously read Anfield and piece data
        //and print the coordinates of the next move

        loop {

            let mut buffer = [0; 256];

            // Read the Anfield dimensions and state
            //e.g.: Anfield 20 15:
            let mut anfield_info = String::new();
            //stdin.lock().read_line(&mut anfield_info).unwrap();
            let mut bytes_read = stdin.lock().read(&mut buffer).unwrap();
            let anfield_info = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
            println!("dimensions:{:?}", dimensions);
            // Remove the colon at the end and parse the number of rows
            let rows: usize = dimensions[2].trim_end_matches(':').parse().unwrap();
            //let anf_rows = dimensions[2].trim_end_matches(':');
            //println!("anf_rows:{}", anf_rows);
            //let rows: usize = anf_rows.parse().unwrap();
            //println!("rows:{}", rows);
            //not used: let cols: usize = dimensions[2].parse().unwrap();
            //I am using VecDeque because I need to remove the first element
            //and the VecDeque has the pop_front() method

            let mut anfield = VecDeque::new();
            //I need 0 to 15 which gives 16 rows
            //note that 0..rows yields values from 0 (inclusive) to rows (exclusive)
            //I add 1 to account for column headings which which are later removed
            for _ in 0..rows + 1 {
            let mut line = String::new();
            //stdin.lock().read_line(&mut line).unwrap();
            bytes_read = stdin.lock().read(&mut buffer).unwrap();
            line = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            //remove row number and space at the beginning
            let off_with_head: Vec<&str> = line.split_whitespace().collect();
            if off_with_head.len() > 1 {
            line = off_with_head[1].to_string();
            anfield.push_back(line.trim().to_string());
            } else {
            anfield.push_back(line.trim().to_string());
            }
        }
            //println!("anfield after removing row headers:{:?}", anfield);
            //now remove the slice at index '0'
            //because don't need column headings
            anfield.pop_front();
            println!("anfield after removing column headers:{:?}", anfield);
            //This is the last Anfield row that is stored twice and I don't need it

            //let mut extra_line = String::new();
            //let _throw_away = stdin.lock().read_line(&mut extra_line).unwrap();

            // Read the piece dimensions and shape
            /*
            let mut piece_info = String::new();
            stdin.lock().read_line(&mut piece_info).unwrap();
            */
            //let mut buffer = [0; 256];
            bytes_read = stdin.lock().read(&mut buffer).unwrap();
            let piece_info = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            let mut piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
            //println!("piece_dimensions:{:?}", piece_dimensions);

            //not used: let piece_width: usize = piece_dimensions[1].parse().unwrap();
            //remove the colon at the end and parse the number of rows
            let mut piece_height: usize =  piece_dimensions[2].trim_end_matches(':').parse().unwrap();
            //println!("piece_height:{}", piece_height);

            // Signal the thread to simulate the "Enter" key press
            //tx.send(()).unwrap();

            let mut piece = Vec::new();
            //looping from 0 up to excluding height
            //let mut count = 0;
            //println!("piece vector live");
            /*
            for i in 0..piece_height{
                //count += 1;
                //print!("storing line: {}", count);
                let mut piece_line = String::new();
                let bytes_read = stdin.lock().read_line(&mut piece_line).unwrap();
                if bytes_read == 0 && i == piece_height - 1 {
                    // If it's the last line and no newline character is found, append one
                    piece_line.push('\n');
                }
                //piece gets printed on a new line. Why?
                //print!("piece_line:{}", piece_line);
                //println!("piece before push: {:?}", piece);
                piece.push(piece_line.trim().to_string());//last iteration does not run
                //print!("piece after push: {:?}", piece);//last iteration does not run
            }
            */
            //changing input reading approach
            //to avoid program freezing due to lack of
            //newline character at the end of the last line
            for _ in 0..piece_height {
                let mut buffer = [0; 256];
                let bytes_read = stdin.lock().read(&mut buffer).unwrap();
                let piece_line = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                piece.push(piece_line.trim().to_string());
            }

            eprintln!("piece:{:?}", piece);

            //if !piece.is_empty() {
                //println!("piece length:{}", piece[0].len());
            //}
            // Determine the next move
            // Simple approach: place the piece so its first cell
            // will sit on my previous piece's last cell, if possible
            let (right_x, right_y) = expand_right(&anfield, &tokens);
            //println!("right_x:{} right_y:{}", right_x, right_y);
            //check if last character to the right can be used as anchor for next piece
            let available_right = check_right(&anfield, &piece, &tokens);
            //println!("available_right:{}", available_right);
            let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
            //println!("left_x:{} left_y:{}", left_x, left_y);
            let available_left = check_left(&anfield, &piece, &tokens);
            //println!("available_left:{}", available_left);
            // Find the opponent's last cell
            let(foe_x, foe_y) = find_opponent(&anfield, &tokens);
            //println!("foe_x:{} foe_y:{}", foe_x, foe_y);
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
            writeln!(lok, " {} {}", x, y).unwrap();
            lok.flush().unwrap();


        }//end of loop
    }

    */

/*
//this is my original code that reads input line-by-line.
//It stops until I press Enter
fn main() {


    let stdin = io::stdin();

    //The println! macro will lock the standard output on each call.
    //If you call println! within a hot loop, this behavior may be the bottleneck of the loop.
    //To avoid this, lock stdout with io::stdout().lock():
    let mut stdout = io::stdout();
    //let mut stdou = stdout.lock();

    //changing approach to gathering terminal input:
    //now I read all information at once
    // Read all input lines at once
    /*
    let input: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    println!("input all at once: {:?}", input);
    */
    /*
    // Create a channel to signal when to simulate the "Enter" key press
    let (tx, rx) = mpsc::channel::<()>();

    // Spawn a thread to simulate the "Enter" key press when signaled
    thread::spawn(move || {
        // Wait for the signal to simulate the "Enter" key press
        rx.recv().unwrap();
        let mut stdout = io::stdout();
        let mut handle = stdout.lock();
        //writeln!(handle, "ENTER").unwrap();
        writeln!(handle, "{} {}", x, y).unwrap();
    });
   */
/*
    // Simulate an "Enter" key press after a delay
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        let mut stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "").unwrap();
    });
*/
    // Read player 1 information
    let mut p1_info = String::new();
    stdin.lock().read_line(&mut p1_info).unwrap();
    //stdin.read_line(&me_info).unwrap();
    //println!("p1_info:{:?}", p1_info);
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

    //println!("p1:{:?}", p1);
    // Assign symbols to self and opponent

    //read player 2 info
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

    //println!("p2:{:?}", p2);

    let mut me: usize = 0;

    //detect which player I am
    if p1_info == "$$$ exec p1 : [target/release/filler]\n"{
        me = p1;
    } else {
        me = p2;
    }

    //println!("me:{:?}", me);

    let tokens = Tokens::new(me);

    //continuously read Anfield and piece data
    //and print the coordinates of the next move

    loop {

        // Read the Anfield dimensions and state
        //e.g.: Anfield 20 15:
        let mut anfield_info = String::new();
        stdin.lock().read_line(&mut anfield_info).unwrap();
        let dimensions: Vec<&str> = anfield_info.split_whitespace().collect();
        //println!("dimensions:{:?}", dimensions);
        // Remove the colon at the end and parse the number of rows
        let rows: usize = dimensions[2].trim_end_matches(':').parse().unwrap();
        //let anf_rows = dimensions[2].trim_end_matches(':');
        //println!("anf_rows:{}", anf_rows);
        //let rows: usize = anf_rows.parse().unwrap();
        //println!("rows:{}", rows);
        //not used: let cols: usize = dimensions[2].parse().unwrap();
        //I am using VecDeque because I need to remove the first element
        //and the VecDeque has the pop_front() method

        let mut anfield = VecDeque::new();
        //I need 0 to 15 which gives 16 rows
        //note that 0..rows yields values from 0 (inclusive) to rows (exclusive)
        //I add 1 to account for column headings which which are later removed
        for _ in 0..rows + 1 {
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
        //println!("anfield after removing row headers:{:?}", anfield);
        //now remove the slice at index '0'
        //because don't need column headings
        anfield.pop_front();
        //println!("anfield after removing column headers:{:?}", anfield);
        //This is the last Anfield row that is stored twice and I don't need it

        //let mut extra_line = String::new();
        //let _throw_away = stdin.lock().read_line(&mut extra_line).unwrap();

        // Read the piece dimensions and shape
        /*
        let mut piece_info = String::new();
        stdin.lock().read_line(&mut piece_info).unwrap();
        */
        let mut buffer = [0; 256];
            let bytes_read = stdin.lock().read(&mut buffer).unwrap();
            let piece_info = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        let mut piece_dimensions: Vec<&str> = piece_info.split_whitespace().collect();
        //println!("piece_dimensions:{:?}", piece_dimensions);

        //not used: let piece_width: usize = piece_dimensions[1].parse().unwrap();
        //remove the colon at the end and parse the number of rows
        let mut piece_height: usize =  piece_dimensions[2].trim_end_matches(':').parse().unwrap();
        //println!("piece_height:{}", piece_height);

        // Signal the thread to simulate the "Enter" key press
        //tx.send(()).unwrap();

        let mut piece = Vec::new();
        //looping from 0 up to excluding height
        //let mut count = 0;
        //println!("piece vector live");
        /*
        for i in 0..piece_height{
            //count += 1;
            //print!("storing line: {}", count);
            let mut piece_line = String::new();
            let bytes_read = stdin.lock().read_line(&mut piece_line).unwrap();
            if bytes_read == 0 && i == piece_height - 1 {
                // If it's the last line and no newline character is found, append one
                piece_line.push('\n');
            }
            //piece gets printed on a new line. Why?
            //print!("piece_line:{}", piece_line);
            //println!("piece before push: {:?}", piece);
            piece.push(piece_line.trim().to_string());//last iteration does not run
            //print!("piece after push: {:?}", piece);//last iteration does not run
        }
        */
        //changing input reading approach
        //to avoid program freezing due to lack of
        //newline character at the end of the last line
        for _ in 0..piece_height {
            let mut buffer = [0; 256];
            let bytes_read = stdin.lock().read(&mut buffer).unwrap();
            let piece_line = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            piece.push(piece_line.trim().to_string());
        }

        //eprintln!("piece:{:?}", piece);

        //if !piece.is_empty() {
            //println!("piece length:{}", piece[0].len());
        //}
        // Determine the next move
        // Simple approach: place the piece so its first cell
        // will sit on my previous piece's last cell, if possible
        let (right_x, right_y) = expand_right(&anfield, &tokens);
        //println!("right_x:{} right_y:{}", right_x, right_y);
        //check if last character to the right can be used as anchor for next piece
        let available_right = check_right(&anfield, &piece, &tokens);
        //println!("available_right:{}", available_right);
        let (left_x, left_y) = expand_left(&anfield, &piece, &tokens);
        //println!("left_x:{} left_y:{}", left_x, left_y);
        let available_left = check_left(&anfield, &piece, &tokens);
        //println!("available_left:{}", available_left);
        // Find the opponent's last cell
        let(foe_x, foe_y) = find_opponent(&anfield, &tokens);
        //println!("foe_x:{} foe_y:{}", foe_x, foe_y);
        let (mut x, mut y): (usize, usize) = (0, 0);

        if available_right {
            (x, y) = (right_x, right_y);
        } else if available_left {
            (x, y) = (left_x, left_y);
        }else{
            //kill the game
            (x, y) = (foe_x, foe_y);
        };

        //println!("my move is: {} {}", x, y);
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
        writeln!(lok, " {} {}", x, y).unwrap();
        lok.flush().unwrap();


    }//end of loop
}
//this is the original code that stops until I press Enter
*/
