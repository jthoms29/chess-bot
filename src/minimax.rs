
use crate::state;
#[derive(Default)]
pub struct minimax_node {
    state: Option<Box<state::State>> { Some, None},
    minimax_val: int16,
    alpha_beta_val: int16,
    

}