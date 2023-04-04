use rand::{thread_rng, Rng};
use serde::Serialize;
use std::ops::Deref;

#[derive(Clone, Serialize)]
/// A rapresentation of the function that generates a distribution of lengths of strings
pub struct LengthDistributionFunction {
    pub name: &'static str, // todo: is the name useful?
    #[serde(skip_serializing)]
    pub function: fn(n: usize, min: f64, max: f64) -> Vec<usize>,
}

/// Uniform distribution of lengths
pub const UNIFORM: LengthDistributionFunction = LengthDistributionFunction {
    name: "Uniform",
    function: uniform_length_set,
};

/// Exponential distribution of lengths
pub const EXPONENTIAL: LengthDistributionFunction = LengthDistributionFunction {
    name: "Exponential",
    function: exponential_length_set,
};

/// Uniform random distribution of lengths
pub const UNIFORM_RANDOM: LengthDistributionFunction = LengthDistributionFunction {
    name: "Uniform random",
    function: uniform_random_length_set,
};

/// Exponential random distribution of lengths
pub const EXPONENTIAL_RANDOM: LengthDistributionFunction = LengthDistributionFunction {
    name: "Exponential random",
    function: exponential_random_length_set,
};

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

impl Deref for LengthDistributionFunction {
    type Target = fn(n: usize, min: f64, max: f64) -> Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

#[derive(Clone, Serialize)]
pub struct LengthDistribution {
    pub length_distribution_fn: LengthDistributionFunction,
    pub min_value: f64,
    pub max_value: f64,
}

impl LengthDistribution {
    /// Creates a new distribution
    ///
    /// # Arguments
    ///
    /// * `length_distribution` - The distribution of the lengths of the strings
    /// * `min_value` - The minimum value of the length of the strings
    /// * `max_value` - The maximum value of the length of the strings
    ///
    /// # Examples
    ///
    /// ```
    /// use time_complexity_plot::random::lengths::{LengthDistribution, EXPONENTIAL};
    ///
    /// let length_distribution = LengthDistribution::new(EXPONENTIAL, 1000, 500_000);
    /// ```
    pub fn new(
        length_distribution_fn: LengthDistributionFunction,
        min_value: i32,
        max_value: i32,
    ) -> Self {
        LengthDistribution {
            length_distribution_fn,
            min_value: min_value as f64,
            max_value: max_value as f64,
        }
    }

    /// Creates a vector of lengths of strings using the distribution specified in the struct
    ///
    /// # Arguments
    ///
    /// * `n` - The number of lengths to be generated
    ///
    /// # Panics
    ///
    /// * Panics if the number of lengths to be generated is less than 1
    pub fn create_length_set(&self, n: usize) -> Vec<usize> {
        assert!(
            n > 0,
            "The number of lengths to be generated must be greater than 0"
        );
        (self.length_distribution_fn.function)(n, self.min_value, self.max_value)
    }
}
