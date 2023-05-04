use crate::input::SearchInput;
use std::cmp::Ordering;

/// Linear search algorithm
///
/// # Arguments
///
/// * `v`: sorted vector of T
/// * `val`: value to search for
///
/// # Returns
/// Index of val in v if found, None otherwise
pub fn linear_search<T: Ord>(v: &[T], val: T) -> Option<usize> {
    for (i, item) in v.iter().enumerate() {
        if *item == val {
            return Some(i);
        }
    }
    None
}

/// Implementation of linear search algorithm for a vector of i8
pub fn linear_search_input(input: &SearchInput) -> Option<usize> {
    linear_search(&input.vector, input.target)
}

/// Binary search algorithm
///
/// # Arguments
/// * `v`: sorted vector of T
/// * `val`: value to search for
///
/// # Returns
/// Index of val in v if found, None otherwise
pub fn binary_search<T: Ord>(v: &[T], val: T) -> Option<usize> {
    let mut low = 0;
    let mut high = v.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        match v[mid].cmp(&val) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
        }
    }
    None
}

/// implementation of binary search for a vector of i8
pub fn binary_search_input(input: &SearchInput) -> Option<usize> {
    binary_search(&input.vector, input.target)
}
