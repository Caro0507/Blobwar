//! Alpha - Beta algorithm.
use std::fmt;
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);

    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (m,_) = state.movements()
                                .map(|m| (m,alpha_beta(&state.play(&m), &self.0 - 1, std::i8::MIN,std::i8::MAX,true)))
                                .max_by_key(|&(_, value)| value)
                                .unwrap();
        return Some(m);
    }
}

fn alpha_beta(state:&Configuration, depth: u8, mut alpha: i8, mut beta: i8, maximzing_player:bool) ->i8
{
    // get out of the recursive function when depth in null or if there is no more node 
    if depth == 0 || state.movements().next().is_none() {
        return state.value();
    }
    let ret;
    if maximzing_player {
        // other turn, only changes alpha, returns min
        for m in state.movements() {
            alpha = std::cmp::max(alpha, alpha_beta(&state.play(&m), depth - 1, alpha, beta,!maximzing_player));

            // if alpha >= beta of the node, deletes the node (cut node)
            if alpha >= beta {
                return beta;
            }
        }
        ret = alpha;
    // other turn (takes the max)
    } else {
        // my turn, only changes beta, returns max
        for m in state.movements(){
            beta = std::cmp::min(beta, alpha_beta(&state.play(&m), depth - 1, alpha, beta,!maximzing_player));
            
            // if alpha >= beta of the node, deletes the node (cut node)
             if alpha >= beta {
                return alpha;
            }
        }
        ret = beta;
    }

    return ret;

}
