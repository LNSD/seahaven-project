//! # Seahaven configuration
//!
//! This crate provides a configuration for the Seahaven project.

/// Add two `u64` numbers
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn add_two_numbers() {
        //* Given
        let left = 2;
        let right = 2;

        //* When
        let result = add(left, right);

        //* Then
        assert_eq!(result, 4);
    }
}
