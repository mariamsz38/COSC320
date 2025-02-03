// Fixing the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string() // Convert the string slice to a String
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
