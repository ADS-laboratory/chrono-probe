use core::ops::{Deref, DerefMut};
use rand::thread_rng;
use rand::Rng;
use time_complexity_plot::input::Input;

#[derive(Clone)]
pub struct InputVec(Vec<u32>);
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
pub type Vector = InputVec;

impl Input for Vector {
    type Builder = ();

    fn get_size(&self) -> usize {
        self.len()
    }

    fn generate_input(size: usize, _builder: Self::Builder) -> Self {
        let mut rng = thread_rng();
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            let rand: u32 = rng.gen();
            v.push(rand);
        }
        InputVec(v)
    }
}
