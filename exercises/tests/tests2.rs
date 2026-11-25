// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // assert_eq! wants two values of the same type that can be compared;
        // giving it 1 and 1 makes it a no-op, so the test passes.
        assert_eq!(1, 1);

        // The other experiments the hint suggests are all compile errors or
        // panics, which is exactly why they stay commented out:
        //   assert_eq!(1, 2);          -> panics, test fails
        //   assert_eq!(1, "1");        -> mismatched types, won't compile
        //   assert_eq!("1", 1);        -> argument order matters for PartialEq
    }
}
