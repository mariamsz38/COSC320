struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
    let valid_rect1 = Rectangle::new(5, 10);
    let valid_rect2 = Rectangle::new(3, 7);
    let valid_rect3 = Rectangle::new(8, 8);

    println!("Valid Rectangles:");
    println!("Rectangle 1: Width = {}, Height = {}", valid_rect1.width, valid_rect1.height);
    println!("Rectangle 2: Width = {}, Height = {}", valid_rect2.width, valid_rect2.height);
    println!("Rectangle 3: Width = {}, Height = {}", valid_rect3.width, valid_rect3.height);
    println!("\nAttempting to create invalid rectangles...");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: This test should check if the rectangle has the size that we
        // pass to its constructor.
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);  // Check width
        assert_eq!(rect.height, 20); // Check height

    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative width.
    #[test]
    fn negative_width() {
        #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }
}
}
    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative height.
    #[test]
    fn negative_height() {
        #[should_panic(expected = "Rectangle width and height must be positive")]
        fn negative_height() {
            let _rect = Rectangle::new(10, -10);
        }
}