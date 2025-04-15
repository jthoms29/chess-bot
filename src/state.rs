use std::collections::HashMap;
pub struct State {
    // Dict mapping white piece location to piece
    white: HashMap<(i8, i8), char>,
    // Dict mapping black piece location to piece
    black: HashMap<(i8, i8), char>,
    // Boolean dictating which side's turn it is
    white_turn: bool,
}

impl State {
    fn to_string(&self) -> String {
        let mut str_rep: String = String::from("");

        for i in 0..8 {
            for j in 0..8 {
                str_rep.push_str(".");
            }
            str_rep.push_str("\n");
        }
    return str_rep;
    }
}

