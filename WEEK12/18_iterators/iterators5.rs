// Let's define a simple model to track Rustlings' exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try to not use imperative loops (for/while).

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

// Implementing the counting using an iterator
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|&&v| v == value).count()
}

// Implementing the counting for a collection of HashMaps using an iterator
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection
        .iter()  // Iterate over each HashMap in the collection
        .map(|map| map.values().filter(|&&v| v == value).count()) // For each map, count matching progress values
        .sum() // Sum up the counts from all HashMaps
}

// These functions are used to generate test data for both tests and main
fn get_map() -> HashMap<String, Progress> {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("variables1"), Complete);
    map.insert(String::from("functions1"), Complete);
    map.insert(String::from("hashmap1"), Complete);
    map.insert(String::from("arc1"), Some);
    map.insert(String::from("as_ref_mut"), None);
    map.insert(String::from("from_str"), None);

    map
}

fn get_vec_map() -> Vec<HashMap<String, Progress>> {
    use Progress::*;

    let map = get_map();

    let mut other = HashMap::new();
    other.insert(String::from("variables2"), Complete);
    other.insert(String::from("functions2"), Complete);
    other.insert(String::from("if1"), Complete);
    other.insert(String::from("from_into"), None);
    other.insert(String::from("try_from_into"), None);

    vec![map, other]
}

fn main() {
    let map = get_map();
    println!("Complete count: {}", count_iterator(&map, Progress::Complete));
    println!("Some count: {}", count_iterator(&map, Progress::Some));
    println!("None count: {}", count_iterator(&map, Progress::None));

    let collection = get_vec_map();
    println!(
        "Collection Complete count: {}",
        count_collection_iterator(&collection, Progress::Complete)
    );
    println!(
        "Collection Some count: {}",
        count_collection_iterator(&collection, Progress::Some)
    );
    println!(
        "Collection None count: {}",
        count_collection_iterator(&collection, Progress::None)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_iterator(&collection, Progress::Complete),
            6,
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
        }
    }
}
