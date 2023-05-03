use crate::input::InputVec;

pub fn merge_sort_input(v: &mut InputVec) {
    merge_sort(v)
}

pub fn merge_sort<T: Ord + Clone>(v: &mut Vec<T>) {
    let n = v.len();
    if n > 1 {
        let mid = n / 2;
        let mut left = v[..mid].to_vec();
        let mut right = v[mid..].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);
        merge(v, &left, &right);
    }
}

fn merge<T: Ord + Clone>(v: &mut [T], left: &Vec<T>, right: &Vec<T>) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            v[k] = left[i].clone();
            i += 1;
        } else {
            v[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        v[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        v[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

pub fn quick_sort_input(v: &mut InputVec) {
    quick_sort(v)
}

pub fn quick_sort<T: Ord + Clone>(v: &mut Vec<T>) {
    quick_sort_rec(v, 0, v.len() - 1);
}

fn quick_sort_rec<T: Ord + Clone>(v: &mut Vec<T>, low: usize, high: usize) {
    if low < high {
        let p = partition(v, low, high);
        quick_sort_rec(v, low, p - 1);
        quick_sort_rec(v, p + 1, high);
    }
}

fn partition<T: Ord + Clone>(v: &mut [T], low: usize, high: usize) -> usize {
    let pivot = v[high].clone();
    let mut i = low;
    for j in low..high {
        if v[j] < pivot {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, high);
    i
}