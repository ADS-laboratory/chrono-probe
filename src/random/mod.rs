pub mod strings;
pub mod lengths;

use lengths::LengthDistribution;
use strings::StringGen;

#[derive(Clone)]
pub struct Distribution {
    pub length_distribution: LengthDistribution,
    pub min_value: f64,
    pub max_value: f64,
}

impl Distribution {
    /// Creates a new distribution
    /// 
    /// # Arguments
    /// 
    /// * `length_distribution` - The distribution of the lengths of the strings
    /// * `min_value` - The minimum value of the length of the strings
    /// * `max_value` - The maximum value of the length of the strings
    /// 
    /// # Examples
    /// 
    /// ```
    /// use time_complexity_plot::random::{Distribution,
    ///                                 lengths::{LengthDistribution, EXPONENTIAL}};
    ///
    /// let distribution = Distribution::new(EXPONENTIAL, 1000, 500_000);
    /// ```
    pub fn new(length_distribution: LengthDistribution, min_value: i32, max_value: i32) -> Distribution {
        Distribution {
            length_distribution,
            min_value: min_value as f64,
            max_value: max_value as f64,
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
    fn create_random_string(&self, n: usize, method: &StringGen, char_set: &Vec<char>) -> String {
        if n < 1 {
            panic!("The length of the string to be generated must be greater than 0");
        }
        (method.function)(n, char_set)
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
    /// * Panics if the character set contains repetitions
    /// * Panics if the character set contains non ascii characters
    /// 
    /// # Examples
    /// 
    /// ```
    /// use time_complexity_plot::random::{Distribution, GeneratedStrings,
    ///                                 lengths::{LengthDistribution, EXPONENTIAL},
    ///                                 strings::{StringGen, METHOD1}};
    ///
    /// let distribution = Distribution::new(EXPONENTIAL, 1000, 500_000);
    /// let generated_strings = distribution.create_random_strings(METHOD1, vec!['a', 'b'], 100);
    /// ```
    pub fn create_random_strings(&self, generation_method: StringGen, char_set: Vec<char>, n: usize) -> GeneratedStrings {
        if n < 1 {
            panic!("The number of strings to be generated must be greater than 0");
        }
        if char_set.is_empty() {
            panic!("The character set must not be empty");
        }
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
        let mut strings = Vec::with_capacity(n);
        let length_distribution = self.length_set(n);
        println!("\n\nGenerating strings...\n");
        let mut j: usize = 0; // used to update progress percentage
        for i in length_distribution {
            strings.push(self.create_random_string(i, &generation_method, &char_set));
            j += 1;
            if j % (n / 20) == 0 {
                println!("{}%", (j+n/20) * 100 / n);
            }
        }
        GeneratedStrings {
            strings,
            generation_method,
            char_set: char_set.to_vec(),
            distribution: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct GeneratedStrings {
    pub strings: Vec<String>,
    pub distribution: Distribution,
    pub char_set: Vec<char>,
    pub generation_method: StringGen,
}