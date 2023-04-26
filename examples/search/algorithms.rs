/// Linear search algorithm
///
/// # Arguments
///
/// * `v`: sorted vector of T
/// * `val`: value to search for
///
/// # Returns
/// Index of val in v if found, None otherwise
pub fn linear_search<T: Ord>(v: &Vec<T>, val: T) -> Option<usize> {
    for i in 0..v.len() {
        if v[i] == val {
            return Some(i);
        }
    }
    None
}

/// Implementation of linear search algorithm for a vector of i8
pub fn linear_search_input(v: & InputVec, val: usize) -> Option<usize> {
    linear_search(v, val)
}

/// Binary search algorithm
///
/// # Arguments
/// * `v`: sorted vector of T
/// * `val`: value to search for
///
/// # Returns
/// Index of val in v if found, None otherwise
pub fn binary_search<T: Ord>(v: &Vec<T>, val: T) -> Option<usize> {
    let mut low = 0;
    let mut high = v.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if v[mid] == val {
            return Some(mid);
        } else if v[mid] < val {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

/// implementation of binary search for a vector of i8
pub fn binary_search_input(v: & InputVec, val: usize) -> Option<usize> {
    binary_search(v, val)
}