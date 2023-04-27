use std::ops::Deref;
use rand::Rng;
use time_complexity_plot::input::Input;


pub struct InputSortedVec(Vec<i8>);

impl Deref for InputSortedVec {
    type Target = Vec<i8>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

pub type Vector = InputSortedVec;

impl Input for Vector {
    type Builder = ();

    fn get_size(&self) -> usize {
        self.len()
    }

    fn generate_input(size: usize, _builder: Self::Builder) -> Self {
        generate_n_ordered_random_vector(size, 0, i8::MAX)
    }
}

fn generate_n_ordered_random_vector(n: usize, min: i8, max: i8) -> Vector {
    let mut rng = rand::thread_rng();
    let bucket_size = (max - min) / n as i8;
    let mut vec = Vec::with_capacity(n);
    let mut count = 0;

    'outer: for i in 0..n {
        let bucket_min = min + i as i8 * bucket_size;
        let bucket_max = bucket_min + bucket_size;
        let num_numbers = rng.gen_range(1..4);
        let mut bucket_vec = Vec::with_capacity(num_numbers);
        for _ in 0..num_numbers {
            if count == n {
                break 'outer;
            }
            let num = rng.gen_range(bucket_min..bucket_max);
            bucket_vec.push(num);
            count += 1;
        }
        bucket_vec.sort();
        vec.extend(bucket_vec);
    }
    InputSortedVec(vec)
}