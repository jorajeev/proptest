//! Example using custom strategies with proptest.
//!
//! This example shows how to create custom strategies to generate
//! test data with specific constraints.
//!
//! Run with: cargo run --example custom_strategy_test

use proptest::prelude::*;
use proptest_minimal::sum_vector;

/// Custom strategy that generates vectors of small positive integers (1..=5)
fn small_positive_vec() -> impl Strategy<Value = Vec<i32>> {
    prop::collection::vec(1..=5i32, 1..10)
}

proptest! {
    /// Property test using a custom strategy.
    ///
    /// This test generates vectors containing only small positive integers (1-5).
    /// It still tests that the sum doesn't equal 10, which will likely fail
    /// with inputs like [5, 5] or [2, 3, 5].
    #[test]
    fn test_sum_not_ten_small_positive(vec in small_positive_vec()) {
        let sum = sum_vector(&vec);
        prop_assert_ne!(
            sum, 10,
            "Found a vector of small positive numbers that sums to 10: {:?}",
            vec
        );
    }

    /// Another property test with a different range.
    ///
    /// This uses inline strategy definition with numbers from -10 to 10.
    #[test]
    fn test_sum_not_ten_range(vec in prop::collection::vec(-10..=10i32, 0..20)) {
        let sum = sum_vector(&vec);
        prop_assert_ne!(
            sum, 10,
            "Found a vector in range [-10,10] that sums to 10: {:?}",
            vec
        );
    }
}

fn main() {
    println!("This example demonstrates custom strategies in proptest.");
    println!("Run with: cargo test --example custom_strategy_test");
    println!("\nCustom strategies allow you to generate test data with");
    println!("specific constraints relevant to your domain.");
}
