//! This module implements an easy way to abstract the generation of input sizes.
//!
//! Provides:
//! 1) A trait that can be used to define your own distribution for input sizes.
//! 2) A trait that can be used to define your a general probability distribution.
//! 3) A set of predefined distributions.
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
//! use chrono_probe::input::distribution::*;
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
//! To achieve this goal, we will take two different approaches:
//!
//! * Implement the [`Distribution`] trait directly
//! * Implement the [`ProbabilityDistribution`] trait
//!
//! ### Implementing the [`Distribution`] trait
//!
//! * Create a struct representing the custom distribution
//! * Implement a way of creating an instance of the distribution
//! * Implement the [`Debug`] trait to allow printing the name of your distribution in the plots
//! * Implement the [`Distribution`] trait, which specifies how to generate the input sizes
//!
//! ```
//! use std::fmt::Debug;
//!
//! use chrono_probe::input::distribution::*;
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
//! ### Implementing the [`ProbabilityDistribution`] trait
//!
//! Implementing the [`ProbabilityDistribution`] trait is very similar to implementing the
//! [`Distribution`] trait. The only difference is that this time we only have to implement a way to
//! generate a single input size form an uniform distribution, and the trait will take care of
//! implementing the [`Distribution`] trait for us.
//!
//! ```
//! // Same as before
//!
//! use std::fmt::Debug;
//! use chrono_probe::input::distribution::*;
//!
//! struct Constant {
//!     k: usize,
//! }
//!
//! impl Constant {
//!     pub fn new(k: usize) -> Self { Self { k } }
//! }
//!
//! impl Debug for Constant {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!        write!(f, "Costant")
//!     }
//! }
//!
//! // This time we implement the ProbabilityDistribution trait
//! impl ProbabilityDistribution for Constant {
//!     fn inverse_cdf(&self, _u: f64) -> f64 {
//!        self.k as f64
//!     }
//! }
//! ```
//!
//! For more information on how to implement a the [`ProbabilityDistribution`] trait, see the
//! documentation of the trait itself.
//!
//! Note that this example is deliberately simple. In practice, you may want to generate
//! input sizes that are more diverse than a constant value. Nevertheless, this example
//! provides a basic understanding of how to create a custom distribution and can be used
//! as a starting point for implementing more complex distributions tailored to your needs.
//!
//! ### Which approach should I use?
//!
//! The approach you should use depends on the complexity of the distribution you want to implement.
//! If you want to implement a simple distribution, with a simple inverse cumulative distribution,
//! you can use the [`ProbabilityDistribution`] trait. If you want to implement a more complex
//! distribution, you should implement the [`Distribution`] trait directly.

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
#[derive(Debug, Clone)]
pub enum GenerationType {
    /// Generates input in fixed intervals.
    FixedIntervals,
    /// Generates input in random intervals.
    Random,
}

// ==============================
// = PREDEFINED IMPLEMENTATIONS =
// ==============================

/// This trait defines a certain probability distribution. It is used to generate input sizes
/// according to the distribution.
///
/// If a type implements this trait, and the Debug trait, it also implements the Distribution trait.
/// In this way, the user can easily create a probability distribution and use it to generate input
/// sizes, without having to implement the Distribution trait.
///
pub trait ProbabilityDistribution {
    /// This function takes a value x in \[0,1] uniformly distributed and returns a value that is
    /// distributed according to the probability distribution chosen.
    ///
    /// This can be done with the [inverse transform sampling method](https://en.wikipedia.org/wiki/Inverse_transform_sampling):
    /// given a random variable X with cumulative distribution function F, then F<sup>-1</sup>
    /// <sub>X</sub>(U) follows the same distribution of X, where U is uniformly distributed in \[0,1].
    fn inverse_cdf(&self, u: f64) -> f64;

    /// Returns the generation type of the distribution.
    ///
    /// This is used to determine whether the input sizes should be generated in fixed intervals or
    /// in random intervals. By default, it returns [`GenerationType::Random`] but it can be
    /// overridden to return the desired generation type.
    fn get_gen_type(&self) -> &GenerationType {
        &GenerationType::Random
    }
}

