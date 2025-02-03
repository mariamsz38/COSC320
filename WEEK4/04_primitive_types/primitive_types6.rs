fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Access the second element of the tuple using index 1
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
