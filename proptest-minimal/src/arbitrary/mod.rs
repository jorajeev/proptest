//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Defines the `Arbitrary` trait for types that can be randomly generated.
//!
//! The `Arbitrary` trait provides a standard way to generate random values
//! of a type for property testing.

use core::fmt;
use crate::strategy::Strategy;

mod primitives;

//==============================================================================
// Arbitrary trait
//==============================================================================

/// Arbitrary determines a canonical [`Strategy`] for the implementing type.
///
/// It provides the method `arbitrary_with` which generates a `Strategy` for
/// producing arbitrary values of the implementing type *(`Self`)*. In general,
/// these strategies will produce the entire set of values possible for the
/// type, up to some size limitation or constraints set by their parameters.
///
/// This trait analogous to
/// [Haskell QuickCheck's implementation of `Arbitrary`][HaskellQC].
/// In this interpretation of `Arbitrary`, `Strategy` is the equivalent of
/// the `Gen` monad.
///
/// `Arbitrary` currently only works for types which represent owned data as
/// opposed to borrowed data. This is a fundamental restriction of `proptest`.
///
/// [HaskellQC]:
/// https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html
pub trait Arbitrary: Sized + fmt::Debug {
    /// The type of parameters that [`arbitrary_with`] accepts for configuration
    /// of the generated [`Strategy`]. Parameters must implement [`Default`].
    ///
    /// [`arbitrary_with`]: trait.Arbitrary.html#tymethod.arbitrary_with
    type Parameters: Default;

    /// Generates a [`Strategy`] for producing arbitrary values
    /// of type the implementing type (`Self`).
    ///
    /// Calling this for the type `X` is the equivalent of using
    /// [`X::arbitrary_with(Default::default())`].
    ///
    /// This method is defined in the trait for optimization for the
    /// default if you want to do that. It is a logic error to not
    /// preserve the semantics when overriding.
    ///
    /// [`X::arbitrary_with(Default::default())`]:
    ///     trait.Arbitrary.html#tymethod.arbitrary_with
    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with(Default::default())
    }

    /// Generates a [`Strategy`] for producing arbitrary values of type the
    /// implementing type (`Self`). The strategy is passed the arguments given
    /// in args.
    ///
    /// If you wish to use the [`default()`] arguments,
    /// use [`arbitrary`] instead.
    ///
    /// [`arbitrary`]: trait.Arbitrary.html#method.arbitrary
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;

    /// The type of [`Strategy`] used to generate values of type `Self`.
    type Strategy: Strategy<Value = Self>;
}

//==============================================================================
// Type aliases for associated types
//==============================================================================

/// `StrategyFor` allows you to mention the type of [`Strategy`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break. This can be especially beneficial when the
/// type of `Strategy` that you are dealing with is very long in name
/// (the case with generics).
pub type StrategyFor<A> = <A as Arbitrary>::Strategy;

/// `ParamsFor` allows you to mention the type of [`Parameters`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break.
///
/// [`Parameters`]: trait.Arbitrary.html#associatedtype.Parameters
pub type ParamsFor<A> = <A as Arbitrary>::Parameters;

//==============================================================================
// Free functions
//==============================================================================

/// Generates a [`Strategy`] producing [`Arbitrary`][trait Arbitrary] values of
/// `A`. Unlike [`arbitrary`][fn arbitrary], it should be used for being
/// explicit on what `A` is. For clarity, this may be a good idea.
///
/// Use this version instead of [`arbitrary`][fn arbitrary] if you want to be
/// clear which type you want to generate a `Strategy` for, or if you don't
/// have an anchoring type for type inference to work with.
///
/// If you want to customize how the strategy is generated, use
/// [`any_with::<A>(args)`] where `args` are any arguments accepted by
/// the `Arbitrary` impl in question.
///
/// # Example
///
/// ```
/// # #![allow(dead_code)]
/// use proptest_minimal::prelude::*;
///
/// // Example usage (requires implementations):
/// // fn test_function(value: u32) { /* ... */ }
/// //
/// // // Generate arbitrary u32 values
/// // let strategy = any::<u32>();
/// ```
///
/// [`any_with::<A>(args)`]: fn.any_with.html
/// [fn arbitrary]: fn.arbitrary.html
/// [trait Arbitrary]: trait.Arbitrary.html
#[must_use = "strategies do nothing unless used"]
pub fn any<A: Arbitrary>() -> StrategyFor<A> {
    A::arbitrary()
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A` with the
/// given configuration arguments passed in `args`. Unlike [`arbitrary_with`],
/// it should be used for being explicit on what `A` is.
/// For clarity, this may be a good idea.
///
/// Use this version instead of [`arbitrary_with`] if you want to be clear which
/// type you want to generate a `Strategy` for, or if you don't have an anchoring
/// type for type inference to work with.
///
/// If you don't want to specify any arguments and instead use the default
/// behavior, you should use [`any::<A>()`].
///
/// [`any::<A>()`]: fn.any.html
/// [`arbitrary_with`]: trait.Arbitrary.html#tymethod.arbitrary_with
#[must_use = "strategies do nothing unless used"]
pub fn any_with<A: Arbitrary>(args: ParamsFor<A>) -> StrategyFor<A> {
    A::arbitrary_with(args)
}
