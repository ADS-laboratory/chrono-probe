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
//! * Figured out which distribution suits your needs (read the [distribution] documention
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
//!             number: rng.gen_range(2u32.pow(size-1 as u32)..2u32.pow(size as u32)),
//!         }
//!     }
//! }
//! ```
//!
//! Now we can use our [`PrimeTestInput`] type to generate inputs for testing our algorithm!
//!
//! Note that the input size is taken as an argument by the [`generate_input`] method. If
//! you want to know more about the input sizes generation, you can read the documentation
//! of the [`distribution`] submodule.
//!
//! ## Multiple input generators
//!
//! Now, you may be curious about the [`Builder`](Input::Builder) type.
//!
//! The [`Builder`](Input::Builder) type is helpful when you want to choose between different
//! input generators. In the previus example, we only have needed one generator, so we used ()
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
//! impl Input for PrimeTestInput {
//!     type Builder = Generator;
//!
//!     // Return the size of the input.
//!     fn get_size(&self) -> usize {
//!         1
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
//! If you're new to Rust, you may be wondering why we need to create a new type for the input
//! when we could just use the `u32` type itself. The reason is that only traits you own can be
//! implemented for primitive types, and this library owns the [`Input`] trait, not your crate
//! where you're using this library.
//!
//! ## Using primitive types as input
//!
//! If you need to use a primitive type as input but can't wrap it in a new type, you can use
//! the [impl_input] macro.
//!
//! TODO:...

use serde::Serialize;
use std::fs::File;

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
    pub(crate) inputs: Vec<Vec<I>>,
}

/// Struct used for building an [`InputSet`].
#[derive(Serialize)]
pub struct InputBuilder<I: Input, D: Distribution> {
    pub(crate) distribution: D,
    pub(crate) builder: I::Builder,
}

impl<I: Input, D: Distribution> InputBuilder<I, D> {
    /// Creates a new [`InputBuilder`].
    ///
    /// # Arguments
    ///
    /// * `distribution` - The builder of the distribuition that will be used to generate the inputs.
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
    /// * `n` - The number of inputs to be generated (exluding repetitions: the actual amount of inputs generated is n*repetitions).
    /// * `repetitions` - The number of repetitions for each input size.
    pub fn build_with_repetitions(&self, n: usize, repetitions: usize) -> InputSet<I> {
        assert!(
            n > 0,
            "The number of inputs to be generated must be greater than 0"
        );
        assert!(
            repetitions > 0,
            "The number of repetitions must be greater than 0"
        );
        let mut inputs = Vec::with_capacity(n);
        let length_distribution = self.distribution.generate(n);
        #[cfg(feature = "debug")]
        println!("Generating inputs...\n");
        for (_j, input_size) in length_distribution.iter().enumerate() {
            let mut inputs_with_same_size = Vec::with_capacity(repetitions); // TODO: do we need this vec? (maybe we could just push the inputs directly into the inputs vec without a Vec<Vec<_>>)
            for _ in 0..repetitions {
                inputs_with_same_size.push(I::generate_input(*input_size, &self.builder));
            }
            inputs.push(inputs_with_same_size);
            #[cfg(feature = "debug")]
            {
                if _j % (n / 20) == 0 {
                    println!("{}%", _j * 100 / n);
                }
            }
        }
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
    }
}

/// Implements Input for the given type using the given closure to get the size of the input.
/// Useful for not having to create a wrapper for built-in data types.
///
/// # Syntax
///
/// (`$generate_input_closure`)(`$builder`) -> `$input`, `$get_size_closure`
///
/// # Arguments
///
/// * `$generate_input_closure` - The closure that will be used to generate the input through the builder.
///     (`|usize, $builder| -> $built_in_type`)
/// * `$builder` - The type of the builder that will be used to generate the input.
/// * `$input` - The type to implement [Input](Input) for.
/// * `$get_size_closure` - The closure that will be used to get the size of the input.
///     (`|$built_in_type| -> usize`)
///
/// # Example
///
/// ```
/// use time_complexity_plot::impl_input;
/// // TODO: example
/// // impl_input!(()() -> Vec<i32>, |v: Vec<i32>| v.len());
/// ```
#[macro_export]
macro_rules! impl_input {
    (($generate_input_closure:expr)($builder:ty) -> $input:ty, $get_size_closure:expr) => {
        /// Implementation of "Input" for $built_in_type
        impl $crate::input::Input for $input {
            type Builder<'a> = $builder;

            /// Gets the size of the input.
            fn get_size(&self) -> usize {
                $get_size_closure(self.clone())
            }

            /// Generates an input of the given size using the given builder.
            ///
            /// # Arguments
            ///
            /// * `size` - The size of the input to be generated.
            /// * `builder` - The builder that will be used to generate the input (it's basically a function).
            ///
            /// # Example
            ///
            /// ```
            /// use time_complexity_plot::impl_input;
            /// use time_complexity_plot::input::Input;
            ///
            /// struct VecBuilder<'a> {
            ///     func: &'a fn(usize) -> Vec<i32>,
            /// }
            ///
            /// impl_input!( (|size: usize, builder: VecBuilder| (builder.func)(size))(VecBuilder<'a>) -> Vec<i32>, |v: Vec<i32>| v.len());
            /// let builder: VecBuilder = todo!();
            /// let input = Vec::<i32>::generate_input(10, builder);
            /// ```
            fn generate_input(size: usize, builder: Self::Builder) -> Self {
                $generate_input_closure(size, builder)
            }
        }
    };
}

fn f() {
    let a = 0u32;
    let b = a.to_be_bytes().len();
}
