use rand::{thread_rng, Rng};
use std::{fmt::Display, ops::RangeInclusive};

pub trait Distribution: Display {
    fn generate(&self, n: usize) -> Vec<usize>;
}

pub struct Uniform {
    range: RangeInclusive<usize>,
}

impl Uniform {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        Uniform { range }
    }
}

impl Display for Uniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uniform")
    }
}

impl Distribution for Uniform {
    fn generate(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        let a = self.range.start();
        let b = (self.range.end() - self.range.start()) / (n - 1);
        for i in 0..n {
            let x = a + b * i;
            lengths.push(x);
        }
        lengths
    }
}

pub struct UniformRandom {
    range: RangeInclusive<usize>,
}

impl UniformRandom {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        UniformRandom { range }
    }
}

impl Display for UniformRandom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniformRandom")
    }
}

impl Distribution for UniformRandom {
    fn generate(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        for _ in 0..n {
            lengths.push(thread_rng().gen_range(self.range.clone()));
        }
        lengths
    }
}

pub struct Exponential {
    range: RangeInclusive<usize>,
    lambda: f64,
}

impl Exponential {
    pub fn new(range: RangeInclusive<usize>, lambda: f64) -> Self {
        assert!(lambda > 0.0, "Lambda must be grater then zero");
        Exponential { range, lambda }
    }
}

impl Display for Exponential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exponential λ={}", self.lambda)
    }
}

impl Distribution for Exponential {
    fn generate(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        for i in 0..n {
            let x: f64 = i as f64 / (n - 1) as f64;
            let exp_x = exp_distribution(
                x,
                self.lambda,
                *self.range.start() as f64,
                *self.range.end() as f64,
            );
            lengths.push(exp_x as usize);
        }
        lengths
    }
}

pub struct ExponentialRandom {
    range: RangeInclusive<usize>,
    lambda: f64,
}

impl ExponentialRandom {
    pub fn new(range: RangeInclusive<usize>, lambda: f64) -> Self {
        assert!(lambda > 0.0, "Lambda must be grater then zero");
        ExponentialRandom { range, lambda }
    }
}

impl Display for ExponentialRandom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExponentialRandom λ={}", self.lambda)
    }
}

impl Distribution for ExponentialRandom {
    fn generate(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        for _ in 0..n {
            let x: f64 = thread_rng().gen::<f64>();
            let exp_x = exp_distribution(
                x,
                self.lambda,
                *self.range.start() as f64,
                *self.range.end() as f64,
            );
            lengths.push(exp_x as usize);
        }
        lengths
    }
}

fn exp_distribution(u: f64, lambda: f64, min: f64, max: f64) -> f64 {
    let a = (-lambda * min).exp();
    let b = (-lambda * max).exp();
    let log_exp = (a - (a - b) * u).ln();
    -(1.0 / lambda) * log_exp
}
