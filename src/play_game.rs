
use std::io;
use std::collections::HashSet;


/*
 * Get an input from the player. This function ensures that the move the user gives as input is legal
 */
pub fn get_player_input(legal_moves:&HashSet<String>) -> String {
    let mut input = String::new();

    println!("Input a move:");
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        /* Remove newline - checks for both \n and \r */
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }

        if legal_moves.contains(&input) {
            break;
        }
        else {
            println!("Invalid move. Try again:");
        }
    }
    return input;
}