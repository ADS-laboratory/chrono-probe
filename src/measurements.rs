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

fn get_time_with_resolution<F>(f: F, string: String, relative_error: f32, resolution: Duration) -> Duration
    where
        F: Fn(&[u8]) -> (),
{
    let mut n = 0;
    let start = Instant::now();
    loop {
        f(string.as_bytes());
        n += 1;
        let end = start.elapsed();
        if end > resolution * ((1.0 / relative_error) + 1.0) as u32 {
            return end / n;
        }
    }
}

fn get_time<F>(f: F, string: String, relative_error: f32) -> Duration
    where
        F: Fn(&[u8]) -> (),
{
    let resolution = get_resolution();
    get_time_with_resolution(f, string, relative_error, resolution)
}

pub fn get_times<F>(f: F, strings: Vec<String>, relative_error: f32) -> Vec<Duration>
    where
        F: Fn(&[u8]) -> (),
{
    let n = strings.len();
    let mut times = Vec::with_capacity(n);
    let resolution = get_resolution();
    for string in strings {
        let time = get_time_with_resolution(&f, string, relative_error, resolution);
        times.push(time);
    }
    times
}