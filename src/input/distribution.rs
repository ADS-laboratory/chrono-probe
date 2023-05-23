//! This module implements an easy way to abstract the generation of input sizes.
//!
//! Provides:
//! 1) A trait that can be used to define your own distribution.
//! 2) A set of predefined distributions.
//!
//! # Examples
//!
//! To test this module you can easily copy and paste the following code snippets.
//!
//! ## Predefined distributions
//!
//! This example demonstrates how to use a predefined distribution to generate a vector of input
//! sizes. In this specific case, we use the [`Uniform`] distribution as an example.
//!
//! ```
//! use time_complexity_plot::input::distribution::*;
//!
//! // First, we create an instance of the Uniform distribution
//! let uniform = Uniform::new(1..=100);
//!
//! // Then we generate a vector of 10 input sizes using the distribution
//! let lengths = uniform.generate(10);
//!
//! // Finally, we print the vector of input sizes
//! println!("{:?}", lengths);
//! ```
//!
//! ## Custom distribution
//!
//! In this example we will cover the steps needed to create a custom distribution.
//! The goal is to generate a vector of input sizes that are all equal to a given constant.
//! To achieve this goal, we will follow these steps:
//! * Create a struct representing the custom distribution
//! * Implement a way of creating an instance of the distribution
//! * Implement the [`Debug`] trait to allow printing the name of your distribution in the plots
//! * Implement the [`Distribution`] trait, which specifies how to generate the input sizes
//!
//! ```
//! use std::fmt::Debug;
//!
//! use time_complexity_plot::input::distribution::*;
//!
//! // First, we create the struct representing the custom distribution
//! struct Constant {
//!     k: usize,
//! }
//!
//! // Then we implement a way of creating an instance of the distribution
//! impl Constant {
//!     pub fn new(k: usize) -> Self { Self { k } }
//! }
//!
//! // By implementing the Display trait, we can print the name of our distribution in the plots
//! impl Debug for Constant {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!        write!(f, "Costant")
//!     }
//! }
//!
//! // Lastly, we implement the Distribution trait, which specifies how to generate the input sizes
//! impl Distribution for Constant {
//!     fn generate(&self, n: usize) -> Vec<usize> {
//!         let mut lengths = Vec::with_capacity(n);
//!         for _ in 0..n {
//!             lengths.push(self.k);
//!         }
//!         lengths
//!     }
//! }
//!
//! let constant = Constant::new(5);
//! let lengths = constant.generate(10);
//! println!("{:?}", lengths);
//! ```
//!
//! Note that this example is deliberately simple. In practice, you may want to generate
//! input sizes that are more diverse than a constant value. Nevertheless, this example
//! provides a basic understanding of how to create a custom distribution and can be used
//! as a starting point for implementing more complex distributions tailored to your needs.

use std::fmt::Debug;
use std::ops::RangeInclusive;

use rand::{Rng, thread_rng};

// =====================
// = THE MODULE ITSELF =
// =====================

/// This trait defines a Distribution in an abstract way.
///
/// Without implementing lower level mechanisms this trait defines the shared behaviour of a
/// distribution, i.e. the property of being able to generate the input sizes.
pub trait Distribution: Debug {
    /// Generates a vector of input sizes. The number of input sizes to generate is given as
    /// argument.
    fn generate(&self, n: usize) -> Vec<usize>;
}

/// This enum defines the possible generation types.
#[derive(Debug)]
pub enum GenerationType {
    /// Generates input in fixed intervals.
    FixedIntervals,
    /// Generates input in random intervals.
    Random,
}

// ==============================
// = PREDEFINED IMPLEMENTATIONS =
// ==============================

/// The struct representing an uniform distribution.
///
/// Given a range, it generates a vector of uniform distributed input sizes.
pub struct Uniform {
    range: RangeInclusive<usize>,
    gen_type: GenerationType,
}

impl Uniform {
    /// Creates a new uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        assert!(!range.is_empty(), "The range must not be empty.");
        Uniform {
            range,
            gen_type: GenerationType::FixedIntervals,
        }
    }

    /// Sets the generation type of the exponential distribution.
    /// The generation type can be either fixed intervals or random.
    ///
    /// # Arguments
    ///
    /// * `gen_type` - The generation type.
    pub fn set_gen_type(&mut self, gen_type: GenerationType) {
        self.gen_type = gen_type;
    }
}

impl Debug for Uniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uniform, generation type: {:?}", self.gen_type)
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
        let a = *self.range.start() as f64;
        let b = (self.range.end() - self.range.start()) as f64;

        // Generating the input sizes using the step
        for i in 0..n {
            let x = match self.gen_type {
                GenerationType::FixedIntervals => {
                    i as f64 / (n - 1) as f64
                }
                GenerationType::Random => {
                    thread_rng().gen::<f64>()
                }
            };
            lengths.push((a + b * x) as usize);
        }

        // Returning the vector of input sizes
        lengths
    }
}

/// The struct representing an exponential distribution.
///
/// Given a range, it generates a vector of input sizes using an exponential distribution.
pub struct Exponential {
    range: RangeInclusive<usize>,
    lambda: f64,
    gen_type: GenerationType,
}

impl Exponential {
    /// Creates a new exponential distribution.
    /// The mean of the distribution is set to match the mean of the inverse uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        assert!(!range.is_empty(), "The range must not be empty.");
        let lambda =
            ((range.end() / range.start()) as f64).ln() / ((range.end() - range.start()) as f64);
        let gen_type = GenerationType::FixedIntervals;
        Exponential { range, lambda, gen_type }
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

    /// Sets the generation type of the exponential distribution.
    /// The generation type can be either fixed intervals or random.
    ///
    /// # Arguments
    ///
    /// * `gen_type` - The new generation type of the exponential distribution.
    pub fn set_gen_type(&mut self, gen_type: GenerationType) {
        self.gen_type = gen_type;
    }
}

impl Debug for Exponential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exponential λ={}, generation type: {:?}", self.lambda, self.gen_type)
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
        assert!(n > 0, "The number of input sizes must be greater than zero");
        // Preallocating the vector of input sizes
        let mut lengths = Vec::with_capacity(n);

        for i in 0..n {
            let x: f64 = match self.gen_type {
                GenerationType::FixedIntervals => i as f64 / (n - 1) as f64,
                GenerationType::Random => thread_rng().gen::<f64>(),
            };
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
// u ∈ [0, 1], lambda > 0, min > 0, max > 0, min < max
fn exp_distribution(u: f64, lambda: f64, min: f64, max: f64) -> f64 {
    /*
    In order to generate an exponential distribution the inverse transform sampling method is used.
    Given an uniform distributed value u ∈ [0, 1] a linear transformation is applied in order to
    get e ∈ [F(min), F(max)] where F(x) is the cumulative distribution function of the exponential
    distribution. The inverse of F(x) is then applied to e in order to get the desired value:

    F^-1(x) = -ln(1 - x) / lambda

    Desired value = F^-1(e)
     */

    let x = lambda * min;
    let y = lambda * max;
    let z: f64;

    if u == 1.0_f64 {
        return max;
    } else if u == 0.0 {
        return min;
    }

    /*
    If the difference between y and x is small enough, we can use the exact formula to compute the
    desired value.
     */
    if y - x < f64::MAX_EXP as f64 * 2.0_f64.ln() {
        z = y - ((1.0 - u) * (y - x).exp() + u).ln();
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
