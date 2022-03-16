use rand::Rng;
use rand::SeedableRng;
use rand_hc::Hc128Rng;

#[cfg(test)]
use mockall::automock;

/// Random number generator provider
#[cfg_attr(test, automock)]
pub trait RngEngine {
    fn next(&mut self, low: u32, high: u32) -> u32;
}

/// A random number generator that implements the Hc128 algorithm
struct Hc128RngEngine {
    csprng: Hc128Rng,
}

impl Hc128RngEngine {
    pub fn new() -> Self {
        Hc128RngEngine {
            csprng: Hc128Rng::from_entropy(),
        }
    }
}

impl RngEngine for Hc128RngEngine {
    fn next(&mut self, low: u32, high: u32) -> u32 {
        self.csprng.gen_range(low..=high)
    }
}
