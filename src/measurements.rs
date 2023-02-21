use std::time::{Duration, Instant};
use crate::algorithms::Algorithm;
use crate::random::strings::GeneratedStrings;

#[derive(Clone)]
pub struct Point {
    pub length_of_string: usize,
    pub time: Duration,
}

#[derive(Clone)]
pub struct Measurement {
    pub algorithm_name: &'static str,
    pub measurement: Vec<Point>,
}

#[derive(Clone)]
pub struct Measurements {
    pub measurements: Vec<Measurement>,
    pub relative_error: f32,
    pub resolution: Duration,
}

/// Estimates the resolution of the clock
pub fn get_resolution() -> Duration {
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
pub fn get_average_resolution() -> Duration {
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
fn get_time(f: &Algorithm, string: &[u8], relative_error: f32, resolution: Duration) -> Point {
    // todo: make this more accurate by subtracting the time it takes to run the lines of code after the function call
    let mut n = 0;
    let min_time_measurable = resolution * ((1.0 / relative_error) + 1.0) as u32;
    let mut end: Duration;
    let start = Instant::now();
    loop {
        (f.function)(string);
        end = start.elapsed();
        n += 1;
        if end > min_time_measurable {
            break
        }
    }
    return Point {
        length_of_string: string.len(),
        time: end / n,
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
fn get_times(f: &Algorithm, strings: &Vec<String>, relative_error: f32, resolution: Duration) -> Measurement {
    let n = strings.len();
    let mut times = Vec::with_capacity(n);
    for (i, string) in strings.iter().enumerate() {
        let time = get_time(f, string.as_bytes(), relative_error, resolution);
        times.push(time);
        if i % (n / 20) == 0 {
            println!("{}%", (i+n/20) * 100 / n);
        }
    }
    Measurement {
        algorithm_name: f.name,
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
pub fn measure(strings: &GeneratedStrings, algorithms: &Vec<Algorithm>, relative_error: f32) -> Measurements {
    let resolution = get_average_resolution();
    let mut results = Vec::with_capacity(algorithms.len());
    for (i, algorithm) in algorithms.iter().enumerate() {
        println!("\n\nProcessing {} ({}/{})...\n", algorithm.name, i+1, algorithms.len());
        let measurement = get_times(algorithm, &strings.strings, relative_error, resolution);
        results.push(measurement);
    }
    Measurements {
        measurements: results,
        relative_error,
        resolution,
    }
}

// Some useful functions for Measurements
impl Measurement {
    /// Get the maximum time it took to run the function
    pub fn max_time(&self) -> Duration {
        let mut max = Duration::ZERO;
        for point in self.measurement.iter() {
            if point.time > max {
                max = point.time;
            }
        }
        max
    }

    /// Get the minimum time it took to run the function
    pub fn min_time(&self) -> Duration {
        let mut min = Duration::MAX;
        for point in self.measurement.iter() {
            if point.time < min {
                min = point.time;
            }
        }
        min
    }

    /// Get the maximum length of the strings passed to the function
    pub fn max_length(&self) -> usize {
        let mut max = 0;
        for point in self.measurement.iter() {
            if point.length_of_string > max {
                max = point.length_of_string;
            }
        }
        max
    }

    /// Get the minimum length of the strings passed to the function
    pub fn min_length(&self) -> usize {
        let mut min = usize::MAX;
        for point in self.measurement.iter() {
            if point.length_of_string < min {
                min = point.length_of_string;
            }
        }
        min
    }

    pub fn linear_regression(&self) -> (f32, f32) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        let mut n = 0.0;
        for point in self.measurement.iter() {
            let x = point.length_of_string as f32;
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
            algorithm_name: self.algorithm_name,
            measurement: Vec::with_capacity(self.measurement.len()),
        };
        for point in self.measurement.iter() {

            new_measurement.measurement.push(Point {
                length_of_string: (point.length_of_string as f32).log2() as usize,
                time: Duration::from_micros((point.time.as_micros() as f32).log2() as u64),
            });
        }
        new_measurement
    }
}