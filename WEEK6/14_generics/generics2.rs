// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper <T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    // Demonstrate usage of the Wrapper struct
    let int_wrapper = Wrapper::new(42); // Wrapping an integer
    println!("Wrapped integer: {}", int_wrapper.value);

    let string_wrapper = Wrapper::new("Foo"); // Wrapping a string
    println!("Wrapped string: {}", string_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}