// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    let test_values = [1, 2, 3, 4, 5, 6];
    for &value in test_values.iter() {
        if is_even(value) {
            println!("{} is even", value);
        } else {
            println!("{} is odd", value);
        }}
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;

    fn test_is_even_case(n: i64, expected: bool) {
        assert_eq!(is_even(n), expected, "Failed on input: {}", n);
    }


    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2));  // 2 is even, should pass
        assert!(!is_even(3)); // 3 is odd, should fail if incorrectly implemented
    }
}