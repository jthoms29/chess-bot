use std::collections::HashSet;
use std::collections::HashMap;
/*
Check if coordinates would put piece out of bounds.
- true: if in bounds
- false: if out of bounds 
*/
pub fn in_bound(x:i8, y:i8) -> bool {
    return (x >= 0 && x <= 7) && (y >= 0 && y <= 7);
}


/* 
Compute legal moves a pawn piece at loc_x,loc_y can do. Adds them to the passed in legal_moves vector
*/
pub fn pawn_legal_moves(white_turn: bool, loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>,
opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut HashSet<[i8; 4]>) {

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
        //legal_moves.insert(format!("{loc_x},{loc_y} to {loc_x},{new_y}"));
        legal_moves.insert([loc_x, loc_y, loc_x, new_y]);
    }

    // if first move, can move up/down twice
    if (white_turn && loc_y == 6) || (!white_turn && loc_y == 1) &&
    !cur_player.contains_key(&(loc_x, loc_y+direction*2)) && 
    !opp_player.contains_key(&(loc_x, loc_y+direction*2)) {
        let new_y = loc_y+direction*2;
        //legal_moves.insert(format!("{loc_x},{loc_y} to {loc_x},{new_y}"));
        legal_moves.insert([loc_x, loc_y, loc_x, new_y]);
    }

    // check diagonals for opposite team pieces
    // up/down-left
    if opp_player.contains_key(&(loc_x-1, loc_y+direction)) {
        let new_x = loc_x-1;
        let new_y = loc_y+direction;
        //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
        legal_moves.insert([loc_x, loc_y, new_x, new_y]);
    }
    // up/down-right
    if opp_player.contains_key(&(loc_x+1, loc_y+direction)) {
        let new_x = loc_x+1;
        let new_y = loc_y+direction;
        //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
        legal_moves.insert([loc_x, loc_y, new_x, new_y]);
    }
}


/* 
Compute legal moves a rook piece at loc_x,loc_y can do. Adds them to the passed in legal_moves vector
*/
pub fn rook_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut HashSet<[i8; 4]>) {

    for i in 0..4 {
        let mut new_x: i8 = loc_x;
        let mut new_y: i8 = loc_y;

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
            new_x += x_dir;
            new_y += y_dir;
            if in_bound(new_x, new_y) &&
            !cur_player.contains_key(&(new_x, new_y)) {
                //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
                legal_moves.insert([loc_x, loc_y, new_x, new_y]);

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



pub fn bishop_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut HashSet<[i8; 4]>) {

    // check the four diagonals. The changes to a coordinate for a given direction are decided by x_dir and y_dir
    for i in 0..4 { 
        let mut new_x: i8 = loc_x;
        let mut new_y: i8 = loc_y;

        let x_dir = match i {
            //left
            0 | 3 => -1,
            //right
            1 | 2 => 1,
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
            new_x += x_dir;
            new_y += y_dir;

            if in_bound(new_x, new_y) &&
            !cur_player.contains_key(&(new_x, new_y)) {
                //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
                legal_moves.insert([loc_x, loc_y, new_x, new_y]);

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


pub fn queen_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, opp_player:&HashMap<(i8,i8), char>, legal_moves:&mut HashSet<[i8; 4]>) {
    // check paths from all 8 directions
    for i in 0..8 {
        let mut new_x = loc_x;
        let mut new_y = loc_y;

        // direction from origin. Goes clockwise 0-7
        let x_dir = match i {
            // up/down
            0 | 4 => 0,
            // upright, right, downright
            1 | 2 | 3 => 1,
            // downleft, left, upleft
            5 | 6 | 7 => -1,
            //should never be reached
            _ => 0
        };

        let y_dir = match i {
            // right/left
            2 | 6 => 0,
            // up, upright, up-left
            0 | 1 | 7 => -1,
            // downright, down, downleft
            3 | 4 | 5 => 1,
            //should never be reached
            _ => 0
        };
        // go through all spaces in the current direction until you can't anymore (piece or edge of board)
        loop {
            new_x += x_dir;
            new_y += y_dir;

            if in_bound(new_x, new_y) &&
            !cur_player.contains_key(&(new_x, new_y)) {
                //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
                legal_moves.insert([loc_x, loc_y, new_y, new_y]);
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
Computes legal moves for king to take in the current state. Similar to queen function, but without looping - 1 move each direction
*/
pub fn king_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, legal_moves:&mut HashSet<[i8; 4]>) {
    // check paths from all 8 directions
    for i in 0..8 {
        let new_x = loc_x;
        let new_y = loc_y;

        // direction from origin. Goes clockwise 0-7
        let x_dir = match i {
            // up/down
            0 | 4 => 0,
            // upright, right, downright
            1 | 2 | 3 => 1,
            // downleft, left, upleft
            5 | 6 | 7 => -1,
            //should never be reached
            _ => 0
        };

        let y_dir = match i {
            // right/left
            2 | 6 => 0,
            // up, upright, up-left
            0 | 1 | 7 => -1,
            // downright, down, downleft
            3 | 4 | 5 => 1,
            //should never be reached
            _ => 0
        };

        // check if 1 space forward from current position is valid (in bound, no pieces from own side)
        let new_x = new_x + x_dir;
        let new_y = new_y + y_dir;

        if in_bound(new_x, new_y) &&
        !cur_player.contains_key(&(new_x, new_y)) {
            //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
            legal_moves.insert([loc_x, loc_y, new_x, new_y]);
        }

    }
}

/*
Computes legal moves knight can take in this current state
*/
pub fn knight_legal_moves(loc_x:i8, loc_y:i8, cur_player:&HashMap<(i8, i8), char>, legal_moves:&mut HashSet<[i8; 4]>) {

    // the knight has 8 different possible landing points. This match looks confusing
    // but trust me it gets them all. Combination of two spaces then one

    for i in 0..8 {
        let new_x = loc_x;
        let new_y = loc_y;

        // direction from origin. Goes clockwise 0-7
        let x_dir = match i {
            // short end goes left
            0 | 5 => -1,
            // short end goes right
            1 | 4  => 1,
            //long end goes left
            6 | 7 => -2,
            //long end goes right
            2 | 3 => 2,
            //should never be reached
            _ => 0
        };

        let y_dir = match i {
            // long end goes up
            0 | 1 => -2,
            // long end goes down
            4 | 5 => 2,
            // short end goes up
            2 | 7 => -1,
            // short end goes down
            3 | 6 => 1,
            //should never be reached
            _ => 0
        };

        // check if landing position is valid (in bound, no pieces from own side)
        let new_x = new_x + x_dir;
        let new_y = new_y + y_dir;

        if in_bound(new_x, new_y) &&
        !cur_player.contains_key(&(new_x, new_y)) {
            //legal_moves.insert(format!("{loc_x},{loc_y} to {new_x},{new_y}"));
            legal_moves.insert([loc_x, loc_y, new_x, new_y]);
        }
    }
}
