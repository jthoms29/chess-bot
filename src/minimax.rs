use std::collections::HashSet;
use crate::state::{self, action_to_state, copy_state, generate_legal_moves, State};
#[derive(Default)]
pub struct MinimaxSearch {
    max_best: i16,
    min_best: i16,
    depth_limit: u8,
}


impl MinimaxSearch {

    pub fn new(depth_limit: u8) -> MinimaxSearch {
        let search_struct = MinimaxSearch { 
            max_best: (i16::MIN), 
            min_best: (i16::MAX), 
            depth_limit: (depth_limit) 
        };

        return search_struct;
    }

    /* Returns the best move that can be taken given a state and depth-limit */
    pub fn search_max(&self, cur_state: &State, depth_limit: u16) -> String {
        let mut best_action: String = String::new();
        let mut best_val: i16 = i16::MIN;
        let mut cur_val: i16;

        let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);
        for action in &legal_moves {
            let mut new_state = copy_state(&cur_state);
            action_to_state(&mut new_state, &action);
            cur_val = min_recurse(&new_state, depth_limit-1)
        }

        return best;
    }


    fn max_recurse(&self, cur_state: &State, depth_limit: u16) -> i16 {
        if State::victory_check(&cur_state) == -1 {
            return -1000;
        }
        if State:: victory_check(&cur_state) == 1 {
            return 1000;
        }


        let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);

        for action in &legal_moves {
            let mut new_state = copy_state(&cur_state);
            action_to_state(&mut new_state, &action);
        }
    }

    fn min_recurse(&self, cur_state: &State, depth_limit: u16) -> i16 {
        if State::victory_check(cur_state) == -1 {
            return -1000;
        }
        if State:: victory_check(cur_state) == 1 {
            return 1000;
        }

        let legal_moves: HashSet<String> = generate_legal_moves(&cur_state);

        for action in &legal_moves {
            let mut new_state = copy_state(&cur_state);
            action_to_state(&mut new_state, &action);
        }
    }


}