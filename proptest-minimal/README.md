# proptest-minimal

Minimal examples demonstrating property-based testing with proptest.

## Overview

This crate provides simple, focused examples that demonstrate how to use proptest for property-based testing. Each example tests a simple `sum_vector` function and shows different proptest features.

## Examples

### 1. Basic Property Test (`basic_sum_test.rs`)

Demonstrates the simplest form of property-based testing with proptest:
- Generates random vectors of integers
- Tests a basic property (sum should not equal 10)
- Shows how proptest finds and shrinks counterexamples

```bash
cargo test --example basic_sum_test
```

### 2. Custom Strategies (`custom_strategy_test.rs`)

Shows how to create and use custom strategies:
- Defines custom generators for test data
- Demonstrates inline strategy definitions
- Tests with constrained input ranges

```bash
cargo test --example custom_strategy_test
```

### 3. Filtered Tests (`filtered_test.rs`)

Demonstrates filtering and assumptions:
- Uses `prop_assume!` to skip unwanted test cases
- Shows how to filter based on element properties
- Illustrates the difference between filtering and generation

```bash
cargo test --example filtered_test
```

## Note on Test Failures

These examples intentionally test a property that will likely fail (sum should not equal 10). This demonstrates proptest's ability to:
1. Find counterexamples automatically
2. Shrink them to minimal failing cases
3. Make it easy to reproduce failures

In real-world usage, you would test properties that should actually hold true for your code.

## Running Examples

To see the output of an example without running tests:
```bash
cargo run --example basic_sum_test
```

To run the property tests:
```bash
cargo test --example basic_sum_test
```
