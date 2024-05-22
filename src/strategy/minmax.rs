//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use rayon::prelude::*;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // current = true, begins by max()
        let (m,_) = state.movements()
                            .map(|m| (m,minmax(&state.play(&m), &self.0 - 1, true)))
                            .max_by_key(|&(_, value)| value)
                            .unwrap();   
        return Some(m);
    }
}

fn minmax(state:&Configuration, depth: u8 , maximizing_player:bool)->i8
{
    // get out of the recursive function when depth in null or if there is no more node 
    if depth == 0 || state.movements().next().is_none() {
        return state.value();
    }

    let value;
    if maximizing_player {
        value = state.movements()
                .par_bridge()
                .map(|m| (minmax(&state.play(&m), depth - 1, !maximizing_player)))
                .max()
                .unwrap();
    } else {
        value = state.movements()
                .par_bridge()
                .map(|m| (minmax(&state.play(&m), depth - 1, !maximizing_player)))
                .min()
                .unwrap();
    }

    return value;
}


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
