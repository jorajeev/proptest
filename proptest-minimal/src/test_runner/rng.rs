//-
// Copyright 2017, 2018, 2019, 2020 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

/// A random number generator used by proptest.
///
/// This is a simple wrapper around ChaChaRng for the minimal implementation.
#[derive(Clone, Debug)]
pub struct TestRng {
    rng: ChaChaRng,
}

impl TestRng {
    /// Create a new TestRng from a seed.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        TestRng {
            rng: ChaChaRng::from_seed(seed),
        }
    }

    /// Create a deterministic RNG with a hardcoded seed.
    ///
    /// This is useful for reproducible tests.
    pub fn deterministic_rng() -> Self {
        Self::from_seed([0u8; 32])
    }
}

impl RngCore for TestRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

impl Default for TestRng {
    fn default() -> Self {
        Self::deterministic_rng()
    }
}
