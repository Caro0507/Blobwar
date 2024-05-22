//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::fmt;
use rayon::prelude::*;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        //calcule le max value de chaque movement.
        //return le movement qui a le max value s'il y a
        let (m,_)= state.movements().par_bridge().map(|m| (m,state.play(&m).value())).max_by_key(|&(_, value)| value).unwrap();
        return Some(m);
    }
}
