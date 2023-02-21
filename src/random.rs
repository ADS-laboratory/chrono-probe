use rand::{Rng, thread_rng};

// todo: is it better to split random.rs into more files?
#[derive(Clone)]
pub struct LengthDistribution {
    pub name: &'static str, // todo: is the name useful?
    pub function: fn(n: usize, min: f64, max: f64) -> Vec<usize>,
}

pub const UNIFORM: LengthDistribution = LengthDistribution {
    name: "Uniform",
    function: uniform_length_set,
};

pub const EXPONENTIAL: LengthDistribution = LengthDistribution {
    name: "Exponential",
    function: exponential_length_set,
};

pub const UNIFORM_RANDOM: LengthDistribution = LengthDistribution {
    name: "Uniform random",
    function: uniform_random_length_set,
};

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

pub struct StringGen {
    pub name: &'static str, // todo: is the name useful?
    pub function: fn(n: usize, char_set: &Vec<char>) -> String,
}

// todo: better method names
pub const METHOD1: StringGen = StringGen {
    name: "Method 1",
    function: create_random_string1,
};

pub const METHOD2: StringGen = StringGen {
    name: "Method 2",
    function: create_random_string2,
};

pub const METHOD3: StringGen = StringGen {
    name: "Method 3",
    function: create_random_string3,
};

pub const METHOD4: StringGen = StringGen {
    name: "Method 4",
    function: create_random_string4,
};

fn create_random_string1(n: usize, char_set: &Vec<char>) -> String {
    let mut s = String::with_capacity(n);
    let number_of_chars = char_set.len();
    for _ in 0..n {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char);
    }
    s
}

fn create_random_string2(n: usize, char_set: &Vec<char>) -> String {
    let mut s: Vec<u8> = vec![];
    let number_of_chars = char_set.len();
    let q = thread_rng().gen_range(0..n);
    for _ in 0..q {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char as u8);
    }
    for i in q..n {
        // todo: use another type instead of String
        let char = s[(i - 1) % (q + 1)];
        s.push(char);
    }
    String::from_utf8(s).unwrap()
}

fn create_random_string3(_n: usize, _char_set: &Vec<char>) -> String {
    "todo".to_string()
}

fn create_random_string4(n: usize, char_set: &Vec<char>) -> String {
    let mut s = String::with_capacity(n);
    let number_of_chars = char_set.len();
    let mut char = char_set[0];
    for i in 0..n-1 {
        char = char_set[i % number_of_chars];
        s.push(char);
    }
    s.push(char);
    s
}

#[derive(Clone)]
pub struct Distribution {
    pub length_distribution: LengthDistribution,
    pub min_value: f64,
    pub max_value: f64,
    pub char_set: Vec<char>,
}

pub struct GeneratedStrings {
    pub strings: Vec<String>,
    pub distribution: Distribution,
    pub generation_method: StringGen,
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
    /// use random::Distribution;
    /// use random::LengthDistribution;
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
    /// use random::Distribution;
    /// use random::LengthDistribution;
    /// use random::StringGen;
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