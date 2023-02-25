use rand::{Rng, thread_rng};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct StringGen {
    pub name: &'static str, // todo: is the name useful?
    #[serde(skip_serializing)]
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