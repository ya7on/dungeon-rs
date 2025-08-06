use rand::{
    Rng, RngCore, SeedableRng,
    distr::uniform::{SampleRange, SampleUniform},
};
use rand_chacha::ChaCha8Rng;

/// A random number generator wrapper.
#[derive(Debug)]
pub(crate) struct MyRng {
    rng: ChaCha8Rng,
    tick_id: usize,
}

impl MyRng {
    /// Create a new random number generator.
    pub fn new() -> Self {
        MyRng { rng: ChaCha8Rng::from_os_rng(), tick_id: 0 }
    }

    /// Create a new random number generator with a given seed.
    #[must_use]
    pub(crate) fn from_seed(seed: [u8; 32]) -> Self {
        MyRng { rng: ChaCha8Rng::from_seed(seed), tick_id: 0 }
    }

    /// Load a random number generator from a seed and tick ID.
    ///
    /// # Warning!
    /// O(n)
    #[must_use]
    pub(crate) fn load(seed: [u8; 32], tick_id: usize) -> Self {
        let mut rng = ChaCha8Rng::from_seed(seed);
        (0..tick_id).for_each(|_| {
            rng.next_u32();
        });
        MyRng { rng, tick_id }
    }

    /// Generate the next random number.
    #[must_use]
    pub(crate) fn range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform + Copy,
        R: SampleRange<T>,
    {
        self.tick_id += 1;
        self.rng.random_range(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_new() {
        let seed = [0; 32];
        let mut rng = MyRng::from_seed(seed);
        assert_eq!(rng.tick_id, 0);

        assert_eq!(rng.range(0..=100), 18);
        assert_eq!(rng.range(0..=100), 84);
        assert_eq!(rng.range(0..=100), 91);

        assert_eq!(rng.tick_id, 3);
    }

    #[test]
    fn test_rng_load() {
        let seed = [0; 32];
        {
            let mut rng = MyRng::load(seed, 0);
            assert_eq!(rng.tick_id, 0);
            assert_eq!(rng.range(0..=100), 18);
            assert_eq!(rng.tick_id, 1);
        }
        {
            let mut rng = MyRng::load(seed, 1);
            assert_eq!(rng.tick_id, 1);
            assert_eq!(rng.range(0..=100), 84);
            assert_eq!(rng.tick_id, 2);
        }
        {
            let mut rng = MyRng::load(seed, 2);
            assert_eq!(rng.tick_id, 2);
            assert_eq!(rng.range(0..=100), 91);
            assert_eq!(rng.tick_id, 3);
        }
    }

    #[test]
    fn test_different_seeds() {
        let seed1 = [0; 32];
        let seed2 = [1; 32];
        let mut rng1 = MyRng::load(seed1, 0);
        let mut rng2 = MyRng::load(seed2, 0);
        assert_ne!(rng1.range(0..=100), rng2.range(0..=100));
    }
}
