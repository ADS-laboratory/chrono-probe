// SOME USEFUL INPUT IMPLEMENTATIONS

pub mod distribution;
use rand::{thread_rng, Rng};

use crate::input::Input;

/// Implements Input for the given type using the given closure to get the size of the input.
/// Useful for not having to create a wrapper for built-in data types.
/// 
/// # Arguments
/// 
/// * `$built_in_type` - The type to implement [Input] for.
/// * `$closure` - The closure that will be used to get the size of the input.
/// 
/// # Example
/// 
/// ```
/// impl_input!(Vec<i32>, |v: Vec<i32>| v.len());
/// ```
#[macro_export]
macro_rules! impl_input {
    ($built_in_type:ty, $closure:expr) => {
        /// Implementation of "Input" for $built_in_type
        impl Input for $built_in_type {
            type Builder = fn(usize) -> Self;

            fn get_size(&self) -> usize {
                $closure(self.clone())
            }

            fn generate_input(size: usize, builder: Self::Builder) -> Self {
                builder(size)
            }
        }
    };
}


// IMPLEMENTATIONS:
impl_input!(Vec<i32>, |v: Vec<i32>| v.len());
impl_input!(String, |s: String| s.len());


// CONSTANTS:

/// Vector of i32
pub const VEC_U8_BUILDER: fn(usize) -> Vec<i32> = vec_i32_builder;

/// String builder - method 1
/// Note: you cannot use this directly as a builder, you need to set it the char_set (&Vec<char>) first.
pub const STRING_1_BUILDER: fn(usize, &Vec<char>) -> String = create_random_string1;

/// String builder - method 2
/// Note: you cannot use this directly as a builder, you need to set it the char_set (&Vec<char>) first.
pub const STRING_2_BUILDER: fn(usize, &Vec<char>) -> String = create_random_string2;

/// String builder - method 3
/// Note: you cannot use this directly as a builder, you need to set it the char_set (&Vec<char>) first.
pub const STRING_3_BUILDER: fn(usize, &Vec<char>) -> String = create_random_string3;

/// String builder - method 4
/// Note: you cannot use this directly as a builder, you need to set it the char_set (&Vec<char>) first.
pub const STRING_4_BUILDER: fn(usize, &Vec<char>) -> String = create_random_string4;


// FUNCTIONS:

/// Vec<i32> builder.
/// 
/// # Arguments
/// 
/// * `size` - The size of the vector.
/// 
/// # Returns
/// 
/// A vector of size `size` with random values.
/// 
/// # Example
/// 
/// ```
/// let v = Vec::<i32>::generate_input(10, vec_i32_builder);
/// ```
fn vec_i32_builder (size: usize) -> Vec<i32> {
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rand::random());
    }
    v
}

/// String builder - method 1.
/// 
/// # Arguments
/// 
/// * `size` - The size of the string.
/// * `char_set` - The set of characters to use.
/// 
/// # Returns
/// 
/// A string of size `size` with random characters from `char_set`.
fn create_random_string1(n: usize, char_set: &Vec<char>) -> String {
    let mut s = String::with_capacity(n);
    let number_of_chars = char_set.len();
    for _ in 0..n {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char);
    }
    s
}

fn create_random_string2(n: usize, char_set: &Vec<char>) -> String {
    let mut s: Vec<u8> = Vec::with_capacity(n);
    let number_of_chars = char_set.len();
    let q = thread_rng().gen_range(0..n);
    for _ in 0..q {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char as u8);
    }
    for i in q..n {
        let char = s[(i - 1) % (q + 1)];
        s.push(char);
    }
    String::from_utf8(s).unwrap()
}

fn create_random_string3(_n: usize, _char_set: &Vec<char>) -> String {
    todo!()
}

fn create_random_string4(n: usize, char_set: &Vec<char>) -> String {
    let mut s = String::with_capacity(n);
    let number_of_chars = char_set.len();
    let mut char = char_set[0];
    for i in 0..n - 1 {
        char = char_set[i % number_of_chars];
        s.push(char);
    }
    s.push(char);
    s
}