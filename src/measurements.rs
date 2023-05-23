//! # Measurements
//!
//! This module contains the methods to measure the performance of an algorithm.
//!
//! Provides the following functions:
//!
//! * `measure`
//! * `measure_mut`
//!
//! Those functions take as input:
//! * A reference to an [`InputSet`](crate::input::InputSet), which contains the inputs to test the algorithm on.
//! * A relative error threshold.
//! * A slice of algorithms to test.
//!
//! An [`InputSet`] should be created using an [`InputSet`](crate::input::InputSet), for more information
//! read the [crate::input] module documentation.\
//!
//! Relative error is a float number that can be set to adjust the precision of the measurements.
//! The smaller the relative error, the more precise the measurements will be, but the longer
//! it will take to run the tests.\
//!
//! Each algorithms must be a function that takes **one** input that implements the [`Input`] trait
//! and returns **one** output. This means that if your algorithm takes more than one input, you need
//! to wrap them in a struct and implement the [`Input`] trait for it.
//! The [`measure`] function take algorithms of type `Fn(&I) -> O` as input, while the [`measure_mut`]
//! function takes algorithms of type `Fn(&mut I) -> O`. This means that if your algorithm mutate the
//! input (like a sorting algorithm) you need to use the [`measure_mut`] function, otherwise you can
//! use the [`measure`] function. If the input of your algorithm is not mutable prefer using the
//! [`measure`] function, as it is faster and more precise.\
//!
//! The output of these functions is a [`Measurements`] struct, which contains the measurements of
//! each algorithm on each input. Useful methods are provided like [`Measurements::serialize_json`]
//! to save the measurements to a file or [`Measurements::log_log_scale`] to scale the measurements
//! to a log-log scale.
//!
//! Examples of the use of these two function can be found in the [examples](https://github.com/ADS-laboratory/time-complexity-plot/tree/lib/examples) folder.


use std::fs::File;
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::input::{Input, InputSet};

/// A point containing the size of the input and the time it took to process it
#[derive(Serialize, Clone)]
pub struct Point {
    /// The size of the input
    pub size: usize,
    /// The time it took to process the input
    pub time: Duration,
}

/// A measurement of an algorithm.
/// Contains all the times it took the algorithm to process a set of inputs
#[derive(Serialize, Clone)]
pub struct Measurement {
    /// The name of the algorithm
    pub algorithm_name: String,
    /// Vector of points in the measurement
    pub measurement: Vec<Point>,
}

/// A set of measurements for some algorithms.
#[derive(Serialize, Clone)]
pub struct Measurements {
    /// Vector of measurements
    pub measurements: Vec<Measurement>,
    /// The relative error of the measurements
    pub relative_error: f32,
    /// The resolution of the clock
    pub resolution: Duration,
}

/// Estimates the resolution of the clock
fn get_resolution() -> Duration {
    // A measurement of a monotonically nondecreasing clock
    let start = Instant::now();
    loop {
        let end = start.elapsed();
        if end != Duration::ZERO {
            return end;
        }
    }
}

/// Estimates the resolution of the clock by averaging 100 measurements
fn get_average_resolution() -> Duration {
    let mut sum = Duration::ZERO;
    for _ in 0..100 {
        sum += get_resolution();
    }
    sum / 100
}

/// Estimates the time it takes to run a function given a single input
///
/// # Arguments
///
/// * `f` - The function to measure
/// * `string` - The string to pass to the function
/// * `relative_error` - The required relative error of the measurement
/// * `resolution` - The resolution of the clock
fn get_time<I, O, Alg>(f: Alg, input: &I, relative_error: f32, resolution: Duration) -> Duration
where
    I: Input,
    Alg: Fn(&I) -> O,
{
    let mut n = 0;
    // The minimum time measurable
    let min_time_measurable = resolution * ((1.0 / relative_error) + 1.0) as u32;
    let mut end: Duration;
    let start = Instant::now();
    loop {
        // Run the function
        (f)(input);

        n += 1;

        // Measure the time it takes to run the function
        end = start.elapsed();

        // Exit the loop if the time it takes to run the function is greater than the minimum time measurable
        if end > min_time_measurable {
            break;
        }
    }
    end / n
}

