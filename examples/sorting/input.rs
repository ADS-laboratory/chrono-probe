use core::ops::{Deref, DerefMut};

use rand::Rng;
use rand::thread_rng;

use chrono_probe::input::Input;

// Here we define a new Input type, which is a vector of u32.

// This type needs to implement the Clone trait because in order to get a precise measure
// for an input we repeatedly run the algorithm on the same input.

// Since sorting algorithms usually modify the input, we need to implement Clone for InputVec.
#[derive(Clone)]
pub struct InputVec(Vec<u32>);

// Here we implement Deref and DerefMut for InputVec, so that we can use it as a Vec<u32>.
// This is not necessary, but it makes it easier to use.

impl Deref for InputVec {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InputVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Here we implement the Input trait for InputVec.
impl Input for InputVec {
    // We don't need to choose between different input generators, so we don't need a Builder.
    type Builder = ();

    // Return the size of the input.
    fn get_size(&self) -> usize {
        self.len()
    }

    // Generate a random input of the given size.
    fn generate_input(size: usize, _builder: &Self::Builder) -> Self {
        let mut rng = thread_rng();
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            let rand: u32 = rng.gen();
            v.push(rand);
        }
        InputVec(v)
    }
}
