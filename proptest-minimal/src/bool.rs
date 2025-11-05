//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Strategies for generating boolean values.

use crate::strategy::{NewTree, Strategy, ValueTree};
use crate::test_runner::TestRng;
use rand::Rng;

/// A `ValueTree` for bool values.
///
/// Shrinks `true` to `false`.
#[derive(Debug, Clone, Copy)]
pub struct BoolValueTree {
    current: bool,
    state: BoolValueTreeState,
}

#[derive(Debug, Clone, Copy)]
enum BoolValueTreeState {
    /// No simplification has been performed yet.
    Start,
    /// A simplification has been performed, so no further simplification is possible.
    Simplified,
}

impl BoolValueTree {
    fn new(value: bool) -> Self {
        BoolValueTree {
            current: value,
            state: BoolValueTreeState::Start,
        }
    }
}

impl ValueTree for BoolValueTree {
    type Value = bool;

    fn current(&self) -> bool {
        self.current
    }

    fn simplify(&mut self) -> bool {
        match (self.current, self.state) {
            (true, BoolValueTreeState::Start) => {
                self.current = false;
                self.state = BoolValueTreeState::Simplified;
                true
            }
            _ => false,
        }
    }

    fn complicate(&mut self) -> bool {
        match self.state {
            BoolValueTreeState::Simplified => {
                self.current = true;
                self.state = BoolValueTreeState::Start;
                true
            }
            _ => false,
        }
    }
}

/// Strategy that generates boolean values.
///
/// Shrinks `true` to `false`.
#[derive(Debug, Clone, Copy)]
#[must_use = "strategies do nothing unless used"]
pub struct Any(());

/// Constant for generating any boolean value.
pub const ANY: Any = Any(());

impl Strategy for Any {
    type Tree = BoolValueTree;
    type Value = bool;

    fn new_tree(&self, rng: &mut TestRng) -> NewTree<Self> {
        Ok(BoolValueTree::new(rng.random()))
    }
}
