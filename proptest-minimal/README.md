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

- **Supporting Types**:
  - `Reason` - Describes why a value was rejected
  - `TestRng` - Random number generator wrapper

- **no_std Support**: This crate can be used in `no_std` environments with `alloc`.

## What's NOT Included

This is a minimal core. It does NOT include:

- The full test runner infrastructure
- Built-in strategies for common types (integers, strings, collections, etc.)
- Strategy combinators (map, filter, flat_map, etc.)
- The `proptest!` macro
- Persistence/regression test support
- Fork/timeout features

For the complete proptest experience, use the main `proptest` crate.

## Features

- `default` = `["std"]`
- `std` - Enables standard library support
- `alloc` - Enables allocator support for `no_std` environments

## Usage

This crate is primarily intended for advanced users who want to build on proptest's foundation or understand its core architecture.

For normal property testing, use the main `proptest` crate instead.
