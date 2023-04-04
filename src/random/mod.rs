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

    /// Creates a vector of strings of length `n`.
    /// Strings are generated with the given distribution and generation method.
    /// Only one string is generated for each length.
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
        self.create_random_strings_with_repetitions(n, 1)
    }

    /// Creates a vector of strings of length `n` with #`repetitions` strings for each length.
    /// Strings are generated with the given distribution and generation method.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of strings to be generated
    /// * `repetitions` - The number of strings to be generated for each length
    ///
    /// # Panics
    ///
    /// * Panics if the number of strings to be generated is less than 1
    /// * Panics if the number of repetitions is less than 1
    ///
    pub fn create_random_strings_with_repetitions(
        &self,
        n: usize,
        repetitions: usize,
    ) -> GeneratedStrings {
        assert!(
            n > 0,
            "The number of strings to be generated must be greater than 0"
        );
        assert!(
            repetitions > 0,
            "The number of repetitions must be greater than 0"
        );
        let mut strings = Vec::new();
        let length_distribution = self.distribution.create_length_set(n);
        println!("\n\nGenerating strings...\n");
        #[cfg(feature = "debug")]
        // used to update progress percentage
        let mut j: usize = 0;
        for string_size in length_distribution {
            let mut strings_with_same_size = Vec::new();
            for _ in 0..repetitions {
                strings_with_same_size
                    .push(self.generation_method.create_random_string(string_size));
            }
            strings.push(strings_with_same_size);
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
    pub strings: Vec<Vec<String>>,
    pub builder: &'a StringsBuilder,
}
