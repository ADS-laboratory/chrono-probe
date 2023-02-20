use rand::{Rng, thread_rng};

pub enum StringGen {
    // todo: better names
    Method1,
    Method2,
    Method3,
    Method4,
}

#[derive(Copy, Clone)]
pub enum LengthDistribution {
    Uniform,
    Exponential,
    UniRand,
    ExpRand,
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
    pub fn new(length_distribution: LengthDistribution, min_value: i32, max_value: i32, char_set: Vec<char>) -> Distribution {
        Distribution {
            length_distribution,
            min_value: min_value as f64,
            max_value: max_value as f64,
            // todo: check for repetitions in char_set
            // todo: check for non ascii characters
            char_set,
        }
    }

    fn uniform_length_set(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        let a = self.min_value;
        let b = (self.max_value - self.min_value) / n as f64;
        for i in 0..n {
            let x = a + b * (i as f64);
            let final_x = x.floor() as usize;
            lengths.push(final_x);
        }
        lengths
    }

    fn exponential_length_set(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        let a = self.min_value;
        let b = (self.max_value / self.min_value).powf(1.0 / n as f64);
        for i in 0..n {
            let x = a * b.powf(i as f64);
            let final_x = x.floor() as usize;
            lengths.push(final_x);
        }
        lengths
    }

    fn uniform_random_length_set(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        for _ in 0..n {
            let x: f64 = thread_rng().gen::<f64>();
            let final_x = x.floor() as usize;
            lengths.push(final_x);
        }
        lengths
    }

    fn exponential_random_length_set(&self, n: usize) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(n);
        for _ in 0..n {
            let x: f64 = thread_rng().gen::<f64>();
            let scaled_x = self.min_value * (self.max_value / self.min_value).powf(x);
            let final_x = scaled_x.floor() as usize;
            lengths.push(final_x);
        }
        lengths
    }

    pub fn length_set(&self, n: usize) -> Vec<usize> {
        match self.length_distribution {
            LengthDistribution::Uniform => self.uniform_length_set(n),
            LengthDistribution::Exponential => self.exponential_length_set(n),
            LengthDistribution::UniRand => self.uniform_random_length_set(n),
            LengthDistribution::ExpRand => self.exponential_random_length_set(n),
        }
    }

    pub fn create_random_string1(&self, n: usize) -> String {
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

    pub fn create_random_string2(&self, n: usize) -> String {
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

    pub fn create_random_string3(&self, n: usize) -> String {
        "todo".to_string()
    }

    pub fn create_random_string4(&self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        let number_of_chars = self.char_set.len();
        let mut char = self.char_set[0];
        for i in 0..n-1 {
            char = self.char_set[i % number_of_chars];
            s.push(char);
        }
        s.push(char);
        s
    }

    pub fn create_random_string(&self, n: usize, method: &StringGen) -> String {
        match method {
            StringGen::Method1 => self.create_random_string1(n),
            StringGen::Method2 => self.create_random_string2(n),
            StringGen::Method3 => self.create_random_string3(n),
            StringGen::Method4 => self.create_random_string4(n),
        }
    }

    pub fn create_random_strings(&self, generation_method: StringGen, n: usize) -> GeneratedStrings {
        let mut strings = Vec::with_capacity(n);
        let length_distribution = self.length_set(n);
        println!("\n\nGenerating strings...\n");
        let mut j: usize = 0; // used to update progress percentage
        for i in length_distribution {
            // todo: match only one time
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