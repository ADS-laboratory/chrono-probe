use crate::distribution::Distribution;
use serde::Serialize;
use std::fs::File;

pub trait Input {
    type Builder: Clone;
    fn get_size(&self) -> usize;
    fn generate_input(size: usize, builder: Self::Builder) -> Self;
}

#[derive(Serialize)]
pub struct InputStruct<I: Input> {
    pub(crate) inputs: Vec<Vec<I>>,
}

#[derive(Serialize)]
pub struct InputBuilder<I: Input> {
    pub(crate) distribution: Distribution,
    pub(crate) builder: I::Builder,
}

impl<I: Input> InputBuilder<I> {
    pub fn new(distribution: Distribution, builder: I::Builder) -> InputBuilder<I> {
        InputBuilder {
            distribution,
            builder,
        }
    }

    pub fn generate_inputs(&self, n: usize) -> InputStruct<I> {
        self.generate_inputs_with_repetitions(n, 1)
    }

    pub fn generate_inputs_with_repetitions(&self, n: usize, repetitions: usize) -> InputStruct<I> {
        assert!(
            n > 0,
            "The number of strings to be generated must be greater than 0"
        );
        assert!(
            repetitions > 0,
            "The number of repetitions must be greater than 0"
        );
        let mut inputs = Vec::new();
        let length_distribution = self.distribution.create_length_set(n);
        #[cfg(feature = "debug")]
        println!("Generating inputs...\n");
        for (_j, input_size) in length_distribution.into_iter().enumerate() {
            let mut inputs_with_same_size = Vec::new();
            for _ in 0..repetitions {
                inputs_with_same_size.push(I::generate_input(input_size, self.builder.clone()));
            }
            inputs.push(inputs_with_same_size);
            #[cfg(feature = "debug")]
            {
                if _j % (n / 20) == 0 {
                    println!("{}%", _j * 100 / n);
                }
            }
        }
        InputStruct { inputs }
    }
}

impl<I: Input + Serialize> InputStruct<I> {
    pub fn serialize_json(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        serde_json::to_writer(&mut file, &self).unwrap();
    }
}
