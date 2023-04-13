// SOME STANDARD LENGTH DISTRIBUTION FUNCTIONS

use rand::{thread_rng, Rng};

// TODO: is it right that we cannot create a DistribuitionBuilder without min and max? Maybe we can set it in .build()?
/// Uniform distribution of lengths
pub const UNIFORM: fn(n: usize, min: f64, max: f64) -> Vec<usize> = uniform_length_set;

/// Exponential distribution of lengths
pub const EXPONENTIAL: fn(n: usize, min: f64, max: f64) -> Vec<usize> = exponential_length_set;

/// Uniform random distribution of lengths
pub const UNIFORM_RANDOM: fn(n: usize, min: f64, max: f64) -> Vec<usize> =
    uniform_random_length_set;

/// Exponential random distribution of lengths
pub const EXPONENTIAL_RANDOM: fn(n: usize, min: f64, max: f64) -> Vec<usize> =
    exponential_random_length_set;

/// Creates a vector of lengths of strings using an uniform distribution
///
/// # Arguments
///
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn uniform_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    let a = min;
    let b = (max - min) / n as f64;
    for i in 0..n {
        let x = a + b * (i as f64);
        let final_x = x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

/// Creates a vector of lengths of strings using an exponential distribution
///
/// # Arguments
///
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn exponential_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    let a = min;
    let b = (max / min).powf(1.0 / n as f64);
    for i in 0..n {
        let x = a * b.powf(i as f64);
        let final_x = x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

/// Creates a vector of lengths of strings using an uniform random distribution
///
/// # Arguments
///
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn uniform_random_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    for _ in 0..n {
        let x = thread_rng().gen_range(min..max);
        let final_x = x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

/// Creates a vector of lengths of strings using an exponential random distribution
///
/// # Arguments
///
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn exponential_random_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    for _ in 0..n {
        let x: f64 = thread_rng().gen::<f64>();
        let scaled_x = min * (max / min).powf(x);
        let final_x = scaled_x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}