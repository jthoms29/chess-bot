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
        let mut str_rep: String = String::from("   A  B  C  D  E  F  G  H \n");
        for i in 0..8 {
            // y axis coordinates are 1-8 from bottom to top
            str_rep.push_str(&format!("{} ", i8::abs(i-8)));
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
            str_rep.push_str(&format!(" {}\n", i8::abs(i-8)));
        }
        str_rep.push_str("   A  B  C  D  E  F  G  H \n");
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
                '♕' => { minimax_val += 50},
                '♔' => { minimax_val += 100 },
                '♖' => { minimax_val += 15 },
                '♗' => { minimax_val += 10 },
                '♘' => { minimax_val += 7 },
                '♙' => { minimax_val += 1 },
                _   => { () }
            }
        }  
        for (&_key, value) in &self.black {
            match value {
                '♛' => { minimax_val -= 50},
                '♚' => { minimax_val -= 100 },
                '♜' => { minimax_val -= 15 },
                '♝' => { minimax_val -= 10 },
                '♞' => { minimax_val -= 7 },
                '♟' => { minimax_val -= 1 },
                _   => { () }
            }
        }  
        return minimax_val;
    }


    pub fn action_to_state(&mut self, action: &[i8; 4]) {

        // get piece positions from action array
        let start_x = action[0];
        let start_y = action[1];
        let end_x = action[2];
        let end_y = action[3];

        // get the current player's hashmap. Mutable borrow, so done in block
        {
            let cur_player: &mut HashMap<(i8, i8), char> = match self.white_turn {
                true => { &mut self.white },
                false => { &mut self.black },
            };

            // move piece from starting position to ending position
            let piece: char = cur_player.remove(&(start_x, start_y)).expect("piece not in hashmap");
            cur_player.insert((end_x, end_y), piece);
        }

        // second block, check if opposing player has piece in end_pos. Remove if so 
        {
            let opp_player: &mut HashMap<(i8, i8), char> = match self.white_turn {
                false => { &mut self.white },
                true => { &mut self.black },
            };
            //remove if possible
            let opp_piece = opp_player.remove(&(end_x, end_y));
            match opp_piece {
                Some(x) => {
                    /* If a piece did exist at this spot, check if it was a king and set state's victory flag accordingly */
                    self.victory_flag = match x {
                        // white has taken out opposing king
                        '♚' => 1,
                        // black has taken out opposing king
                        '♔' => -1,
                        // neither has taken out a king, victory flag remains unset
                        _ => 0
                    }
                }
                // wasn't a king, do nothing
                _ => (),
            }
        }
        // swap who's turn it is
        self.white_turn = !self.white_turn;

    }

    /*
    Creates a copy of this state
    */
    pub fn copy_state(&self) -> State {
        let mut new_state: State = State::new();
        new_state.white = self.white.clone();
        new_state.black = self.black.clone();
        new_state.victory_flag = self.victory_flag;
        new_state.white_turn = self.white_turn;

        return new_state;
    }
    /*
    Generate a list of legal moves that can be applied to the current state
    */
    pub fn generate_legal_moves(&self) -> HashSet<[i8; 4]> {

        let mut legal_moves: HashSet<[i8; 4]> = HashSet::new();

        /* Get the piece positions for the current player and the opposing player */
        let cur_player = if self.white_turn {
            &self.white
        } else {
            &self.black
        };

        let opp_player = if self.white_turn {
            &self.black
        } else {
            &self.white
        };

        for (&key, value) in cur_player {

            let loc_x = key.0;
            let loc_y = key.1;

            // put all legal moves for current board state in legal_moves vector
            match value {
                '♛' | '♕' => {
                    legal_moves::queen_legal_moves(loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
                }
                '♚' | '♔' => {
                    legal_moves::king_legal_moves(loc_x, loc_y, &cur_player, &mut legal_moves);
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
                    legal_moves::pawn_legal_moves(self.white_turn, loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
                }
                _ => ()
            }
        }

            return legal_moves;
    }


}