/// Estimates the time it takes to run a function given a single mutable input
///
/// # Arguments
///
/// * `f` - The function to measure
/// * `string` - The string to pass to the function
/// * `relative_error` - The required relative error of the measurement
/// * `resolution` - The resolution of the clock
fn get_time_mut<I, O, Alg>(
    f: Alg,
    input: &I,
    relative_error: f32,
    resolution: Duration,
) -> Duration
where
    I: Input + Clone,
    Alg: Fn(&mut I) -> O,
{
    let mut n = 0;
    // The minimum time measurable
    let min_time_measurable = resolution * ((1.0 / relative_error) + 1.0) as u32;
    let mut end: Duration;
    let mut start = Instant::now();
    loop {
        // Measure the time it takes to clone the input
        let start_input_clone = Instant::now();
        let input_cloned = &mut input.clone();
        n += 1;
        let end_input_clone = start_input_clone.elapsed();

        // Run the function
        (f)(input_cloned);

        // Remove the time it takes to clone the input
        start += end_input_clone;

        // Measure the time it takes to run the function
        end = start.elapsed();

        // Exit the loop if the time it takes to run the function is greater than the minimum time measurable
        if end > min_time_measurable {
            break;
        }
    }
    end / n
}

/// Estimates the time it takes to run a function given a vector of inputs of the same length.
/// Return a Point with the length of the strings and the total time it took to run the function on all the strings.
///
/// # Arguments
///
/// * `f` - The function to measure
/// * `strings` - The vector of strings to pass to the function
/// * `relative_error` - The required relative error of the measurement
/// * `resolution` - The resolution of the clock
fn get_time_same_length<I, O, Alg>(
    f: &Alg,
    inputs: &Vec<I>,
    relative_error: f32,
    resolution: Duration,
) -> Point
where
    I: Input,
    Alg: Fn(&I) -> O,
{
    let mut total_time = Duration::ZERO;
    let size = inputs[0].get_size();
    for input in inputs {
        let time = get_time(f, input, relative_error, resolution);
        total_time += time;
    }
    Point {
        size,
        time: total_time,
    }
}

/// Estimates the time it takes to run a function given a mutable vector of inputs of the same length.
/// Return a Point with the length of the strings and the total time it took to run the function on all the strings.
///
/// # Arguments
///
/// * `f` - The function to measure
/// * `strings` - The vector of strings to pass to the function
/// * `relative_error` - The required relative error of the measurement
/// * `resolution` - The resolution of the clock
fn get_time_same_length_mut<I, O, Alg>(
    f: &Alg,
    inputs: &Vec<I>,
    relative_error: f32,
    resolution: Duration,
) -> Point
where
    I: Input + Clone,
    Alg: Fn(&mut I) -> O,
{
    let mut total_time = Duration::ZERO;
    let size = inputs[0].get_size();
    for input in inputs {
        let time = get_time_mut(f, input, relative_error, resolution);
        total_time += time;
    }
    Point {
        size,
        time: total_time,
    }
}

/// Estimates the times it takes to run a function given a vector of inputs
///
/// # Arguments
///
/// * `f` - The function to measure
/// * `strings` - The vector of strings to pass to the function
/// * `relative_error` - The required relative error of the measurement
/// * `resolution` - The resolution of the clock
fn get_times<I, O, Alg>(
    f: &Alg,
    inputs: &InputSet<I>,
    relative_error: f32,
    resolution: Duration,
) -> Measurement
where
    I: Input,
    Alg: Fn(&I) -> O,
{
    let n = inputs.inputs.len();
    let mut times = Vec::with_capacity(n);
    for (_i, input) in inputs.inputs.iter().enumerate() {
        let time = get_time_same_length(f, input, relative_error, resolution);
        times.push(time);
        #[cfg(feature = "debug")]
        {
            if _i % (n / 20) == 0 {
                println!("{}%", _i * 100 / n);
            }
        }
    }
    Measurement {
        algorithm_name: get_algorithm_name(f),
        measurement: times,
    }
}

