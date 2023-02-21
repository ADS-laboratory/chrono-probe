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

/// Measures the resolution of the clock
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

pub fn get_average_resolution() -> Duration {
    let mut sum = Duration::ZERO;
    for _ in 0..100 {
        sum += get_resolution();
    }
    sum / 100
}

fn get_time_with_resolution(f: &Algorithm, string: &[u8], relative_error: f32, resolution: Duration) -> Point {
    let mut n = 0;
    let start = Instant::now();
    loop {
        (f.function)(string);
        n += 1;
        let end = start.elapsed();
        if end > resolution * ((1.0 / relative_error) + 1.0) as u32 {
            return Point {
                length_of_string: string.len(),
                time: end / n,
            };
        }
    }
}

pub fn get_time(f: &Algorithm, string: &[u8], relative_error: f32) -> Point {
    let resolution = get_average_resolution();
    get_time_with_resolution(f, string, relative_error, resolution)
}

fn get_times_with_resolution(f: &Algorithm, strings: &Vec<String>, relative_error: f32, resolution: Duration) -> Measurement {
    let n = strings.len();
    let mut times = Vec::with_capacity(n);
    for (i, string) in strings.iter().enumerate() {
        let time = get_time_with_resolution(f, string.as_bytes(), relative_error, resolution);
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

pub fn get_times(f: &Algorithm, strings: &Vec<String>, relative_error: f32) -> Measurement {
    let resolution = get_average_resolution();
    get_times_with_resolution(f, strings, relative_error, resolution)
}

fn measure_with_resolution(strings: &GeneratedStrings, algorithms: &Vec<Algorithm>, relative_error: f32, resolution: Duration) -> Measurements {
    let mut results = Vec::with_capacity(algorithms.len());
    for (i, algorithm) in algorithms.iter().enumerate() {
        println!("\n\nProcessing {} ({}/{})...\n", algorithm.name, i+1, algorithms.len());
        let measurement = get_times_with_resolution(algorithm, &strings.strings, relative_error, resolution);
        results.push(measurement);
    }
    Measurements {
        measurements: results,
        relative_error,
        resolution,
    }
}

pub fn measure(strings: &GeneratedStrings, algorithms: &Vec<Algorithm>, relative_error: f32) -> Measurements {
    let resolution = get_average_resolution();
    measure_with_resolution(strings, algorithms, relative_error, resolution)
}

impl Measurement {
    pub fn max_time(&self) -> Duration {
        let mut max = Duration::ZERO;
        for point in self.measurement.iter() {
            if point.time > max {
                max = point.time;
            }
        }
        max
    }

    pub fn min_time(&self) -> Duration {
        let mut min = Duration::MAX;
        for point in self.measurement.iter() {
            if point.time < min {
                min = point.time;
            }
        }
        min
    }

    pub fn max_length(&self) -> usize {
        let mut max = 0;
        for point in self.measurement.iter() {
            if point.length_of_string > max {
                max = point.length_of_string;
            }
        }
        max
    }

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