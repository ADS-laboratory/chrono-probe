use rand::{thread_rng, Rng};
use serde::Serialize;

/// Distribution of the lengths of the strings
pub struct DistributionSet {
    // Vector of the lengths of the strings
    pub lengths: Vec<usize>,
}

/// Struct that let you build the [DistributionSet]
#[derive(Serialize)]
pub struct DistributionBuilder {
    #[serde(skip_serializing)]
    pub length_distribution_fn: fn(n: usize, min: f64, max: f64) -> Vec<usize>,
    pub(crate) length_distribution_name: String,
    pub min_value: f64,
    pub max_value: f64,
}

impl DistributionBuilder {
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
    /// use time_complexity_plot::random::lengths::{Distribution, EXPONENTIAL};
    ///
    /// let length_distribution = DistributionBuilder::new(EXPONENTIAL, 1000, 500_000);
    /// ```
    pub fn new(
        length_distribution_fn: fn(n: usize, min: f64, max: f64) -> Vec<usize>,
        min_value: i32,
        max_value: i32,
    ) -> Self {
        DistributionBuilder {
            length_distribution_fn,
            length_distribution_name: format!("{:?}", length_distribution_fn),
            min_value: min_value as f64,
            max_value: max_value as f64,
        }
    }

    /// Creates a [DistributionSet] that contains the vector of input lengths generated using the distribution function.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of lengths to be generated
    ///
    /// # Panics
    ///
    /// * Panics if the number of lengths to be generated is less than 1
    pub fn build(&self, n: usize) -> DistributionSet {
        assert!(
            n > 0,
            "The number of lengths to be generated must be greater than 0"
        );
        DistributionSet {
            lengths: (self.length_distribution_fn)(n, self.min_value, self.max_value),
        }
    }
}

// Some distributions
// TODO: move these const below to an example?

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
