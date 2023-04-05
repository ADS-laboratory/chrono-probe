use crate::input::{Input, InputSet};
use serde::Serialize;
use std::fs::File;
use std::time::{Duration, Instant};

#[derive(Serialize)]
pub struct Point {
    pub size: usize,
    pub time: Duration,
}

#[derive(Serialize)]
pub struct Measurement {
    pub algorithm_name: String,
    pub measurement: Vec<Point>,
}

#[derive(Serialize)]
pub struct Measurements {
    pub measurements: Vec<Measurement>,
    pub relative_error: f32,
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
fn get_time<I: Input + Clone, O>(
    f: &fn(I) -> O,
    input: I,
    relative_error: f32,
    resolution: Duration,
) -> Duration {
    let mut n = 0;
    // The minimum time measurable
    let min_time_measurable = resolution * ((1.0 / relative_error) + 1.0) as u32;
    let mut end: Duration;
    let mut start = Instant::now();
    loop {
        // Measure the time it takes to clone the input
        let start_input_clone = Instant::now();
        let input_cloned = input.clone();
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
fn get_time_same_length<I: Input + Clone, O>(
    f: &fn(I) -> O,
    inputs: &Vec<I>,
    relative_error: f32,
    resolution: Duration,
) -> Point {
    let mut total_time = Duration::ZERO;
    let size = inputs[0].get_size();
    for input in inputs {
        let time = get_time(f, input.clone(), relative_error, resolution);
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
fn get_times<I: Input + Clone, O>(
    f: &fn(I) -> O,
    inputs: &InputSet<I>,
    relative_error: f32,
    resolution: Duration,
) -> Measurement {
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
        algorithm_name: format!("{:?}", f),
        measurement: times,
    }
}

/// Measures the time it takes to run different functions given a vector of inputs
///
/// # Arguments
///
/// * `strings` - The vector of strings to pass to the functions
/// * `algorithms` - The vector of functions to measure
/// * `relative_error` - The required relative error of the measurements
///
/// # Example
///
/// ```
/// use time_complexity_plot::{
///     algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART},
///     measurements::measure,
///     random::{
///         lengths::{LengthDistribution, EXPONENTIAL},
///         strings::{StringGen, METHOD1},
///         StringsBuilder,
///     },
/// };
///
/// let length_distribution = LengthDistribution::new(EXPONENTIAL, 1000, 500_000);
/// let string_gen = StringGen::new(METHOD1, vec!['a', 'b']);
/// let strings_builder = StringsBuilder::new(length_distribution, string_gen);
/// let strings = strings_builder.create_random_strings(100);
/// let algorithms = vec![PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];
/// let measurements = measure(&strings, &algorithms, 0.01);
/// ```
pub fn measure<I: Input + Clone, O>(
    inputs: &InputSet<I>,
    algorithms: &Vec<fn(I) -> O>,
    relative_error: f32,
) -> Measurements {
    assert!(relative_error > 0.0, "Relative error must be positive");
    let resolution = get_average_resolution();
    let mut results = Vec::with_capacity(algorithms.len());
    for (_i, algorithm) in algorithms.iter().enumerate() {
        #[cfg(feature = "debug")]
        println!(
            "\n\nProcessing {:?} ({}/{})...\n",
            algorithm,
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

    pub fn linear_regression(&self) -> (f32, f32) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        let mut n = 0.0;
        for point in self.measurement.iter() {
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

    pub fn log_scale(&self) -> Self {
        let mut new_measurement = Measurement {
            algorithm_name: self.algorithm_name.clone(),
            measurement: Vec::with_capacity(self.measurement.len()),
        };
        for point in self.measurement.iter() {
            new_measurement.measurement.push(Point {
                size: (point.size as f32).log2() as usize,
                time: Duration::from_micros((point.time.as_micros() as f32).log2() as u64),
            });
        }
        new_measurement
    }
}

impl Measurements {
    pub fn max_time(&self) -> Duration {
        self.measurements
            .iter()
            .max_by_key(|measurement| measurement.max_time())
            .unwrap()
            .max_time()
    }

    pub fn min_time(&self) -> Duration {
        self.measurements
            .iter()
            .min_by_key(|measurement| measurement.min_time())
            .unwrap()
            .min_time()
    }

    pub fn max_length(&self) -> usize {
        self.measurements
            .iter()
            .max_by_key(|measurement| measurement.max_length())
            .unwrap()
            .max_length()
    }

    pub fn min_length(&self) -> usize {
        self.measurements
            .iter()
            .min_by_key(|measurement| measurement.min_length())
            .unwrap()
            .min_length()
    }

    pub fn log_scale(&self) -> Self {
        let mut new_measurements = Measurements {
            measurements: Vec::with_capacity(self.measurements.len()),
            relative_error: self.relative_error,
            resolution: self.resolution,
        };
        for measurement in self.measurements.iter() {
            new_measurements.measurements.push(measurement.log_scale());
        }
        new_measurements
    }

    pub fn serialize_json(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        serde_json::to_writer(&mut file, &self).unwrap();
    }
}
