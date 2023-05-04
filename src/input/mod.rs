use serde::Serialize;
use std::fs::File;

use self::distribution::Distribution;

/// Distribution module implements an easy way to abstract the generation of input sizes.
///
/// Provides:
/// 1) A trait that can be used to define your own distribution.
/// 2) A set of predefined distributions.
///
/// # Example
///
/// To test this module you can easily copy and paste the following code snippets.
///
/// ## Predefined distributions
///
/// ```
/// use time_complexity_plot::input::distribution::*;
///
/// let uniform = Uniform::new(1..=100);
/// let lengths = uniform.generate(10);
/// println!("{:?}", lengths);
/// ```
///
/// ## Custom distribution
///
/// ```
/// use std::fmt::Display;
///
/// use time_complexity_plot::input::distribution::*;
///
/// // The struct representing your custom distribution
/// struct Constant {
///     k: usize,
/// }
///
/// // Implement a way of creating your custom distribution
/// impl Constant {
///     pub fn new(k: usize) -> Self { Self { k } }
/// }
///
/// // Implement Display in order to print the name of your distribution in the plots
/// impl Display for Constant {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///        write!(f, "Costant")
///     }
/// }
///
/// // Implement the trait Distribution i.e. the way of generating the input sizes
/// impl Distribution for Constant {
///     fn generate(&self, n: usize) -> Vec<usize> {
///         let mut lengths = Vec::with_capacity(n);
///         for _ in 0..n {
///             lengths.push(self.k);
///         }
///         lengths
///     }
/// }
///
/// let constant = Constant::new(5);
/// let lengths = constant.generate(10);
/// println!("{:?}", lengths);
/// ```
pub mod distribution;

/// Trait that must be implemented by algorithms' input types.
pub trait Input {
    type Builder: Clone;
    fn get_size(&self) -> usize;
    fn generate_input(size: usize, builder: Self::Builder) -> Self;
}

/// Struct that holds the inputs.
#[derive(Serialize)]
pub struct InputSet<I: Input> {
    pub(crate) inputs: Vec<Vec<I>>,
}

/// Struct that let you build the [`InputSet`].
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
        ); // TODO: can this check be removed? (already done in the distribution)
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
                inputs_with_same_size.push(I::generate_input(*input_size, self.builder.clone()));
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
