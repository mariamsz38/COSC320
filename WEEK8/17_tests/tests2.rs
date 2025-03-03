// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
    let test_values = [0, 1, 2, 3, 4, 5, 10];

    for &value in test_values.iter() {
        println!("2 to the power of {} is {}", value, power_of_2(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        assert_eq!(power_of_2(0), 1);  // 2^0 = 1
        assert_eq!(power_of_2(1), 2);  // 2^1 = 2
        assert_eq!(power_of_2(2), 4);  // 2^2 = 4
        assert_eq!(power_of_2(3), 8);  // 2^3 = 8
        assert_eq!(power_of_2(4), 16); // 2^4 = 16
    }
}