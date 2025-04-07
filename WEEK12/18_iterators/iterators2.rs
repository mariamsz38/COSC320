// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
// Capitalize first letter of a string
// Capitalize the first letter of a string
// Capitalize the first letter of a string
#[allow(dead_code)]
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

// Capitalize multiple words and return a vector using iterator
#[allow(dead_code)]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// Capitalize multiple words and join into a single string with spaces
#[allow(dead_code)]
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter()
        .map(|&word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")  // Add space between the words
}

fn main() {
    // Example usage of the functions
    let test_words = vec!["hello", "world"];
    println!("{}", capitalize_first("hello"));  // "Hello"
    println!("{:?}", capitalize_words_vector(&test_words));  // ["Hello", "World"]
    println!("{}", capitalize_words_string(&test_words));  // "Hello World"
}

// Add this attribute to suppress dead_code warnings for test code
#[cfg(test)]
mod tests {
    // This allows dead code within the test module
    #![allow(dead_code)]
    
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
