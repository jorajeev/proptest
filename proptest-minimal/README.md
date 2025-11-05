# proptest-minimal

A minimal core of the proptest property testing framework, containing only the essential `Strategy` and `ValueTree` traits and basic supporting types.

## Purpose

This crate extracts the fundamental abstractions from proptest into a lightweight, minimal package with fewer dependencies. It's intended for:

- Building custom property testing tools on top of proptest's core architecture
- Understanding the fundamental concepts of proptest
- Using in environments where the full proptest dependency tree is too heavy

## What's Included

- **Core Traits**:
  - `Strategy` - The fundamental trait for generating and shrinking values
  - `ValueTree` - Represents a generated value and its simplifications
  - `Arbitrary` - Trait for types that can be randomly generated

- **Basic Strategies**:
  - `Just` - Strategy that always produces the same value
  - `LazyJust` - Lazy version of Just for non-Clone types
  - Numeric strategies with binary search shrinking for all integer types (i8-i128, u8-u128, isize, usize)
  - Boolean strategy

- **Arbitrary Implementations**:
  - All primitive integer types (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize)
  - Boolean type

- **Supporting Types**:
  - `Reason` - Describes why a value was rejected
  - `TestRng` - Random number generator wrapper
  - `BinarySearch` - ValueTree that shrinks numbers toward 0

- **no_std Support**: This crate can be used in `no_std` environments with `alloc`.

## What's NOT Included

This is a minimal core. It does NOT include:

- The full test runner infrastructure with Config, failure persistence, etc.
- String and character strategies
- Collection strategies (Vec, HashMap, etc.)
- Tuple strategies
- Option and Result strategies
- Advanced strategy combinators (map, filter, flat_map, etc.)
- The `proptest!` macro
- Regression test persistence
- Fork/timeout features
- Floating point strategies

For the complete proptest experience, use the main `proptest` crate.

## Features

- `default` = `["std"]`
- `std` - Enables standard library support
- `alloc` - Enables allocator support for `no_std` environments

## Usage

This crate is primarily intended for advanced users who want to build on proptest's foundation or understand its core architecture.

### Example

```rust
use proptest_minimal::prelude::*;

// Use the Arbitrary trait to generate values
fn my_test() {
    let mut rng = TestRng::deterministic_rng();
    let strategy = any::<u32>();
    let value_tree = strategy.new_tree(&mut rng).unwrap();
    let value = value_tree.current();

    // Use the value in your test...
    println!("Generated value: {}", value);
}

// Create custom strategies
fn custom_even_numbers() {
    let mut rng = TestRng::deterministic_rng();
    // Use range strategies from the num module
    let strategy = 0u32..100;
    let value_tree = strategy.new_tree(&mut rng).unwrap();
    let value = value_tree.current();

    println!("Generated value in range: {}", value);
}
```

For complete property testing with the `proptest!` macro and more features, use the main `proptest` crate instead.
