
use std::io;
use std::collections::HashSet;
use crate::{legal_moves, state::{action_to_state, generate_legal_moves, State}, minimax::{search_min}};


/*
 * Get an input from the player. This function ensures that the move the user gives as input is legal
 */
pub fn get_player_input(legal_moves: &HashSet<String>) -> String {
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

pub fn player_turn(cur_state: &State) -> String {
    let mut input: String = String::new();
    let legal_moves: HashSet<String> = generate_legal_moves(cur_state);

    input = get_player_input(&legal_moves);
    return input;
}

pub fn comp_turn(cur_state: &State) -> String {
    let mut input: String = String::new();
    let input = search_min(&cur_state, 5);
    return input;
}

pub fn play_game() {
    /* Create the initial state */
    let mut state = State::new();
    let mut action = String::new();
    /* temporary. Player is always white */
    let player = true;

    loop {
        if player == state.is_white_turn() {
            println!("{}", state.to_string());
            action = player_turn(&mut state);
            action_to_state(&mut state, &action);
        }
        else {
            action = comp_turn(&state);
            action_to_state(&mut state, &action);
        }
    }
}