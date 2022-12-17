use std::time::{Duration, Instant};

/// Measures the resolution of the clock
pub fn get_resolution() -> Duration {
    // A measurement of a monotonically nondecreasing clock
    let start = Instant::now();
    loop {
        let end = start.elapsed();
        if end != Duration::ZERO {
            return start.elapsed();
        }
    }
}

pub fn get_times<F>(f: F, strings: Vec<String>) -> Vec<Duration>
    where
        F: Fn(&[u8]) -> (),
{
    let n = strings.len();
    let mut times = Vec::with_capacity(n);
    for string in strings {
        let start = Instant::now();
        f(string.as_bytes());
        times.push(start.elapsed());
    }
    times
}