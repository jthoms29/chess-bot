use std::collections::HashSet;
use crate::state::{State};

#[derive(Default)]
pub struct MinimaxResult {
    pub minimax_val: i16,
    pub action: [i8; 4],
}

/* Returns the best move that can be taken given a state and depth-limit */
pub fn search_max(cur_state: &State, depth_limit: u16) -> MinimaxResult {
    let mut best_action: [i8; 4] = [0; 4];
    let alpha: i16 = i16::MIN;
    let beta:i16 = i16::MAX;
    let mut best: i16 = i16::MIN;

    let mut cur_val: i16;

    let legal_moves: HashSet<[i8; 4]> = cur_state.generate_legal_moves();
    for action in &legal_moves {
        let mut new_state = cur_state.copy_state();
        new_state.action_to_state(&action);
        cur_val = min_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val > best {
            best = cur_val;
            best_action = action.clone();
        } 
    }
    let res = MinimaxResult{minimax_val: best, action: best_action};
    return res;
}


/* Returns the best move that can be taken given a state and depth-limit */
pub fn search_min(cur_state: &State, depth_limit: u16) -> MinimaxResult {
    let mut best_action: [i8; 4] = [0; 4];
    let alpha: i16 = i16::MIN;
    let beta:i16 = i16::MAX;
    let mut best: i16 = i16::MAX;

    let mut cur_val: i16;

    let legal_moves: HashSet<[i8; 4]> = cur_state.generate_legal_moves();
    for action in &legal_moves {
        let mut new_state = cur_state.copy_state();
        new_state.action_to_state(&action);
        cur_val = max_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val < best {
            best = cur_val;
            best_action = action.clone();
        } 
    }
    let res = MinimaxResult{minimax_val: best, action: best_action};
    return res;
}


fn max_recurse(cur_state: &State, mut alpha: i16, beta: i16, depth_limit: u16) -> i16 {
    if cur_state.victory_check() == -1 {
        return -1000;
    }
    if cur_state.victory_check() == 1 {
        return 1000;
    }

    if depth_limit == 0 {
        return cur_state.estimate_minimax();
    }

    let mut cur_val: i16;
    let mut best: i16 = i16::MIN;

    let legal_moves: HashSet<[i8; 4]> = cur_state.generate_legal_moves();
    for action in &legal_moves {
        let mut new_state = cur_state.copy_state();
        new_state.action_to_state(&action);
        cur_val = min_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val > best {
            best = cur_val;
        }
        /* If min finds a lower value than the current best, we can continue */
        if best >= beta {
            return best;
        }
        alpha = std::cmp::max(alpha, best);
    }
    return best;
}


fn min_recurse(cur_state: &State, alpha: i16, mut beta: i16, depth_limit: u16) -> i16 {
    if cur_state.victory_check() == -1 {
        return -1000;
    }
    if cur_state.victory_check() == 1 {
        return 1000;
    } 

    if depth_limit == 0 {
        return cur_state.estimate_minimax();
    }

    let mut cur_val: i16;
    let mut best: i16 = i16::MAX;

    let legal_moves: HashSet<[i8; 4]> = cur_state.generate_legal_moves();

    for action in &legal_moves {
        let mut new_state = cur_state.copy_state();
        new_state.action_to_state(&action);
        cur_val = max_recurse(&new_state, alpha, beta, depth_limit-1);

        if cur_val < best {
            best = cur_val
        }

        /* If max finds a better value than the current best, we can continue */
        if best <= alpha {
            return best;
        }
        beta = std::cmp::min(best, beta);
    }
    return best;
}

