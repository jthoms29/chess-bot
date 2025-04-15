//mod state;
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
    // Dict mapping white piece location to piece
    white: HashMap<(u8, u8), char>,
    // Dict mapping black piece location to piece
    black: HashMap<(u8, u8), char>,
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
                

                if self.white.contains_key(&(i,j)) {
                    let piece: char = *self.white.get(&(i,j)).expect("Not in hashmap");
                    str_rep.push_str(&format!(" {} ", piece));
                }
                else if self.black.contains_key(&(i,j)) {
                    let piece: char = *self.black.get(&(i,j)).expect("Not in hashmap");
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
            ((0,1), '♞'),
            ((0,2), '♝'),
            ((0,3), '♛'),
            ((0,4), '♚'),
            ((0,5), '♝'),
            ((0,6), '♞'),
            ((0,7), '♜'),
        ]);
        for i in 0..8 {
            new_state.black.insert((1, i), '♟');
        }

        new_state.white = HashMap::from([
            ((7,0), '♖'),
            ((7,1), '♘'),
            ((7,2), '♗'),
            ((7,3), '♕'),
            ((7,4), '♔'),
            ((7,5), '♗'),
            ((7,6), '♘'),
            ((7,7), '♖'),
        ]);
        for i in 0..8 {
            new_state.white.insert((6, i), '♙');
        }

        new_state.white_turn = true;

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
fn in_bound(x:u8, y:u8) -> bool {
    return (x >= 0 && x <= 7) && (y >= 0 && y <= 7);
}


/* 
Compute legal moves a rook piece at loc_x,loc_y can do. Adds them to the passed in legal_moves vector
*/
fn rook_legal_moves(loc_x:u8, loc_y:u8, cur_player:&HashMap<(u8, u8), char>, opp_player:&HashMap<(u8,u8), char>, legal_moves:&mut Vec<String>) {
    //check upwards
    for i in (0..=loc_y).rev() {
        // if an upwards coordinate is blocked by own piece, must stop
        if cur_player.contains_key(&(loc_x, i)) {break;}
        legal_moves.push(format!("{loc_x}{loc_y} to {loc_x}{i}"));

        //  if this location contains an opposing piece, must stop here
        if opp_player.contains_key(&(loc_x, i)) {break;}
    }

    //check downwards
    for i in (loc_y..=7) {
        if cur_player.contains_key(&(loc_x, i)) {break;}
        legal_moves.push(format!("{loc_x}{loc_y} to {loc_x}{i}"));

        //  if this location contains an opposing piece, must stop here
        if opp_player.contains_key(&(loc_x, i)) {break;}
    }

    //check left
    for i in (0..=loc_x).rev() {
        // if an upwards coordinate is blocked by own piece, must stop
        if cur_player.contains_key(&(i, loc_y)) {break;}
        legal_moves.push(format!("{loc_x}{loc_y} to {i}{loc_y}"));

        //  if this location contains an opposing piece, must stop here
        if opp_player.contains_key(&(i, loc_y)) {break;}
    }

    //check right
    for i in loc_x..=7 {
        // if an upwards coordinate is blocked by own piece, must stop
        if cur_player.contains_key(&(i, loc_y)) {break;}
        legal_moves.push(format!("{loc_x}{loc_y} to {i}{loc_y}"));

        //  if this location contains an opposing piece, must stop here
        if opp_player.contains_key(&(i, loc_y)) {break;}
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
            _ => println!("uh oh"),
        }
    }

        return legal_moves;
}



fn main() {
    let test1 = State::default_state();
    println!("{}", test1.to_string());
    println!("{}", is_white_turn(&test1));

    let legal_moves = generate_legal_moves(&test1);

}