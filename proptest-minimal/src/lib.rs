//! Minimal examples for proptest property-based testing.
//!
//! This crate provides simple examples demonstrating the basic usage
//! of proptest for property-based testing.

/// A simple function that sums all elements in a vector.
///
/// This is used in the examples to demonstrate property-based testing.
pub fn sum_vector(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vector_basic() {
        assert_eq!(sum_vector(&[1, 2, 3]), 6);
        assert_eq!(sum_vector(&[]), 0);
        assert_eq!(sum_vector(&[-1, 1]), 0);
    }
}
