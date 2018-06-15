//! Contains functions for random data generation

use rand::{Rng, thread_rng};
use rand::random;

/**
   Builds a string of random numbers of the inputted length
*/
pub fn rand_string(length : usize) -> String {
    let s: String = thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();

    return s;
}