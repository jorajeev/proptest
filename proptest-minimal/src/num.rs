//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Strategies for generating numeric values.
//!
//! This module provides simple strategies for generating numbers.
//! All strategies shrink by binary searching towards 0.

use crate::std_facade::fmt;
use crate::strategy::{NewTree, Strategy, ValueTree};
use crate::test_runner::TestRng;
use core::ops::{Add, Sub};
use num_traits::{Bounded, One, Zero};
use rand::Rng;

/// A `ValueTree` that shrinks numeric values by binary search.
///
/// This will binary search between the initial value and a "lo" value,
/// trying to find the smallest value that still causes a test failure.
#[derive(Debug, Clone, Copy)]
pub struct BinarySearch<T> {
    lo: T,
    curr: T,
    hi: T,
}

impl<T> BinarySearch<T>
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Zero
        + One
        + fmt::Debug,
{
    /// Create a new `BinarySearch` that starts at `start` and shrinks toward 0.
    pub fn new(start: T) -> Self {
        BinarySearch {
            lo: T::zero(),
            curr: start,
            hi: start,
        }
    }

    fn reposition(&mut self) -> bool {
        // If we're already at lo, we can't simplify further
        if self.curr == self.lo {
            return false;
        }

        // Try the midpoint, rounding toward lo
        let interval = if self.hi > self.curr {
            self.hi - self.curr
        } else {
            self.curr - self.lo
        };

        let half = interval / (T::one() + T::one());
        if half == T::zero() {
            // No more room to binary search
            self.curr = self.lo;
        } else {
            self.curr = self.lo + half;
        }

        true
    }
}

impl<T> ValueTree for BinarySearch<T>
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Zero
        + One
        + fmt::Debug,
{
    type Value = T;

    fn current(&self) -> T {
        self.curr
    }

    fn simplify(&mut self) -> bool {
        if self.hi <= self.lo {
            return false;
        }

        self.hi = self.curr;
        self.reposition()
    }

    fn complicate(&mut self) -> bool {
        if self.hi <= self.lo {
            return false;
        }

        self.lo = if self.curr == self.hi {
            return false;
        } else {
            self.curr + T::one()
        };

        self.reposition()
    }
}

/// Generate uniform integer strategy for a type.
macro_rules! int_strategy {
    ($type:ty) => {
        /// Strategy that generates values uniformly over the full range.
        #[derive(Debug, Clone, Copy)]
        #[must_use = "strategies do nothing unless used"]
        pub struct Any(());

        /// Constant for generating any value of this type.
        pub const ANY: Any = Any(());

        impl Strategy for Any {
            type Tree = BinarySearch<$type>;
            type Value = $type;

            fn new_tree(&self, rng: &mut TestRng) -> NewTree<Self> {
                Ok(BinarySearch::new(rng.random::<$type>()))
            }
        }

        impl Strategy for core::ops::Range<$type> {
            type Tree = BinarySearch<$type>;
            type Value = $type;

            fn new_tree(&self, rng: &mut TestRng) -> NewTree<Self> {
                use crate::test_runner::Reason;

                if self.is_empty() {
                    return Err(Reason::from("Range is empty"));
                }

                let value = rng.random_range(self.start..self.end);
                Ok(BinarySearch::new(value))
            }
        }

        impl Strategy for core::ops::RangeInclusive<$type> {
            type Tree = BinarySearch<$type>;
            type Value = $type;

            fn new_tree(&self, rng: &mut TestRng) -> NewTree<Self> {
                use crate::test_runner::Reason;

                if self.is_empty() {
                    return Err(Reason::from("Range is empty"));
                }

                let value = rng.random_range(*self.start()..=*self.end());
                Ok(BinarySearch::new(value))
            }
        }
    };
}

/// Module for i8 strategies.
pub mod i8 {
    use super::*;
    int_strategy!(i8);
}

/// Module for i16 strategies.
pub mod i16 {
    use super::*;
    int_strategy!(i16);
}

/// Module for i32 strategies.
pub mod i32 {
    use super::*;
    int_strategy!(i32);
}

/// Module for i64 strategies.
pub mod i64 {
    use super::*;
    int_strategy!(i64);
}

/// Module for i128 strategies.
pub mod i128 {
    use super::*;
    int_strategy!(i128);
}

/// Module for isize strategies.
pub mod isize {
    use super::*;
    int_strategy!(isize);
}

/// Module for u8 strategies.
pub mod u8 {
    use super::*;
    int_strategy!(u8);
}

/// Module for u16 strategies.
pub mod u16 {
    use super::*;
    int_strategy!(u16);
}

/// Module for u32 strategies.
pub mod u32 {
    use super::*;
    int_strategy!(u32);
}

/// Module for u64 strategies.
pub mod u64 {
    use super::*;
    int_strategy!(u64);
}

/// Module for u128 strategies.
pub mod u128 {
    use super::*;
    int_strategy!(u128);
}

/// Module for usize strategies.
pub mod usize {
    use super::*;
    int_strategy!(usize);
}
