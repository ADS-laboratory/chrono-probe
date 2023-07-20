//! # Chrono-probe
//!
//! **Chrono-probe** is a library for measuring the time complexity of algorithms.
//!
//! In order to measure the time complexity of an algorithm, you need to provide an algorithm (or
//! more) and a way to generate inputs for the algorithm. The library will then measure the time
//! it takes for the algorithm to run on the generated inputs of various lengths. In this way, it
//! is possible to obtain a plot of the time taken by the algorithm as a function of the input size.
//!
//! The library is designed to be as flexible as possible. It is possible to use the library to
//! measure the time complexity of any algorithm, as long as the algorithm can be expressed as a
//! function that takes an input and returns an output.
//!
//! ## How to use chrono-probe
//!
//! In this section, we will show how to use **chrono-probe** to measure the time complexity
//! of a sorting algorithm. We will use the `quicksort` algorithm as an example. This example and
//! more can be found in the [examples](https://github.com/ADS-laboratory/time-complexity-plot/tree/lib/examples) folder.
//!
//! The implementation of the quicksort algorithm is not important for this example, the definition
//! would look something like this:
//! ```rust
//! fn quick_sort<T: Ord + Clone>(v: &mut [T]) {
//!    // ...
//! }
//! ```
//!
//! The first step is to define a type that represents the input to the algorithm. In this case we
//! want to measure the time complexity of sorting vectors of u32, so we define a new type that is
//! a vector of u32. This is done because rust does not allow us to implement traits for types
//! defined in other crates and we need to implement the [Input]() trait for our type.
//!
//! ```rust
//! #[derive(Clone)]
//! pub struct InputVec(Vec<u32>);
//! ```
//!
//! We can also implement `Deref` and `DerefMut` for InputVec, so that we can use it as a `Vec<u32>`.
//! This is not necessary, but it makes it easier to use.
//!
//! ```rust
//! impl Deref for InputVec {
//!     type Target = Vec<u32>;
//!
//!     fn deref(&self) -> &Self::Target {
//!         &self.0
//!     }
//! }
//!
//! impl DerefMut for InputVec {
//!     fn deref_mut(&mut self) -> &mut Self::Target {
//!         &mut self.0
//!     }
//! }
//! ```
//!
//! Now we can define the quicksort algorithm as a function that takes an InputVec.
//! ```rust
//! fn quick_sort_measure(v: &mut InputVec) {
//!    quick_sort(v);
//! }
//! ```
//!
//! The next step is to implement the [input::Input] trait for `InputVec`. This trait defines how to generate
//! inputs for the algorithm and what the size of the input is. In this case we don't need to choose
//! between different input generators, so we don't need a Builder, for more information on how to
//! use Builders see the documentation for the [input::Input] trait.
//!
//! ```rust
//! impl Input for InputVec {
//!    // We don't need a Builder.
//!     type Builder = ();
//!
//!     // Return the size of the input, in this case the size is the length of the vector.
//!     fn get_size(&self) -> usize {
//!         self.len()
//!     }
//!
//!     // Generate a vector of the given size and fill it with random numbers.
//!     fn generate_input(size: usize, _builder: &Self::Builder) -> Self {
//!         let mut rng = thread_rng();
//!         let mut v = Vec::with_capacity(size);
//!         for _ in 0..size {
//!             let rand: u32 = rng.gen();
//!             v.push(rand);
//!         }
//!         InputVec(v)
//!     }
//! }
//! ```
//!
//! Now we implemented all the necessary traits and we can use the library to measure the algorithm.
//!
//!
//! In the main function we create a distribution for the length of the vectors. Here we use a linear
//! distribution with a minimum of 1000 and a maximum of 500_000. So all the vectors will have a
//! length between 1000 and 500_000 and the length will be chosen uniformly at random.
//!
//! ```rust
//! let length_distribution = Linear::new(1000..=500_000);
//! ```
//!
//! The next step is to create a builder for the vectors. The builder is used to generate the
//! vectors, we only need to specify the distribution for the length of the vectors and because
//! we don't need a Builder for the InputVec, we can use `()` as the Builder.
//!
//! ```rust
//! let vector_builder = InputBuilder::new(length_distribution, ());
//! ```
//!
//! Now we can build the vectors. Here we build 200 vectors, 10 of each length.
//!
//! ```rust
//! let mut vectors = vector_builder.build_with_repetitions(200, 10);
//! ```
//!
//! Finally we can measure the algorithm. We need to specify the algorithm, the input and the
//! relative error. The relative error is used to determine how many times the algorithm should be
//! run for each input size. The algorithm will be run until the relative error is less than the
//! given relative error. We use the [`measurements::measure_mut`] function because the algorithm takes a mutable
//! reference to the input.
//!
//! ```rust
//! // Create a slice of the algorithms we want to measure
//! let algorithms: &[(fn(&mut input::InputVec), &str); 1] = &[
//!  (quick_sort_input, "Quick sort"),
//! ];
//!
//! // Measure the algorithms on the vectors, given a relative error of 0.001
//! let results = measure_mut(&mut vectors, algorithms, 0.001);
//! ```
//!
//! The results are returned as a vector of [`measurements::Measurement`]s. Each measurement contains the size of
//! the input, the time it took for the algorithm to run and the relative error of the measurement.
//!
//!
//! Results can be plotted using the [`plot::time_plot`] function.
//!
//! ```rust
//! // Plot the results
//! let config = PlotConfig::default()
//!     .with_title("Sorting algorithms")
//!     .with_caption("The time plot of sorting algorithms");
//!
//! time_plot(file_name, results, &config);
//! ```
//!
//! The entire code and other examples can be found in the [examples](https://github.com/ADS-laboratory/time-complexity-plot/tree/lib/examples) folder.

#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]

pub mod input;
pub mod measurements;
pub mod plot;
