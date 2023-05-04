use rand::{thread_rng, Rng};
use std::{fmt::Display, ops::RangeInclusive};

// =====================
// = THE MODULE ITSELF =
// =====================

/// This trait defines a Distribution in an abstract way.
///
/// Without implementing lower level mechanisms this trait defines the shared behaviour of a
/// distribution, i.e. the property of being able to generate the input sizes.
pub trait Distribution: Display {
    fn generate(&self, n: usize) -> Vec<usize>;
}

// ==============================
// = PREDEFINED IMPLEMENTATIONS =
// ==============================

/// The struct representing an uniform distribution.
///
/// Given a range, it generates a vector of equidistant input sizes.
pub struct Uniform {
    range: RangeInclusive<usize>,
}

impl Uniform {
    /// Creates a new uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        Uniform { range }
    }
}

impl Display for Uniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uniform")
    }
}

impl Distribution for Uniform {
    /// Generates a vector of input sizes using an uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of input sizes to generate.
    ///
    /// # Example
    ///
    /// ```
    /// use time_complexity_plot::input::distribution::*;
    ///
    /// let uniform = Uniform::new(1..=100);
    /// let lengths = uniform.generate(10);
    /// println!("{:?}", lengths);
    /// ```
    fn generate(&self, n: usize) -> Vec<usize> {
        // Preallocating the vector of input sizes
        let mut lengths = Vec::with_capacity(n);

        // Computing the step
        let a = self.range.start();
        let b = (self.range.end() - self.range.start()) / (n - 1);

        // Generating the input sizes using the step
        for i in 0..n {
            let x = a + b * i;
            lengths.push(x);
        }

        // Returning the vector of input sizes
        lengths
    }
}

/// The struct representing a uniform random distribution.
///
/// Given a range, it generates a vector of random input sizes (uniform because all input
/// sizes have an equal probability of appearing in the generated vector).
pub struct UniformRandom {
    range: RangeInclusive<usize>,
}

impl UniformRandom {
    /// Creates a new uniform random distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        UniformRandom { range }
    }
}

impl Display for UniformRandom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniformRandom")
    }
}

impl Distribution for UniformRandom {
    /// Generates a vector of input sizes using an uniform random distribution.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of input sizes to generate.
    ///
    /// # Example
    ///
    /// ```
    /// use time_complexity_plot::input::distribution::*;
    ///
    /// let uniform_random = UniformRandom::new(1..=100);
    /// let lengths = uniform_random.generate(10);
    /// println!("{:?}", lengths);
    fn generate(&self, n: usize) -> Vec<usize> {
        // Preallocating the vector of input sizes
        let mut lengths = Vec::with_capacity(n);

        // Generating the input sizes using the functions provided by rand crate
        for _ in 0..n {
            lengths.push(thread_rng().gen_range(self.range.clone()));
        }
        lengths
    }
}

/// The struct representing an exponential distribution.
///
/// Given a range and a &lambda;, it generates a vector of input sizes using an exponential
/// distribution.
pub struct Exponential {
    range: RangeInclusive<usize>,
    lambda: f64,
}

impl Exponential {
    /// Creates a new exponential distribution.
    /// The mean of the distribution is set to match the mean of the inverse uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        let lambda =
            ((range.end() / range.start()) as f64).ln() / ((range.end() - range.start()) as f64);
        Exponential { range, lambda }
    }

    /// Sets the &lambda; of the exponential distribution.
    ///
    /// # Arguments
    ///
    /// * `lambda` - The new &lambda; of the exponential distribution.
    pub fn set_lambda(&mut self, lambda: f64) {
        assert!(lambda > 0.0, "Lambda must be grater then zero");
        self.lambda = lambda;
    }
}

impl Display for Exponential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exponential λ={}", self.lambda)
    }
}

