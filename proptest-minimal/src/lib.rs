//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Proptest Minimal
//!
//! This crate provides the minimal core of the proptest property testing framework.
//! It contains only the essential `Strategy` and `ValueTree` traits and basic
//! supporting types, without the full set of strategy implementations and test
//! runner infrastructure.
//!
//! For the complete proptest framework, see the `proptest` crate.

#![forbid(future_incompatible)]
#![deny(missing_docs, bare_trait_objects)]
#![no_std]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[macro_use]
extern crate alloc;

#[doc(hidden)]
pub mod std_facade;

pub mod strategy;
pub mod test_runner;
pub mod arbitrary;
pub mod num;
pub mod bool;

/// Re-export of commonly used items.
pub mod prelude {
    pub use crate::strategy::{Just, NewTree, Strategy, ValueTree};
    pub use crate::test_runner::{Reason, TestRng};
    pub use crate::arbitrary::{any, any_with, Arbitrary, StrategyFor};
}
