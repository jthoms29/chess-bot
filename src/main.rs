pub mod state;
pub mod legal_moves;
pub mod minimax;

fn main() {
    let test1 = state::State::default_state();
    println!("{}", test1.to_string());
    println!("{}", state::State::is_white_turn(&test1));

    let legal_moves = state::generate_legal_moves(&test1);

    for s in &legal_moves {
        println!("{s}");
    }
}