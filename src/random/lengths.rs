use rand::{Rng, thread_rng};
use crate::random::strings::{GeneratedStrings, StringGen};

#[derive(Clone)]
/// A rapresentation of the function that generates a distribution of lengths of strings
pub struct LengthDistribution {
    pub name: &'static str, // todo: is the name useful?
    pub function: fn(n: usize, min: f64, max: f64) -> Vec<usize>,
}

/// Uniform distribution of lengths
pub const UNIFORM: LengthDistribution = LengthDistribution {
    name: "Uniform",
    function: uniform_length_set,
};

/// Exponential distribution of lengths
pub const EXPONENTIAL: LengthDistribution = LengthDistribution {
    name: "Exponential",
    function: exponential_length_set,
};

/// Uniform random distribution of lengths
pub const UNIFORM_RANDOM: LengthDistribution = LengthDistribution {
    name: "Uniform random",
    function: uniform_random_length_set,
};

/// Exponential random distribution of lengths
pub const EXPONENTIAL_RANDOM: LengthDistribution = LengthDistribution {
    name: "Exponential random",
    function: exponential_random_length_set,
};

/// Creates a vector of lengths of strings using an uniform distribution
/// 
/// # Arguments
/// 
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn uniform_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    let a = min;
    let b = (max - min) / n as f64;
    for i in 0..n {
        let x = a + b * (i as f64);
        let final_x = x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

/// Creates a vector of lengths of strings using an exponential distribution
/// 
/// # Arguments
/// 
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn exponential_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    let a = min;
    let b = (max / min).powf(1.0 / n as f64);
    for i in 0..n {
        let x = a * b.powf(i as f64);
        let final_x = x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

/// Creates a vector of lengths of strings using an uniform random distribution
/// 
/// # Arguments
/// 
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn uniform_random_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    for _ in 0..n {
        let x = thread_rng().gen_range(min..max);
        let final_x = x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

/// Creates a vector of lengths of strings using an exponential random distribution
/// 
/// # Arguments
/// 
/// * `n` - The number of lengths to be generated
/// * `min` - The minimum length of a string
/// * `max` - The maximum length of a string
fn exponential_random_length_set(n: usize, min: f64, max: f64) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(n);
    for _ in 0..n {
        let x: f64 = thread_rng().gen::<f64>();
        let scaled_x = min * (max / min).powf(x);
        let final_x = scaled_x.floor() as usize;
        lengths.push(final_x);
    }
    lengths
}

#[derive(Clone)]
pub struct Distribution {
    pub length_distribution: LengthDistribution,
    pub min_value: f64,
    pub max_value: f64,
    pub char_set: Vec<char>,
}

impl Distribution {
    /// Creates a new distribution
    /// 
    /// # Arguments
    /// 
    /// * `length_distribution` - The distribution of the lengths of the strings
    /// * `min_value` - The minimum value of the length of the strings
    /// * `max_value` - The maximum value of the length of the strings
    /// * `char_set` - The set of characters that the strings can contain
    /// 
    /// # Panics
    /// 
    /// * Panics if the character set contains repetitions
    /// * Panics if the character set contains non ascii characters
    /// 
    /// # Examples
    /// 
    /// ```
    /// use fractional_period::random::{Distribution, LengthDistribution};
    ///
    /// let distribution = Distribution::new(LengthDistribution::Uniform, 1000, 500_000, vec!['a', 'b']);
    /// ```
    pub fn new(length_distribution: LengthDistribution, min_value: i32, max_value: i32, char_set: Vec<char>) -> Distribution {
        // checking for repetitions in char_set and non ascii characters
        let mut char_set_sorted = char_set.clone();
        char_set_sorted.sort_by(|a, b| b.cmp(a));
        for i in 0..char_set_sorted.len() - 1 {
            if char_set_sorted[i] == char_set_sorted[i + 1] {
                panic!("The character set contains repetitions");
            }
            if char_set_sorted[i] as u32 > 127 {
                panic!("The character set contains non ascii characters");
            }
        }
        Distribution {
            length_distribution,
            min_value: min_value as f64,
            max_value: max_value as f64,
            char_set,
        }
    }

    /// Creates a vector of lengths of strings using the distribution specified in the struct
    /// 
    /// # Arguments
    /// 
    /// * `n` - The number of lengths to be generated
    /// 
    /// # Panics
    /// 
    /// * Panics if the number of lengths to be generated is less than 1
    fn length_set(&self, n: usize) -> Vec<usize> {
        if n < 1 {
            panic!("The number of lengths to be generated must be greater than 0");
        }
        (self.length_distribution.function)(n, self.min_value, self.max_value)
    }

    /// Creates a random string using the character set specified in the struct
    /// 
    /// # Arguments
    /// 
    /// * `n` - The length of the string to be generated
    /// * `method` - The method to be used to generate the string
    /// 
    /// # Panics
    /// 
    /// * Panics if the length of the string to be generated is less than 1
    fn create_random_string(&self, n: usize, method: &StringGen) -> String {
        if n < 1 {
            panic!("The length of the string to be generated must be greater than 0");
        }
        (method.function)(n, &self.char_set)
    }

    /// Creates a vector of random strings using the character set specified in the struct
    /// 
    /// # Arguments
    /// 
    /// * `generation_method` - The method to be used to generate the strings
    /// * `n` - The number of strings to be generated
    /// 
    /// # Panics
    /// 
    /// * Panics if the number of strings to be generated is less than 1
    /// * Panics if the character set is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use fractional_period::random::{Distribution, LengthDistribution, StringGen};
    ///
    /// let distribution = Distribution::new(LengthDistribution::Uniform, 1000, 500_000, vec!['a', 'b']);
    /// let generated_strings = distribution.create_random_strings(StringGen::Method1, 100);
    /// ```
    pub fn create_random_strings(&self, generation_method: StringGen, n: usize) -> GeneratedStrings {
        if n < 1 {
            panic!("The number of strings to be generated must be greater than 0");
        }
        if self.char_set.is_empty() {
            panic!("The character set must not be empty");
        }
        let mut strings = Vec::with_capacity(n);
        let length_distribution = self.length_set(n);
        println!("\n\nGenerating strings...\n");
        let mut j: usize = 0; // used to update progress percentage
        for i in length_distribution {
            strings.push(self.create_random_string(i, &generation_method));
            j += 1;
            if j % (n / 20) == 0 {
                println!("{}%", (j+n/20) * 100 / n);
            }
        }
        GeneratedStrings {
            strings,
            generation_method,
            distribution: self.clone(),
        }
    }
}