impl Distribution for Exponential {
    /// Generates a vector of input sizes using an exponential distribution.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of input sizes to generate.
    ///
    /// # Example
    ///
    /// ```
    /// use time_complexity_plot::input::distribution::*;
    ///
    /// let exponential = Exponential::new(1..=100);
    /// let lengths = exponential.generate(10);
    /// println!("{:?}", lengths);
    /// ```
    fn generate(&self, n: usize) -> Vec<usize> {
        // Preallocating the vector of input sizes
        let mut lengths = Vec::with_capacity(n);

        for i in 0..n {
            let x: f64 = i as f64 / (n - 1) as f64;
            let exp_x = exp_distribution(
                x,
                self.lambda,
                *self.range.start() as f64,
                *self.range.end() as f64,
            );
            println!("{}", exp_x);
            lengths.push(exp_x as usize);
        }
        lengths
    }
}

/// The struct representing an exponential random distribution.
///
/// Given a range and a &lambda;, it generates a vector of random input sizes using an
/// exponential distribution. This means that the probability of appearing  in the
/// output of a specific input size n will decrease as n increases.
pub struct ExponentialRandom {
    range: RangeInclusive<usize>,
    lambda: f64,
}

impl ExponentialRandom {
    /// Creates a new exponential random distribution.
    /// The mean of the distribution is set to match the mean of the inverse uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        let lambda =
            ((range.end() / range.start()) as f64).ln() / ((range.end() - range.start()) as f64);
        ExponentialRandom { range, lambda }
    }

    /// Sets the &lambda; of the exponential random distribution.
    ///
    /// # Arguments
    ///
    /// * `lambda` - The new &lambda; of the exponential random distribution.
    pub fn set_lambda(&mut self, lambda: f64) {
        assert!(lambda > 0.0, "Lambda must be grater then zero");
        self.lambda = lambda;
    }
}

impl Display for ExponentialRandom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExponentialRandom λ={}", self.lambda)
    }
}

impl Distribution for ExponentialRandom {
    /// Generates a vector of input sizes using an exponential random distribution.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of input sizes to generate.
    ///
    /// # Example
    ///
    /// ```
    /// use time_complexity_plot::input::distribution::*;
    ///
    /// let exponential_random = ExponentialRandom::new(1..=100);
    /// let lengths = exponential_random.generate(10);
    /// println!("{:?}", lengths);
    /// ```
    fn generate(&self, n: usize) -> Vec<usize> {
        // Preallocating the vector of input sizes
        let mut lengths = Vec::with_capacity(n);

        for _ in 0..n {
            let x: f64 = thread_rng().gen::<f64>();
            let exp_x = exp_distribution(
                x,
                self.lambda,
                *self.range.start() as f64,
                *self.range.end() as f64,
            );
            lengths.push(exp_x as usize);
        }
        lengths
    }
}

/// Helper function to generate an exponential distribution.
fn exp_distribution(u: f64, lambda: f64, min: f64, max: f64) -> f64 {
    /*
    In order to generate an exponential distribution the inverse transform sampling method is used.
    Given an uniform distributed value u ∈ [0, 1] a linear transformation is applied in order to
    get e ∈ [F(min), F(max)] where F(x) is the cumulative distribution function of the exponential
    distribution. The inverse of F(x) is then applied to e in order to get the desired value:

    F^-1(x) = -ln(1 - x) / lambda

    Desired value = F^-1(e)
     */

    assert!((0.0..=1.0).contains(&u), "u must be in [0, 1]");

    let x = lambda * min;
    let y = lambda * max;
    let z: f64;

    if u == 1.0 {
        return max;
    } else if u == 0.0 {
        return min;
    }

    /*
    If the difference between y and x is small enough, we can use the exact formula to compute the
    desired value.
     */
    if y - x < f64::MAX_EXP as f64 * 2.0_f64.ln() {
        z = -y * ((1.0 - u) * (y - x).exp() + u).ln();
        return z / lambda;
    }

    /*
    If the difference between y and x is too big, we use the approximation formula to compute the
    desired value.
     */
    if y - x > (1.0 / (1.0 - u)).ln() {
        z = x - (1.0 - u).ln();
    } else {
        z = y - u.ln();
    }

    z / lambda
}
