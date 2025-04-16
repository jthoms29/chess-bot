//mod state;
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
    // Dict mapping white piece location to piece
    white: HashMap<(i8, i8), char>,
    // Dict mapping black piece location to piece
    black: HashMap<(i8, i8), char>,
    // Boolean dictating which side's turn it is
    white_turn: bool,
}

impl State {

    /*
    Generate a string representation of the current state
     */
    fn to_string(&self) -> String {
        let mut str_rep: String = String::from("");

        str_rep.push_str("   A  B  C  D  E  F  G  H \n");

        for i in 0..8 {
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
            str_rep.push_str("\n");
        }
    return str_rep;
    }



    /*
    Create a default State struct. All pieces in default position
     */
    fn default_state() -> Self {
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
        //new_state.white_turn = false;
        return new_state;

    }
}


// A set of separate functions that act on the state

/*
Check if it is white's turn
 */
fn is_white_turn(cur_state: &State) -> bool {
    return cur_state.white_turn;
}

/*
Check if coordinates would put piece out of bounds.
- true: if in bounds
- false: if out of bounds 
 */
fn in_bound(x:i8, y:i8) -> bool {
    return (x >= 0 && x <= 7) && (y >= 0 && y <= 7);
}



/* 
Compute legal moves a pawn piece at loc_x,loc_y can do. Adds them to the passed in legal_moves vector
*/
fn pawn_legal_moves(white_turn: bool, loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>,
opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut Vec<String>) {

    // white pawns move up(-), black pawns move down(+)
    let direction: i8 = match white_turn {
        true => {-1 }
        false => { 1 }
    };

    // check up/down 
    if in_bound(loc_x, loc_y+direction) && 
    !cur_player.contains_key(&(loc_x, loc_y+direction)) && 
    !opp_player.contains_key(&(loc_x, loc_y+direction)) {
        let new_y = loc_y+direction;
        legal_moves.push(format!("{loc_x},{loc_y} to {loc_x},{new_y}"));
    }

    // if first move, can move up/down twice
    if (white_turn && loc_y == 6) || (!white_turn && loc_y == 1) &&
    !cur_player.contains_key(&(loc_x, loc_y+direction*2)) && 
    !opp_player.contains_key(&(loc_x, loc_y+direction*2)) {
        let new_y = loc_y+direction*2;
        legal_moves.push(format!("{loc_x},{loc_y} to {loc_x},{new_y}"));
    }

    // check diagonals for opposite team pieces
    // up/down-left
    if opp_player.contains_key(&(loc_x-1, loc_y+direction)) {
        let new_x = loc_x-1;
        let new_y = loc_y+direction;
        legal_moves.push(format!("{loc_x},{loc_y} to {new_x},{new_y}"))
    }
    // up/down-right
    if opp_player.contains_key(&(loc_x+1, loc_y+direction)) {
        let new_x = loc_x+1;
        let new_y = loc_y+direction;
        legal_moves.push(format!("{loc_x},{loc_y} to {new_x},{new_y}"))
    }
}


/* 
Compute legal moves a rook piece at loc_x,loc_y can do. Adds them to the passed in legal_moves vector
*/
fn rook_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut Vec<String>) {

    for i in 0..4 {
        let new_x: i8 = loc_x;
        let new_y: i8 = loc_y;

        let x_dir = match i {
            // up/down
            0 | 2 => 0,
            // right
            1 => 1,
            //left
            3 => -1,
            _ => 0 //impossible to reach
        };

        let y_dir = match i {
            // up
            0 => -1,
            // left/right
            1 | 3 => 0,
            // down
            2 => 1,
            _ => 0 //impossible to reach
        };

        // check each space from the current direction
        loop {
            let new_x = new_x + x_dir;
            let new_y = new_y + y_dir;
            if in_bound(new_x, new_y) &&
            !cur_player.contains_key(&(new_x, new_y)) {
                legal_moves.push(format!("{loc_x},{loc_y} to {new_x},{new_y}"));

                // if an opposing player's piece is on this coordinate, we must stop here
                if opp_player.contains_key(&(new_x, new_y)) { break; }
            }
            // no more valid moves possible
            else {
                break;
            }
        }
    }
}



fn bishop_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut Vec<String>) {

    // check the four diagonals. The changes to a coordinate for a given direction are decided by x_dir and y_dir
    for i in 0..4 { 
        let new_x: i8 = loc_x;
        let new_y: i8 = loc_y;

        let x_dir = match i {
            //left
            0 | 2 => -1,
            //right
            1 | 3 => 1,
            _ => 0 //impossible to reach
        };

        let y_dir = match i {
            //down
            0 | 2 => 1,
            //up
            1 | 3 => -1,
            _ => 0 //impossible to reach
        };

        // go through all spaces in the current diagonal until you can't anymore (piece or edge of board)
        loop {
            let new_x = new_x + x_dir;
            let new_y = new_y + y_dir;

            if in_bound(new_x, new_y) &&
            !cur_player.contains_key(&(new_x, new_y)) {
                legal_moves.push(format!("{loc_x},{loc_y} to {new_x},{new_y}"));

                // if an opposing player's piece is on this coordinate, we must stop here
                if opp_player.contains_key(&(new_x, new_y)) { break; }
            }
            // no more valid moves possible
            else {
                break;
            }
        }
    }

}

/*
Generate a list of legal moves that can be applied to the current state
 */
fn generate_legal_moves(cur_state: &State) -> Vec<String> {

    let mut legal_moves: Vec<String> = Vec::new();

    /* Get the piece positions for the current player and the opposing player */
    let cur_player = if is_white_turn(&cur_state) {
        &cur_state.white
    } else {
        &cur_state.black
    };

    let opp_player = if is_white_turn(&cur_state) {
        &cur_state.black
    } else {
        &cur_state.white
    };

    for (key, value) in cur_player {

        let loc_x = key.0;
        let loc_y = key.1;

        match value {
            // could go to any straight line location from origin if not blocked by some other piece
            '♜' | '♖' => {
                rook_legal_moves(loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            },
            '♝' | '♗' => {
                bishop_legal_moves(loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            },
            '♟' | '♙' => {
                pawn_legal_moves(is_white_turn(&cur_state), loc_x, loc_y, &cur_player, &opp_player, &mut legal_moves);
            }
            _ => print!("")
        }
    }

        return legal_moves;
}



fn main() {
    let test1 = State::default_state();
    println!("{}", test1.to_string());
    println!("{}", is_white_turn(&test1));

    let legal_moves = generate_legal_moves(&test1);

    for s in &legal_moves {
        println!("{s}");
    }
}