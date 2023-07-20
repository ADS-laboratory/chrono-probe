//! This module provides tools for generating input data helpful for testing algorithms.
//!
//! The [`Input`] trait is the core of this module and must be implemented by the input types
//! used by algorithms. This trait defines two methods:
//!
//! * `get_size(&self) -> usize`: returns the size of the input.
//! * `generate_input(size: usize) -> Self`: generates a random input of the given size.
//!
//! This module also provides the [`InputBuilder`] struct, which is used to build your input
//! and store it into an [`InputSet`] instance. You can use the InputBuilder as soon as you
//! have:
//!
//! * Figured out which distribution suits your needs (read the [distribution] documentation
//! for more infos).
//! * Created your input type (read the example below).
//!
//! # Example
//!
//! ## Basic usage
//!
//! Let's say we are testing the performance of our new algorithm to check if a given
//! number is prime. We'll start by defining a new type to represent our input type:
//!
//! ```
//! pub struct PrimeTestInput {
//!     pub number: u32,
//! }
//! ```
//!
//! Next, we need to implement the [`Input`] trait for our new type:
//!
//! ```
//! # use rand::Rng;
//! # use chrono_probe::input::Input;
//!
//! # pub struct PrimeTestInput {
//! #    pub number: u32,
//! # }
//!
//! impl Input for PrimeTestInput {
//!     type Builder = ();
//!
//!     // Return the size of the input.
//!     fn get_size(&self) -> usize {
//!         // We use the number of bits as size.
//!         self.number.to_be_bytes().len() * 8
//!     }
//!
//!     // Generate a random input of the given size.
//!     fn generate_input(size: usize, builder: &Self::Builder) -> Self {
//!         let mut rng = rand::thread_rng();
//!         PrimeTestInput {
//!             // We consider the size as the number of bits.
//!             number: rng.gen_range(2u32.pow((size-1) as u32)..2u32.pow(size as u32)),
//!         }
//!     }
//! }
//! ```
//!
//! Now we can use our `PrimeTestInput` type to generate inputs for testing our algorithm!
//!
//! Note that the input size is taken as an argument by the `generate_input` method. If
//! you want to know more about the input sizes generation, you can read the documentation
//! of the [`distribution`] submodule.
//!
//! ## Multiple input generators
//!
//! Now, you may be curious about the [`Builder`](Input::Builder) type.
//!
//! The [`Builder`](Input::Builder) type is helpful when you want to choose between different
//! input generators. In the previous example, we only have needed one generator, so we used ()
//! as the [`Builder`](Input::Builder) type. However, if we had more than one generator, we
//! could define an enum like this:
//!
//! ```
//! pub enum Generator {
//!     Fast,
//!     Uniform,
//! }
//! ```
//!
//! Then, we could use `Generator` as the [`Builder`](Input::Builder) type:
//!
//! ```
//! use chrono_probe::input::Input;
//!
//! # pub struct PrimeTestInput {
//! #    pub number: u32,
//! # }
//!
//! # pub enum Generator {
//! #    Fast,
//! #    Uniform,
//! # }
//!
//! # fn generate_order_vector_fast(size: usize, min: u32, max: u32) -> PrimeTestInput { todo!() }
//! # fn generate_order_vector(size: usize, min: u32, max: u32) -> PrimeTestInput { todo!() }
//!
//! impl Input for PrimeTestInput {
//!     type Builder = Generator;
//!
//!     // Return the size of the input.
//!     fn get_size(&self) -> usize {
//!         // We use the number of bits as size.
//!         self.number.to_be_bytes().len() * 8
//!     }
//!
//!     // Generate a random input of the given size.
//!     fn generate_input(size: usize, builder: &Self::Builder) -> Self {
//!         match builder {
//!             Generator::Fast => generate_order_vector_fast(size, u32::MIN, u32::MAX),
//!             Generator::Uniform => generate_order_vector(size, u32::MIN, u32::MAX),
//!         }
//!     }
//! }
//! ```
//!
//! ### Using primitive types as input
//!
//! If you're new to Rust, you may be wondering why we need to create a new type for the input
//! when we could just use the `u32` type itself. The reason is that only traits you own can be
//! implemented for primitive types, and this library owns the [`Input`] trait, not your crate
//! where you're using this library.
//! If you need to use a primitive type as an input you need to create a new wrapper type, for
//! more information refer to the [rust guide](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)

