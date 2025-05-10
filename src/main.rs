pub mod state;
pub mod legal_moves;
pub mod minimax;
pub mod play_game;

fn main() {
    let mut test1 = state::State::new();

    loop {
        println!("{}", test1.to_string());
        println!("{}", state::State::is_white_turn(&test1));


        let legal_moves = state::generate_legal_moves(&test1);

        /*
        for s in &legal_moves {
            println!("{s}");
        }
        */

        let action: String = play_game::get_player_input(&legal_moves);
        state::action_to_state(&mut test1, &action);
    }
}