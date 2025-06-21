use std::collections::HashMap;
use std::collections::HashSet;
use crate::legal_moves;

#[derive(Default)]
pub struct State {
    // Dict mapping white piece location to piece
    white: HashMap<(i8, i8), char>,
    // Dict mapping black piece location to piece
    black: HashMap<(i8, i8), char>,
    // Boolean dictating which side's turn it is
    white_turn: bool,
    // -1 if black has taken out opposing king, 1 if white has, 0 if neither 
    victory_flag: i8,
}

impl State {

    /*
    Generate a string representation of the current state
    */
    pub fn to_string(&self) -> String {
        //let mut str_rep: String = String::from("   A  B  C  D  E  F  G  H \n");
        let mut str_rep: String = String::from("   0  1  2  3  4  5  6  7 \n");
        for i in 0..8 {
            //str_rep.push_str(&format!("{} ", i8::abs(i-8)));
            str_rep.push_str(&format!("{} ", i));
            for j in 0..8 {
                

                if self.white.contains_key(&(j,i)) {
                    let piece: char = *self.white.get(&(j,i)).expect("Not in hashmap");
                    str_rep.push_str(&format!(" {} ", piece));
                }
                else if self.black.contains_key(&(j,i)) {
                    let piece: char = *self.black.get(&(j,i)).expect("Not in hashmap");
                    str_rep.push_str(&format!(" {} ", piece));
                }
                else {
                    str_rep.push_str(" . ");
                }
            }
            str_rep.push_str("\n");
        }
    return str_rep;
    }


    /* Returns whether or not it's white's turn */
    pub fn is_white_turn(&self) -> bool {
        return self.white_turn;
    }

    /* Check if either player has won. Used in minimax search */
    pub fn victory_check(&self) -> i8 {
        return self.victory_flag;
    }

    /*
    Create a default State struct. All pieces in default position
    */
    pub fn new() -> Self {

        let mut new_state: State = Default::default();
        new_state.black =  HashMap::from([
            ((0,0), '♜'),
            ((1,0), '♞'),
            ((2,0), '♝'),
            ((3,0), '♛'),
            ((4,0), '♚'),
            ((5,0), '♝'),
            ((6,0), '♞'),
            ((7,0), '♜'),
        ]);
        for i in 0..8 {
            new_state.black.insert((i, 1), '♟');
        }

        new_state.white = HashMap::from([
            ((0,7), '♖'),
            ((1,7), '♘'),
            ((2,7), '♗'),
            ((3,7), '♕'),
            ((4,7), '♔'),
            ((5,7), '♗'),
            ((6,7), '♘'),
            ((7,7), '♖'),
        ]);
        for i in 0..8 {
            new_state.white.insert((i, 6), '♙');
        }

        new_state.white_turn = true;
        new_state.victory_flag = 0;
        //new_state.white_turn = false;
        return new_state;

    }

    pub fn estimate_minimax(&self) -> i16 {
        let mut minimax_val: i16 = 0;
        for (&_key, value) in &self.white {
            match value {
                '♕' => { minimax_val += 100},
                '♔' => { minimax_val += 20 },
                '♖' => { minimax_val += 15 },
                '♗' => { minimax_val += 10 },
                '♘' => { minimax_val += 7 },
                '♙' => { minimax_val += 1 },
                _   => { () }
            }
        }  
        for (&_key, value) in &self.black {
            match value {
                '♛' => { minimax_val -= 100},
                '♚' => { minimax_val -= 20 },
                '♜' => { minimax_val -= 15 },
                '♝' => { minimax_val -= 10 },
                '♞' => { minimax_val -= 7 },
                '♟' => { minimax_val -= 1 },
                _   => { () }
            }
        }  
        return minimax_val;
    }

}


// A set of separate functions that act on the state

/*
 Creates a copy of the passed in state
 */
pub fn copy_state(state: &State) -> State {
    let mut new_state: State = State::new();
    new_state.white = state.white.clone();
    new_state.black = state.black.clone();
    new_state.victory_flag = state.victory_flag;
    new_state.white_turn = state.white_turn;

    return new_state;
}



/*
Generate a list of legal moves that can be applied to the current state
*/
pub fn generate_legal_moves(cur_state: &State) -> HashSet<String> {

    let mut legal_moves: HashSet<String> = HashSet::new();

    /* Get the piece positions for the current player and the opposing player */
    let cur_player = if cur_state.white_turn {
        &cur_state.white
    } else {
        &cur_state.black
    };

    let opp_player = if cur_state.white_turn {
        &cur_state.black
    } else {
        &cur_state.white
    };

    for (&key, value) in cur_player {

        let loc_x = key.0;
        let loc_y = key.1;

        // put all legal moves for current board state in legal_moves vector
        match value {
            '♛' | '♕' => {
                legal_moves::king_legal_moves(loc_x, loc_y, &cur_player, &mut legal_moves);
            }
            '♚' | '♔' => {
                legal_moves::queen_legal_moves(loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            },
            '♜' | '♖' => {
                legal_moves::rook_legal_moves(loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            },
            '♝' | '♗' => {
                legal_moves::bishop_legal_moves(loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            },
            '♞' | '♘' => {
                legal_moves::knight_legal_moves(loc_x, loc_y, &cur_player, &mut legal_moves);
            }
            '♟' | '♙' => {
                legal_moves::pawn_legal_moves(cur_state.white_turn, loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            }
            _ => ()
        }
    }

        return legal_moves;
}



/*
Apply a legal action to the given state
*/
pub fn action_to_state(state: &mut State, action: &String) {

    // all actions are strings of the form 'x,y to x,y'
    let parts: Vec<&str> = action.split(" ").collect();

    // get the starting and ending coordinates from the action
    let start_pos: Vec<&str> = parts[0].split(",").collect();
    let end_pos: Vec<&str> = parts[2].split(",").collect();

    let start_x = start_pos[0].parse::<i8>().expect("error converting coordinate to int");
    let start_y = start_pos[1].parse::<i8>().expect("error converting coordinate to int");

    let end_x = end_pos[0].parse::<i8>().expect("error converting coordinate to int");
    let end_y = end_pos[1].parse::<i8>().expect("error converting coordinate to int");


    // get the current player's hashmap. Mutable borrow, so done in block
    {
        let cur_player: &mut HashMap<(i8, i8), char> = match state.white_turn {
            true => { &mut state.white },
            false => { &mut state.black },
        };

        // move piece from starting position to ending position
        let piece: char = cur_player.remove(&(start_x, start_y)).expect("piece not in hashmap");
        cur_player.insert((end_x, end_y), piece);
    }

    // second block, check if opposing player has piece in end_pos. Remove if so 
    {
        let opp_player: &mut HashMap<(i8, i8), char> = match state.white_turn {
            false => { &mut state.white },
            true => { &mut state.black },
        };
        //remove if possible
        let opp_piece = opp_player.remove(&(end_x, end_y));
        match opp_piece {
            Some(x) => {
                /* If a piece did exist at this spot, check if it was a king and set state's victory flag accordingly */
                state.victory_flag = match x {
                    // white has taken out opposing king
                    '♛' => 1,
                    // black has taken out opposing king
                    '♕' => -1,
                    // neither has taken out a king, victory flag remains unset
                    _ => 0
                }
            }
            // wasn't a king, do nothing
            None => (),
        }
    }
    // swap who's turn it is
    state.white_turn = !state.white_turn;

}

