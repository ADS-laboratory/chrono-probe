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