use rand::{Rng, thread_rng};

pub struct Exp {
    min_value: f64,
    max_value: f64,
    char_set: Vec<char>,
}

pub enum StringGen {
    // todo: better names
    Method1,
    Method2,
    Method3,
    Method4,
}

impl Exp {
    pub fn new(min_value: i32, max_value: i32, char_set: Vec<char>) -> Exp {
        Exp {
            min_value: min_value as f64,
            max_value: max_value as f64,
            // todo: check for repetitions in char_set
            // todo: check for non ascii characters
            char_set,
        }
    }

    pub fn sample_int(&self) -> usize {
        let x: f64 = thread_rng().gen::<f64>();
        let scaled_x = self.min_value * (self.max_value / self.min_value).powf(x);
        scaled_x.floor() as usize
    }

    pub fn create_random_string1(&self) -> String {
        let n = self.sample_int();
        let mut s = String::with_capacity(n);
        let number_of_chars = self.char_set.len();
        for _ in 0..n {
            // generate random character
            let char_index = thread_rng().gen_range(0..number_of_chars);
            let char = self.char_set[char_index];
            s.push(char);
        }
        s
    }

    pub fn create_random_string2(&self) -> String {
        let n = self.sample_int();
        let mut s: Vec<u8> = vec![];
        let number_of_chars = self.char_set.len();
        let q = thread_rng().gen_range(0..n);
        for _ in 0..q {
            // generate random character
            let char_index = thread_rng().gen_range(0..number_of_chars);
            let char = self.char_set[char_index];
            s.push(char as u8);
        }
        for i in q..n {
            // todo: use another type instead of String
            let char = s[(i - 1) % (q + 1)];
            s.push(char);
        }
        String::from_utf8(s).unwrap()
    }

    pub fn create_random_string3(&self) -> String {
        "todo".to_string()
    }

    pub fn create_random_string4(&self) -> String {
        "todo".to_string()
    }

    pub fn create_random_string(&self, method: &StringGen) -> String {
        match method {
            StringGen::Method1 => self.create_random_string1(),
            StringGen::Method2 => self.create_random_string2(),
            StringGen::Method3 => self.create_random_string3(),
            StringGen::Method4 => self.create_random_string4(),
        }
    }

    pub fn create_random_strings(&self, method: StringGen, n: usize) -> Vec<String> {
        let mut strings = Vec::with_capacity(n);
        let ref_method = &method;
        println!("\n\nGenerating strings...\n");
        for i in 0..n {
            // todo: match only one time
            strings.push(self.create_random_string(ref_method));
            if i % (n / 20) == 0 {
                println!("{}%", (i+n/20) * 100 / n);
            }
        }
        strings
    }
}