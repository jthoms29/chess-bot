use std::collections::HashSet;
use crate::state::{self, action_to_state, copy_state, generate_legal_moves, State};




/* Returns the best move that can be taken given a state and depth-limit */
pub fn search_max(cur_state: &State, depth_limit: u16) -> String {
    let mut best_action: String = String::new();
    let mut alpha: i16 = i16::MIN;
    let mut beta:i16 = i16::MAX;
    let mut best: i16 = i16::MIN;

    let mut cur_val: i16;

    let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);
    for action in &legal_moves {
        let mut new_state = copy_state(&cur_state);
        action_to_state(&mut new_state, &action);
        cur_val = min_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val > best {
            best = cur_val;
            best_action = action.clone();
        } 
    }
    return best_action;
}


/* Returns the best move that can be taken given a state and depth-limit */
pub fn search_min(cur_state: &State, depth_limit: u16) -> String {
    let mut best_action: String = String::new();
    let mut alpha: i16 = i16::MIN;
    let mut beta:i16 = i16::MAX;
    let mut best: i16 = i16::MIN;

    let mut cur_val: i16;

    let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);
    for action in &legal_moves {
        let mut new_state = copy_state(&cur_state);
        action_to_state(&mut new_state, &action);
        cur_val = max_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val < best {
            best = cur_val;
            best_action = action.clone();
        } 
    }
    return best_action;
}


fn max_recurse(cur_state: &State, alpha: i16, beta: i16, depth_limit: u16) -> i16 {
    if State::victory_check(&cur_state) == -1 {
        return -1000;
    }
    if State:: victory_check(&cur_state) == 1 {
        return 1000;
    }

    if depth_limit == 0 {
        return 0;
    }

    let mut cur_val: i16;
    let mut best: i16 = i16::MIN;

    let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);
    for action in &legal_moves {
        let mut new_state = copy_state(&cur_state);
        action_to_state(&mut new_state, &action);
        cur_val = min_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val > best {
            best = cur_val;
        }
    }
    return best;
}


fn min_recurse(cur_state: &State, alpha: i16, beta: i16, depth_limit: u16) -> i16 {
    if State::victory_check(cur_state) == -1 {
        return -1000;
    }
    if State:: victory_check(cur_state) == 1 {
        return 1000;
    } 

    if depth_limit == 0 {
        return 0;
    }

    let mut cur_val: i16;
    let mut best: i16 = i16::MAX;

    let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);

    for action in &legal_moves {
        let mut new_state = copy_state(&cur_state);
        action_to_state(&mut new_state, &action);
        cur_val = max_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val < best {
            best = cur_val
        }
    }
    return best;
}

