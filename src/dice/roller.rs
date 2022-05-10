#[cfg(test)]
#[path = "tests/roller_tests.rs"]
mod roller_tests;

use crate::dice::Dice;
use crate::dice::RngEngine;


/// Dice roller
pub struct Roller {
    rng_engine: Box<dyn RngEngine>,
}

impl Roller {
    /// Creates a new roller
    ///
    /// * `rng_engine` - Random number generation engine
    pub fn new(rng_engine: Box<dyn RngEngine>) -> Self {
        Roller { rng_engine }
    }

    /// Rolls a single dice
    ///
    /// * `sides` - The sides the dice has
    pub fn roll_dice(&mut self, sides: u32) -> Dice {
        Dice {
            sides: sides,
            value: self.rng_engine.next(1, sides),
        }
    }
}