use std::ptr::null_mut;
use std::time::{Duration, Instant};
use crate::algorithms::Algorithm;

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

fn get_time_with_resolution(f: &Algorithm, string: &[u8], relative_error: f32, resolution: Duration) -> Duration {
    let mut n = 0;
    let start = Instant::now();
    loop {
        (f.function)(string);
        n += 1;
        let end = start.elapsed();
        if end > resolution * ((1.0 / relative_error) + 1.0) as u32 {
            return end / n;
        }
    }
}

fn get_time(f: &Algorithm, string: &[u8], relative_error: f32) -> Duration {
    let resolution = get_resolution();
    get_time_with_resolution(f, string, relative_error, resolution)
}

pub fn get_times_with_resolution(f: &Algorithm, strings: &Vec<String>, relative_error: f32, resolution: Duration) -> Vec<Duration> {
    let n = strings.len();
    let mut times = Vec::with_capacity(n);
    for string in strings {
        let time = get_time_with_resolution(f, string.as_bytes(), relative_error, resolution);
        times.push(time);
    }
    times
}

pub fn get_times(f: &Algorithm, strings: &Vec<String>, relative_error: f32) -> Vec<Duration> {
    let resolution = get_resolution();
    get_times_with_resolution(f, strings, relative_error, resolution)
}