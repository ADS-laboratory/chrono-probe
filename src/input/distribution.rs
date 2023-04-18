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
        let b = (self.range.end() - self.range.start()) / n;
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

// TODO: Maybe add a lambda parameter
pub struct Exponential {
    range: RangeInclusive<usize>,
}

impl Exponential {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        Exponential { range }
    }
}

impl Display for Exponential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exponential")
    }
}

impl Distribution for Exponential {
    fn generate(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        let a = *self.range.start() as f64;
        let b = ((self.range.end() / self.range.start()) as f64).powf(1.0 / n as f64);
        for i in 0..n {
            let x = a * b.powf(i as f64);
            lengths.push(x as usize);
        }
        lengths
    }
}

pub struct ExponentialRandom {
    range: RangeInclusive<usize>,
}

impl ExponentialRandom {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        ExponentialRandom { range }
    }
}

impl Display for ExponentialRandom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExponentialRandom")
    }
}

impl Distribution for ExponentialRandom {
    fn generate(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        let a = *self.range.start() as f64;
        let b = ((self.range.end() / self.range.start()) as f64).powf(1.0 / n as f64);
        for _ in 0..n {
            let x: f64 = thread_rng().gen::<f64>();
            let scaled_x = a * b.powf(x);
            lengths.push(scaled_x as usize);
        }
        lengths
    }
}