impl<T: ProbabilityDistribution + Debug> Distribution for T {
    fn generate(&self, n: usize) -> Vec<usize> {
        assert!(n > 0, "The number of input sizes must be greater than zero");
        // Preallocating the vector of input sizes
        let mut lengths = Vec::with_capacity(n);

        for i in 0..n {
            let u: f64 = match self.get_gen_type() {
                GenerationType::FixedIntervals => {
                    if n != 1 {
                        i as f64 / (n - 1) as f64
                    } else {
                        0.0
                    }
                }
                GenerationType::Random => thread_rng().gen::<f64>(),
            };

            let x = self.inverse_cdf(u);

            lengths.push(x as usize);
        }
        lengths
    }
}

/// The struct representing an uniform distribution.
///
/// Given a range, it generates a vector of uniform distributed input sizes.
#[derive(Clone)]
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

impl ProbabilityDistribution for Uniform {
    fn inverse_cdf(&self, u: f64) -> f64 {
        let a = *self.range.start() as f64;
        let b = (self.range.end() - self.range.start()) as f64;
        a + b * u
    }

    fn get_gen_type(&self) -> &GenerationType {
        &self.gen_type
    }
}

/// The struct representing an exponential distribution.
///
/// Given a range, it generates a vector of input sizes using an exponential distribution.
#[derive(Clone)]
pub struct Exponential {
    range: RangeInclusive<usize>,
    lambda: f64,
    gen_type: GenerationType,
}

impl Exponential {
    /// Creates a new exponential distribution.
    /// The mean of the distribution is set to match the mean of the reciprocal distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        assert!(!range.is_empty(), "The range must not be empty.");
        let lambda =
            ((range.end() / range.start()) as f64).ln() / ((range.end() - range.start()) as f64);
        let gen_type = GenerationType::FixedIntervals;
        Exponential {
            range,
            lambda,
            gen_type,
        }
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
        write!(
            f,
            "Exponential λ={}, generation type: {:?}",
            self.lambda, self.gen_type
        )
    }
}

impl ProbabilityDistribution for Exponential {
    fn inverse_cdf(&self, u: f64) -> f64 {
        /*
        In order to generate an exponential distribution the inverse transform sampling method is
        used. Given an uniform distributed value u ∈ [0, 1] a linear transformation is applied in
        order to get e ∈ [F(min), F(max)] where F(x) is the cumulative distribution function of the
        exponential distribution. The inverse of F(x) is then applied to e in order to get the
        desired value:

        F^-1(x) = -ln(1 - x) / lambda

        Desired value = F^-1(e)
        */

        let min = *self.range.start() as f64;
        let max = *self.range.end() as f64;
        let lambda = self.lambda;

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

    fn get_gen_type(&self) -> &GenerationType {
        &self.gen_type
    }
}

/// The struct representing a uniform distribution.
///
/// Given a range, it generates a vector of input sizes using a uniform distribution.
pub struct Reciprocal {
    range: RangeInclusive<usize>,
    gen_type: GenerationType,
}

impl Reciprocal {
    /// Creates a new uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of the distribution.
    pub fn new(range: RangeInclusive<usize>) -> Self {
        assert!(!range.is_empty(), "The range must not be empty.");
        Reciprocal {
            range,
            gen_type: GenerationType::FixedIntervals,
        }
    }

    /// Sets the generation type of the reciprocal distribution.
    /// The generation type can be either fixed intervals or random.
    ///
    /// # Arguments
    ///
    /// * `gen_type` - The new generation type of the exponential distribution.
    pub fn set_gen_type(&mut self, gen_type: GenerationType) {
        self.gen_type = gen_type;
    }
}

impl Debug for Reciprocal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reciprocal, generation type: {:?}", self.gen_type)
    }
}

impl ProbabilityDistribution for Reciprocal {
    fn inverse_cdf(&self, u: f64) -> f64 {
        (*self.range.end() as f64 / *self.range.start() as f64).powf(u) * *self.range.start() as f64
    }

    fn get_gen_type(&self) -> &GenerationType {
        &self.gen_type
    }
}
