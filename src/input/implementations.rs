// TODO: maybe move this file into a new crate?

use crate::input::Input;

/// Implementation for some built-in data types.
/// If you want to implement Input for a data types defined outside your crate you can either create a wrapper or use this macro.
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

// Implementation for some built-in data types.

// Vec<i32>
impl_input!(Vec<i32>, |v: Vec<i32>| v.len());
/// Vec<u8> builder.
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
/// let v = Vec::<i32>::generate_input(10, vec_u8_builder);
/// ```
fn vec_u8_builder (size: usize) -> Vec<i32> {
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rand::random());
    }
    v
}
