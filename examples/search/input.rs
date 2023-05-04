use rand::Rng;
use time_complexity_plot::input::Input;

pub struct SearchInput {
    pub vector: Vec<u32>,
    pub target: u32,
}

impl Input for SearchInput {
    type Builder = Generator;

    fn get_size(&self) -> usize {
        self.vector.len()
    }

    fn generate_input(size: usize, builder: Self::Builder) -> Self {
        match builder {
            Generator::Fast => generate_order_vector_fast(size, u32::MIN, u32::MAX),
            Generator::Uniform => generate_order_vector(size, u32::MIN, u32::MAX),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Generator {
    Fast,
    Uniform,
}

fn generate_order_vector_fast(n: usize, min: u32, max: u32) -> SearchInput {
    let mut rng = rand::thread_rng();
    let bucket_size = (max - min) / n as u32;
    let mut vec = Vec::with_capacity(n);

    for i in 0..n {
        let bucket_min = min + i as u32 * bucket_size;
        let bucket_max = if i == n - 1 {
            max
        } else {
            bucket_min + bucket_size
        };
        let num = rng.gen_range(bucket_min..bucket_max);
        vec.push(num);
    }
    SearchInput {
        vector: vec,
        target: rng.gen_range(min..max),
    }
}

fn generate_order_vector(n: usize, min: u32, max: u32) -> SearchInput {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(n);

    for _ in 0..n {
        let num = rng.gen_range(min..max);
        vec.push(num);
    }
    vec.sort();
    SearchInput {
        vector: vec,
        target: rng.gen_range(min..max),
    }
}
