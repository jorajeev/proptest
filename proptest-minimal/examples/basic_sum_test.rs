//! Basic example of property-based testing with proptest.
//!
//! This example demonstrates a simple property test that generates
//! random vectors of integers and tests that their sum doesn't equal 10.
//!
//! Run with: cargo run --example basic_sum_test

use proptest::prelude::*;
use proptest_minimal::sum_vector;

proptest! {
    /// Property test: The sum of a vector of integers should not equal 10.
    ///
    /// Note: This property will likely fail! Proptest will find a
    /// counterexample (e.g., [10] or [5, 5]) and shrink it to the
    /// smallest failing case.
    #[test]
    fn test_sum_not_ten(vec in prop::collection::vec(any::<i32>(), 0..100)) {
        let sum = sum_vector(&vec);
        prop_assert_ne!(sum, 10, "Found a vector that sums to 10: {:?}", vec);
    }
}

fn main() {
    println!("This example demonstrates property-based testing with proptest.");
    println!("Run with: cargo test --example basic_sum_test");
    println!("\nThe test will likely fail, showing how proptest finds and");
    println!("shrinks counterexamples to the smallest failing case.");
}
