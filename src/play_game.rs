
use text_io;
use regex::Regex;
use std::collections::HashSet;
use crate::minimax::MinimaxResult;
use crate::state;
use crate::minimax;
use std::{thread, time, io::Write};
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

pub fn comp_turn(cur_state: &state::State) -> MinimaxResult {
    return minimax::search_min(&cur_state, 5);
}

pub fn play_game() {
    /* Create the initial state */
    let mut state = state::State::new();


    let player: bool;

    loop {
        print!("White or black?: ");
        let mut input: String  = text_io::read!("{}\n");
        input = input.to_lowercase();

        if input == "white" {
            player = true;
            break;
        }

        else if input == "black" {
            player = false;
            break;
        }
        println!("Invalid input.")
    }

    while state.victory_check() == 0 {
        if player == state.is_white_turn() {
            println!("{}", state.to_string());
            let action = player_turn(&mut state);
            state.action_to_state(&action);
            println!("{}", state.to_string());
        }
        else {
            // need to give searcher thread own copy
            let clone = state.copy_state();
            let search_thread = thread::spawn(move || {
                comp_turn(&clone)
            });
            while !search_thread.is_finished() {
                let animation = ['|', '/', '-', '\\'];
                for bar in animation {
                    print!("\rThinking {bar}");
                    thread::sleep(time::Duration::from_millis(500));
                    std::io::stdout().flush();
                }
            }
            std::io::stdout().flush();
            let result = search_thread.join().unwrap();

            println!("\rMinimax value: {}", result.minimax_val);
            state.action_to_state(&result.action);
        }
    }
    if state.victory_check() == 1 {
        println!("White wins.")
    }
    else {
        println!("Black wins.")
    }
}