use std::fs::File;

use serde::Serialize;

use self::distribution::Distribution;

pub mod distribution;

/// Trait that must be implemented by algorithms' input types.
pub trait Input {
    /// The type of the builder. A builder can be used to select the type of generation for the inputs.
    type Builder;
    /// Returns the size of the input.
    fn get_size(&self) -> usize;
    /// Generates an input of the given size, using the given builder.
    fn generate_input(size: usize, builder: &Self::Builder) -> Self;
}

/// Struct that holds the inputs.
#[derive(Serialize)]
pub struct InputSet<I: Input> {
    /// The inputs.
    /// The inputs are grouped by size.
    pub inputs: Vec<Vec<I>>,
}

/// Struct used for building an [`InputSet`].
#[derive(Serialize)]
pub struct InputBuilder<I: Input, D: Distribution> {
    // The distribution that will be used to generate the input lengths.
    pub(crate) distribution: D,
    // The builder that will be used to generate the inputs.
    pub(crate) builder: I::Builder,
}

impl<I: Input, D: Distribution> InputBuilder<I, D> {
    /// Creates a new [`InputBuilder`].
    ///
    /// # Arguments
    ///
    /// * `distribution` - The distribution that will be used to generate the input lengths.
    /// * `builder` - The builder that will be used to generate the inputs.
    pub fn new(distribution: D, builder: I::Builder) -> InputBuilder<I, D> {
        InputBuilder {
            distribution,
            builder,
        }
    }

    /// Generates the inputs.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of inputs to be generated.
    pub fn build(&self, n: usize) -> InputSet<I> {
        self.build_with_repetitions(n, 1)
    }

    /// Generates the inputs with repetitions (i.e. multiple inputs with the same size).
    /// This can be useful in order to obtain a more accurate result.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of inputs to be generated (excluding repetitions: the actual amount of inputs generated is n*repetitions).
    /// * `repetitions` - The number of repetitions for each input size.
    pub fn build_with_repetitions(&self, n: usize, repetitions: usize) -> InputSet<I> {
        // TODO: remove these assertions: usually asserts are not used in libraries
        // A better way to handle this would be to return a Result instead of panicking
        assert!(
            n > 0,
            "The number of inputs to be generated must be greater than 0"
        );
        assert!(
            repetitions > 0,
            "The number of repetitions must be greater than 0"
        );

        // Initialize the inputs vec with the correct capacity
        let mut inputs = Vec::with_capacity(n);

        // Generate the input lengths using the given distribution
        let length_distribution = self.distribution.generate(n);

        // Printing in the console for debug purposes
        #[cfg(feature = "debug")]
        println!("Generating inputs...\n");

        // Iterate over the input lengths
        for (_j, input_size) in length_distribution.iter().enumerate() {
            // Initialize the vec holding the inputs with the same size
            let mut inputs_with_same_size = Vec::with_capacity(repetitions);

            // Iterate over the repetitions
            for _ in 0..repetitions {
                // Generate the inputs of the given size and push them to the vec
                inputs_with_same_size.push(I::generate_input(*input_size, &self.builder));
            }

            // Push the vec holding the inputs with the same size to the inputs vec
            inputs.push(inputs_with_same_size);

            // Printing in the console the progress for debug purposes
            #[cfg(feature = "debug")]
            {
                if _j % (n / 20) == 0 {
                    println!("{}%", _j * 100 / n);
                }
            }
        }

        // Return the input set
        InputSet { inputs }
    }
}

impl<I: Input + Serialize> InputSet<I> {
    /// Serializes the input set in a json file.
    /// The file will be created if it doesn't exist, otherwise it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to be created.
    ///
    /// # Panics
    ///
    /// * Panics if the file cannot be created.
    /// * Panics if the input set cannot be serialized.
    ///
    pub fn serialize_json(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        serde_json::to_writer(&mut file, &self).unwrap();
        // TODO: handle errors instead of panicking maybe returning a Result
    }
}
