//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core strategy traits for property-based testing.
//!
//! This module contains the fundamental `Strategy` and `ValueTree` traits that
//! form the foundation of proptest's architecture.

mod traits;

pub use self::traits::{NewTree, Strategy, ValueTree};
