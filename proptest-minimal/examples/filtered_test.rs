//! Example using filters and assumptions with proptest.
//!
//! This example demonstrates how to use prop_assume! to filter out
//! test cases that don't meet certain criteria.
//!
//! Run with: cargo run --example filtered_test

use proptest::prelude::*;
use proptest_minimal::sum_vector;

proptest! {
    /// Property test with filtered inputs using prop_assume!
    ///
    /// This test generates vectors but skips (via prop_assume!) any that
    /// contain the value 10, since such vectors could easily sum to 10.
    ///
    /// Even with this filter, the test will likely still fail because
    /// there are many other combinations that sum to 10 (e.g., [5, 5]).
    #[test]
    fn test_sum_not_ten_filtered(vec in prop::collection::vec(-20..=20i32, 1..15)) {
        // Skip test cases where the vector contains 10
        prop_assume!(!vec.contains(&10));

        // Skip test cases where vector is empty (though technically valid)
        prop_assume!(!vec.is_empty());

        let sum = sum_vector(&vec);
        prop_assert_ne!(
            sum, 10,
            "Found a vector (without 10 in it) that sums to 10: {:?}",
            vec
        );
    }

    /// Another example showing assumptions about vector properties.
    ///
    /// This test only runs on vectors with at least 2 elements where
    /// all elements are even numbers.
    #[test]
    fn test_sum_not_ten_even_only(vec in prop::collection::vec(-10..=10i32, 2..8)) {
        // Only test vectors where all elements are even
        prop_assume!(vec.iter().all(|&x| x % 2 == 0));

        let sum = sum_vector(&vec);
        prop_assert_ne!(
            sum, 10,
            "Found a vector of even numbers that sums to 10: {:?}",
            vec
        );
    }
}

fn main() {
    println!("This example demonstrates filtering and assumptions in proptest.");
    println!("Run with: cargo test --example filtered_test");
    println!("\nUse prop_assume! to skip test cases that don't meet your criteria,");
    println!("but be careful not to filter out too many cases!");
}
