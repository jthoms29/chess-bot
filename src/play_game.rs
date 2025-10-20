
use std::io;
use text_io;
use regex::Regex;
use std::collections::HashSet;
use crate::state;
use crate::minimax;

/*
 * Get an input from the player. This function ensures that the move the user gives as input is legal
 */
pub fn get_player_input(legal_moves: &HashSet<[i8; 4]>) -> [i8; 4] {

    let re = Regex::new("[a-h][1-8] to [a-h][1-8]").unwrap();
    loop {
        print!("Input a move: ");
        // read until newline, not incluing
        let input: String = text_io::read!("{}\n");
        let lower = input.to_lowercase();
        // make sure input move is valid
        if !re.is_match(&lower) {
            println!("Invalid command format. Should to of form 'xy to xy'");
            continue;
        }

        let action = translate_player_input(&lower);
        if !legal_moves.contains(&action) {
            println!("Illegal move. Try again.");
            continue;
        }

        return action;
    }
}

pub fn translate_player_input(input: &String) -> [i8; 4] {

    let mut action: [i8; 4] = [0; 4];
    let parts: Vec<char> = input.chars().collect();

    // translation of coordinates to piece indices
    action[0] = (parts[0] as i8) - 97;

    // converting number character to actual number, then 
    // converting to index
    action[1] = (8 - (parts[1] as i8 - 48)).abs();

    action[2] = (parts[6] as i8) - 97;
    action[3] = (8 - (parts[7] as i8 - 48)).abs();


    return action;
}



pub fn player_turn(cur_state: &state::State) -> [i8; 4] {
    let legal_moves: HashSet<[i8; 4]> = cur_state.generate_legal_moves();

    return get_player_input(&legal_moves);
}

pub fn comp_turn(cur_state: &state::State) -> [i8; 4] {
    return minimax::search_min(&cur_state, 5);
}

pub fn play_game() {
    /* Create the initial state */
    let mut state = state::State::new();
    /* temporary. Player is always white */
    let player = true;

    while state.victory_check() == 0 {
        if player == state.is_white_turn() {
            println!("{}", state.to_string());
            let action = player_turn(&mut state);
            state.action_to_state(&action);
        }
        else {
            let action = comp_turn(&state);
            state.action_to_state(&action);
        }
    }
    if state.victory_check() == 1 {
        println!("White wins.")
    }
    else {
        println!("Black wins.")
    }
}
