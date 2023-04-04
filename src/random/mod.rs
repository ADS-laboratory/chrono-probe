#![allow(clippy::explicit_counter_loop)]
pub mod lengths;
pub mod strings;
use lengths::LengthDistribution;
use serde::Serialize;
use strings::StringGen;

#[derive(Serialize)]
pub struct StringsBuilder {
    pub distribution: LengthDistribution,
    pub generation_method: StringGen,
}

impl StringsBuilder {
    /// Creates a new string builder
    ///
    /// # Arguments
    ///
    /// * `distribution` - The distribution of the lengths of the strings
    /// * `generation_method` - The method used to generate the strings
    pub fn new(distribution: LengthDistribution, generation_method: StringGen) -> Self {
        Self {
            distribution,
            generation_method,
        }
    }

    /// Creates a vector of random strings using the character set specified in the struct
    ///
    /// # Arguments
    ///
    /// * `n` - The number of strings to be generated
    ///
    /// # Panics
    ///
    /// * Panics if the number of strings to be generated is less than 1
    ///
    /// # Examples
    ///
    /// ```
    ///use time_complexity_plot::random::{
    ///     lengths::{LengthDistribution, EXPONENTIAL},
    ///     strings::{StringGen, METHOD1},
    ///     StringsBuilder,
    /// };
    ///
    /// let length_distribution = LengthDistribution::new(EXPONENTIAL, 1000, 500_000);
    /// let string_gen = StringGen::new(METHOD1, vec!['a', 'b']);
    /// let strings_builder = StringsBuilder::new(length_distribution, string_gen);
    /// let strings = strings_builder.create_random_strings(100);
    /// ```
    pub fn create_random_strings(&self, n: usize) -> GeneratedStrings {
        assert!(
            n > 0,
            "The number of strings to be generated must be greater than 0"
        );
        let mut strings = Vec::new();
        let length_distribution = self.distribution.create_length_set(n);
        println!("\n\nGenerating strings...\n");
        #[cfg(feature = "debug")]
        // used to update progress percentage
        let mut j: usize = 0;
        for string_size in length_distribution {
            strings.push(self.generation_method.create_random_string(string_size));
            #[cfg(feature = "debug")]
            {
                if j % (n / 20) == 0 {
                    println!("{}%", j * 100 / n);
                }
                j += 1;
            }
        }
        GeneratedStrings {
            strings,
            builder: self,
        }
    }
}

#[derive(Serialize)]
pub struct GeneratedStrings<'a> {
    #[serde(skip_serializing)]
    pub strings: Vec<String>,
    pub builder: &'a StringsBuilder,
}
