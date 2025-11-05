//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module provides #[cfg(..)]ed type aliases over features.

macro_rules! multiplex_alloc {
    ($($alloc: path, $std: path),*) => {
        $(
            #[cfg(all(feature = "alloc", not(feature = "std")))]
            pub use $alloc;
            #[cfg(feature = "std")]
            pub use $std;
        )*
    };
}

macro_rules! multiplex_core {
    ($($core: path, $std: path),*) => {
        $(
            #[cfg(not(feature = "std"))]
            pub use $core;
            #[cfg(feature = "std")]
            pub use $std;
        )*
    };
}

multiplex_alloc! {
    alloc::borrow::Cow, ::std::borrow::Cow,
    alloc::boxed::Box, ::std::boxed::Box,
    alloc::string::String, ::std::string::String,
    alloc::sync::Arc, ::std::sync::Arc,
    alloc::rc::Rc, ::std::rc::Rc
}

multiplex_core! {
    core::fmt, ::std::fmt
}
