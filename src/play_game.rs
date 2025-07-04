
use std::io;
use std::collections::HashSet;
use crate::{state::{action_to_state, generate_legal_moves, State}, minimax::{search_min}};


/*
 * Get an input from the player. This function ensures that the move the user gives as input is legal
 */
pub fn get_player_input(legal_moves: &HashSet<[i8; 4]>) -> [i8; 4] {
    let mut input = String::new();
    let mut action: [i8; 4];

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

        action = translate_player_input(&input);

        if legal_moves.contains(&action) {
            break;
        }
        else {
            println!("Invalid move. Try again:");
        }
    }
    return action;
}

pub fn translate_player_input(input: &String) -> [i8; 4] {

    // all actions are strings of the form 'x,y to x,y'
    let parts: Vec<&str> = input.split(" ").collect();

    // get the starting and ending coordinates from the action
    let start_pos: Vec<&str> = parts[0].split(",").collect();
    let end_pos: Vec<&str> = parts[2].split(",").collect();

    let start_x = start_pos[0].parse::<i8>().expect("error converting coordinate to int");
    let start_y = start_pos[1].parse::<i8>().expect("error converting coordinate to int");

    let end_x = end_pos[0].parse::<i8>().expect("error converting coordinate to int");
    let end_y = end_pos[1].parse::<i8>().expect("error converting coordinate to int");

    return [start_x, start_y, end_x, end_y];
}

pub fn player_turn(cur_state: &State) -> [i8; 4] {
    let legal_moves: HashSet<[i8; 4]> = generate_legal_moves(cur_state);

    return get_player_input(&legal_moves);
}

pub fn comp_turn(cur_state: &State) -> [i8; 4] {
    return search_min(&cur_state, 5);
}

pub fn play_game() {
    /* Create the initial state */
    let mut state = State::new();
    /* temporary. Player is always white */
    let player = true;

    loop {
        if player == state.is_white_turn() {
            println!("{}", state.to_string());
            let action = player_turn(&mut state);
            action_to_state(&mut state, &action);
        }
        else {
            let action = comp_turn(&state);
            action_to_state(&mut state, &action);
        }
    }
}
