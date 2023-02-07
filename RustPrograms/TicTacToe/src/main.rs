use std::io;
use std::process;

/* 
    This takes matrix (defined in main) and prints it out. 
    It prints each row in a new line 
*/

fn print_board(arr: [[&str; 3]; 3]) {

    //prints column indexes
    println!("\n  1 2 3 <- col");
    let mut r = 1;
    //loops through the matrix
    for row in arr {
        //loop through the line, adding row index
        print!("{} ", r);
        for item in row {
            //print each value
                print!("{item} ");
        }
        println!();
        r = r + 1;
    }
    println!("^\nrow\n");
}

/* 
    This requests a value from the user and checks
    if it is between 1 & 3 (inclusive) and is an integer,
    then returns the value.
*/

fn request_user_value() -> u8{
    //if the user value does not pass conditions, loop code until the user enters the right value
    loop {
        //creates input variable
        let mut value = String::new();

        //takes in input
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");
        
        //converts the value to a u8
        let value: u8 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //Check if the user input "exit" (this means they want to exit the program)
                if value == "exit" {
                    process::exit(0);
                }

                //if it's a different thing, tell user to input a number
                print!("\nPlease enter a number: ");
                continue;
            }
        };

        //checks if the value is between 1 & 3 (inclusive), gives error message if it isn't
        if (value < 1) || (value > 3){
            println!("please input a number between 1 and 3\n");
            continue;
        }

        return value
    }
}

/* 
    Checks if one character occurs 
    in every space in any row. 
    Returns that character, 
    or * if none are found 
*/

fn check_row_victory(arr: [[&str; 3]; 3]) -> &str {
   
    //loops through rows
    for row in arr{
   
        //check if all values in row are equal to a certain character. 
        //we make sure that the value is either "ඞ" or "·", since "*" is just a character we use to denote an empty tile.
        if row[0] == row[1] && row[0] == row[2] && row[0] != "*"{
            return row[0];
        }
    }
   
    //if neither "ඞ" or "·" satisfy our conditions, return "*"
    return "*";
}

/* 
    Checks if one character occurs 
    in every space in any col. 
    Returns that character, 
    or * if none are found. 
*/

fn check_col_victory(arr: [[&str; 3]; 3]) -> &str {
   
    //loops through cols
    for i in 0..3{
   
        //check if all values in col are equal to a certain character. 
        //we make sure that the value is either "ඞ" or "·", since "*" is just a character we use to denote an empty tile.
        if arr[0][i] == arr[1][i] && arr[0][i] == arr[2][i] && arr[0][i] != "*"{
            return arr[0][i];
        }
    }
   
    //if neither "ඞ" or "·" satisfy our conditions, return "*"
    return "*";
}

/*
    Checks if one character occurs
    in every space in either diagonal 
    (top left -> bottom right) or
    (bottom left -> top right).
    Returns that character, 
    or * if none are found. 
*/

fn check_diag_victory(arr: [[&str; 3]; 3]) -> &str {
   
    //loops through diagonals (we manually check with if statements instead of looping)
    //we make sure that the value is either "ඞ" or "·", since "*" is just a character we use to denote an empty tile.
    if arr[0][0] == arr[1][1] && arr[0][0] == arr[2][2] && arr[0][0] != "*"{
        return arr[0][0];
    }
    if arr[0][2] == arr[1][1] && arr[0][2] == arr[2][0] && arr[0][2] != "*"{
        return arr[0][2];
    }    
   
    //if neither "ඞ" or "·" satisfy our conditions, return "*"
    return "*";
}

/*
    This function checks if 'ඞ' or '·' 
    occur 3 times in any row, column, or diagonal. 
    If it does, prints a victory message for said character and end program.
*/
fn check_victory(arr: [[&str; 3]; 3]) {

    //checks rows, columns, and diagonals
    let row = check_row_victory(arr);
    let col = check_col_victory(arr);
    let diag = check_diag_victory(arr);

    //sees if any character meets the condition (with some horrific code) and prints a message and exits program if so.
    if row != "*"  {
        panic!("{}   wins!", row);
    }
    if col != "*"  {
        panic!("{}   wins!", col);
    }
    if diag != "*"  {
        panic!("{}   wins!", diag);
    }

}

fn main() {
    //defines the tic tac toe board as a 3x3 matrix of strings, with "*" acting as a "empty/default" value
    let mut matrix = [["*","*","*"], ["*","*","*"], ["*","*","*"]];
    
    //defines a vector containing the two characters we use to represent the players
    let chars = vec!["ඞ", "■"];

    //defines who the player is. 0 = first player, 1 = second player
    let mut player = 0;

    //defines the number of turns that have passed.
    let mut num_turns = 0;

    //loops until completion (one player wins, or both tie).
    loop {

        //prints whose turn it is.
        println!("\n\nplayer is now {}", chars[player]);

        //if 9 turns have passed, and a win condition has not been met, it is impossible to play on, and the game is a tie.
        //if this happens, we print a tie message and end the game
        if num_turns == 9{
            println!("No more possible moves left, tie.");
            process::exit(1);
        }

        //prints board
        println!();
        print_board(matrix);
        
        //gets user input for the column they'll place the character in.
        println!("Enter a column!");
        let column = request_user_value() - 1;

        //gets user input for the row they'll place the character in.
        println!("\nEnter a row!");
        let row = request_user_value() - 1;

        //gets the character corresponding to the player.
        let char = chars[player];

        //if the place they want to put their character in (row & column) is not occupied by the other players character,
        //place it in the corresponding place.
        if matrix[row as usize][column as usize] == "*" {
            matrix[row as usize][column as usize] = char;

            //switch players
            if player == 1{
                player = 0
            }
            else {
                player = 1
            }
        }

        //if the place was already chosen by the other player, prints an error message and loops again so that
        //the player can input a valid value.
        else {
            println!("You can't change a value that has already been set!");
            continue;
        }

        //check if either player has won yet.
        check_victory(matrix);

        //increment the number of turns.
        num_turns = num_turns + 1;
    }

}
