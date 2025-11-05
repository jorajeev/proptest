//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for primitive types.

use crate::arbitrary::Arbitrary;
use crate::bool;
use crate::num;

// Macro to implement Arbitrary for integer types
macro_rules! impl_arbitrary_int {
    ($type:ty, $mod:ident) => {
        impl Arbitrary for $type {
            type Parameters = ();
            type Strategy = $mod::Any;

            fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
                $mod::ANY
            }
        }
    };
}

impl_arbitrary_int!(i8, i8);
impl_arbitrary_int!(i16, i16);
impl_arbitrary_int!(i32, i32);
impl_arbitrary_int!(i64, i64);
impl_arbitrary_int!(i128, i128);
impl_arbitrary_int!(isize, isize);

impl_arbitrary_int!(u8, u8);
impl_arbitrary_int!(u16, u16);
impl_arbitrary_int!(u32, u32);
impl_arbitrary_int!(u64, u64);
impl_arbitrary_int!(u128, u128);
impl_arbitrary_int!(usize, usize);

impl Arbitrary for bool {
    type Parameters = ();
    type Strategy = bool::Any;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        bool::ANY
    }
}
