use rand::{thread_rng, Rng};
use serde::Serialize;
use std::ops::Deref;

#[derive(Clone, Serialize)]
pub struct StringGenFunction {
    pub name: &'static str, // todo: is the name useful?
    #[serde(skip_serializing)]
    pub function: fn(n: usize, char_set: &Vec<char>) -> String,
}

// todo: better method names
pub const METHOD1: StringGenFunction = StringGenFunction {
    name: "Method 1",
    function: create_random_string1,
};

pub const METHOD2: StringGenFunction = StringGenFunction {
    name: "Method 2",
    function: create_random_string2,
};

pub const METHOD3: StringGenFunction = StringGenFunction {
    name: "Method 3",
    function: create_random_string3,
};

pub const METHOD4: StringGenFunction = StringGenFunction {
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
        let char = s[(i - 1) % (q + 1)];
        s.push(char);
    }
    String::from_utf8(s).unwrap()
}

fn create_random_string3(_n: usize, _char_set: &Vec<char>) -> String {
    todo!()
}

fn create_random_string4(n: usize, char_set: &Vec<char>) -> String {
    let mut s = String::with_capacity(n);
    let number_of_chars = char_set.len();
    let mut char = char_set[0];
    for i in 0..n - 1 {
        char = char_set[i % number_of_chars];
        s.push(char);
    }
    s.push(char);
    s
}

impl Deref for StringGenFunction {
    type Target = fn(n: usize, char_set: &Vec<char>) -> String;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

#[derive(Serialize)]
pub struct StringGen {
    pub function: StringGenFunction,
    pub char_set: Vec<char>,
}

impl StringGen {
    /// Creates a new StringGen struct
    ///
    /// # Arguments
    ///
    /// * `function` - The function used to generate the random string
    /// * `char_set` - The character set used to generate the random string
    ///
    /// # Panics
    ///
    /// * Panics if the character set is empty
    /// * Panics if the character set contains repetitions
    /// * Panics if the character set contains non ascii characters
    ///
    /// # Examples
    ///
    /// ```
    /// use time_complexity_plot::random::strings::{METHOD1, StringGen};
    ///
    /// let char_set = vec!['a', 'b', 'c'];
    /// let string_gen = StringGen::new(METHOD1, char_set);
    /// ```
    pub fn new(function: StringGenFunction, char_set: Vec<char>) -> Self {
        assert!(!char_set.is_empty(), "The character set must not be empty");

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
        Self { function, char_set }
    }

    /// Creates a random string using the character set specified in the struct
    ///
    /// # Arguments
    ///
    /// * `n` - The length of the string to be generated
    ///
    /// # Panics
    ///
    /// * Panics if the length of the string to be generated is less than 1
    pub(crate) fn create_random_string(&self, n: usize) -> String {
        assert!(
            n > 0,
            "The length of the string to be generated must be greater than 0"
        );
        (self.function)(n, &self.char_set)
    }
}
