#[cfg(test)]
#[path = "tests/roller_tests.rs"]
mod roller_tests;

use crate::dice::Dice;
use rand::SeedableRng;
use rand_hc::Hc128Rng;

/// Dice roller
pub struct Roller {
    csprng: Hc128Rng,
}

impl Roller {
    /// Creates a new roller with cryptographicly secure random algorithm
    pub fn new() -> Self {
        Roller {
            csprng: rand_hc::Hc128Rng::from_entropy(),
        }
    }

    /// Rolls a single dice
    ///
    /// * `sides` - The sides the dice has
    pub fn roll_dice(self, sides: u32) -> Dice {
        Dice {
            sides: sides,
            value: 1,
        }
    }
}