/// Estimates the times it takes to run a function given a mutable vector of inputs
///
/// # Arguments
///
/// * `f` - The function to measure
/// * `strings` - The vector of strings to pass to the function
/// * `relative_error` - The required relative error of the measurement
/// * `resolution` - The resolution of the clock
fn get_times_mut<I, O, Alg>(
    f: &Alg,
    inputs: &InputSet<I>,
    relative_error: f32,
    resolution: Duration,
) -> Measurement
where
    I: Input + Clone,
    Alg: Fn(&mut I) -> O,
{
    let n = inputs.inputs.len();
    let mut times = Vec::with_capacity(n);
    for (_i, input) in inputs.inputs.iter().enumerate() {
        let time = get_time_same_length_mut(f, input, relative_error, resolution);
        times.push(time);
        #[cfg(feature = "debug")]
        {
            if _i % (n / 20) == 0 {
                println!("{}%", _i * 100 / n);
            }
        }
    }
    Measurement {
        algorithm_name: get_algorithm_name_mut(f),
        measurement: times,
    }
}

/// Measures the time it takes to run different functions given an [`InputSet`].
///
/// # Arguments
///
/// * `strings` - The [`InputSet`] to pass to the functions
/// * `algorithms` - The vector of functions to measure
/// * `relative_error` - The required relative error of the measurements
///
pub fn measure<I, O, Alg>(
    inputs: &InputSet<I>,
    algorithms: &[Alg],
    relative_error: f32,
) -> Measurements
where
    I: Input,
    Alg: Fn(&I) -> O,
{
    assert!(relative_error > 0.0, "Relative error must be positive");
    let resolution = get_average_resolution();
    let mut results = Vec::with_capacity(algorithms.len());
    for (_i, algorithm) in algorithms.iter().enumerate() {
        #[cfg(feature = "debug")]
        println!(
            "\n\nProcessing {} ({}/{})...\n",
            get_algorithm_name(algorithm),
            _i + 1,
            algorithms.len()
        );
        let measurement = get_times(algorithm, inputs, relative_error, resolution);
        results.push(measurement);
    }
    Measurements {
        measurements: results,
        relative_error,
        resolution,
    }
}

/// Measures the time it takes to run different functions given a mutable [`InputSet`].
///
/// # Arguments
///
/// * `strings` - The [`InputSet`] to pass to the functions
/// * `algorithms` - The vector of functions to measure
/// * `relative_error` - The required relative error of the measurements
///
pub fn measure_mut<I, O, Alg>(
    inputs: &InputSet<I>,
    algorithms: &[Alg],
    relative_error: f32,
) -> Measurements
where
    I: Input + Clone,
    Alg: Fn(&mut I) -> O,
{
    assert!(relative_error > 0.0, "Relative error must be positive");
    let resolution = get_average_resolution();
    let mut results = Vec::with_capacity(algorithms.len());
    for (_i, algorithm) in algorithms.iter().enumerate() {
        #[cfg(feature = "debug")]
        println!(
            "\n\nProcessing {} ({}/{})...\n",
            get_algorithm_name_mut(algorithm),
            _i + 1,
            algorithms.len()
        );
        let measurement = get_times_mut(algorithm, inputs, relative_error, resolution);
        results.push(measurement);
    }
    Measurements {
        measurements: results,
        relative_error,
        resolution,
    }
}

// Some useful functions for Measurement
impl Measurement {
    /// Get the maximum time it took to run the function
    pub fn max_time(&self) -> Duration {
        self.measurement
            .iter()
            .max_by_key(|point| point.time)
            .unwrap()
            .time
    }

