//! Fixture for the action's own CI: one covered line and one that is not.

/// Adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Never called by the tests, so the report has an uncovered line to gate on.
pub fn untouched() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds() {
        assert_eq!(super::add(2, 3), 5);
    }
}