    /// Get the minimum time it took to run the function
    pub fn min_time(&self) -> Duration {
        self.measurement
            .iter()
            .min_by_key(|point| point.time)
            .unwrap()
            .time
    }

    /// Get the maximum length of the strings passed to the function
    pub fn max_length(&self) -> usize {
        self.measurement
            .iter()
            .max_by_key(|point| point.size)
            .unwrap()
            .size
    }

    /// Get the minimum length of the strings passed to the function
    pub fn min_length(&self) -> usize {
        self.measurement
            .iter()
            .min_by_key(|point| point.size)
            .unwrap()
            .size
    }

    /// Get the linear regression of the [`Measurement`]
    pub fn linear_regression(&self) -> (f32, f32) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        let mut n = 0.0;
        for point in &self.measurement {
            let x = point.size as f32;
            let y = point.time.as_micros() as f32;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
            n += 1.0;
        }
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;
        (slope, intercept)
    }

    /// Returns a new [`Measurement`] where the size and time of every [`Point`] is
    /// the logarithm in base 2 of the original ones.
    pub fn log_log_scale(&self) -> Self {
        let mut new_measurement = Measurement {
            algorithm_name: self.algorithm_name.clone(),
            measurement: Vec::with_capacity(self.measurement.len()),
        };
        for point in &self.measurement {
            new_measurement.measurement.push(Point {
                size: (point.size as f32).log2() as usize,
                time: Duration::from_micros((point.time.as_micros() as f32).log2() as u64),
            });
        }
        new_measurement
    }
}

impl Measurements {
    /// Get the maximum time it took to run the functions
    pub fn max_time(&self) -> Duration {
        self.measurements
            .iter()
            .max_by_key(|measurement| measurement.max_time())
            .unwrap()
            .max_time()
    }

    /// Get the minimum time it took to run the functions
    pub fn min_time(&self) -> Duration {
        self.measurements
            .iter()
            .min_by_key(|measurement| measurement.min_time())
            .unwrap()
            .min_time()
    }

    /// Get the maximum length of the strings passed to the functions
    pub fn max_length(&self) -> usize {
        self.measurements
            .iter()
            .max_by_key(|measurement| measurement.max_length())
            .unwrap()
            .max_length()
    }

    /// Get the minimum length of the strings passed to the functions
    pub fn min_length(&self) -> usize {
        self.measurements
            .iter()
            .min_by_key(|measurement| measurement.min_length())
            .unwrap()
            .min_length()
    }

    /// Returns a new [`Measurements`] where the size and time of every [`Point`] is
    /// the logarithm in base 2 of the original ones.
    pub fn log_log_scale(&self) -> Self {
        let mut new_measurements = Measurements {
            measurements: Vec::with_capacity(self.measurements.len()),
            relative_error: self.relative_error,
            resolution: self.resolution,
        };
        for measurement in &self.measurements {
            new_measurements
                .measurements
                .push(measurement.log_log_scale());
        }
        new_measurements
    }

    /// Serialize the [`Measurements`] to a JSON file
    pub fn serialize_json(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        serde_json::to_writer(&mut file, &self).unwrap();
    }
}

/// Get the algorithm name from the path
fn get_algorithm_name<Alg, I, O>(_: Alg) -> String
where
    Alg: Fn(&I) -> O,
{
    let path_name = std::any::type_name::<Alg>();
    // Remove the module path
    let name = path_name.split("::").last().unwrap();
    name.into()
}

/// Get the algorithm name from the path
/// This is the mutable version
fn get_algorithm_name_mut<Alg, I, O>(_: Alg) -> String
where
    Alg: Fn(&mut I) -> O,
{
    let path_name = std::any::type_name::<Alg>();
    // Remove the module path
    let name = path_name.split("::").last().unwrap();
    name.into()
}
