//! # Sorting algorithms
//!
//! This example shows how to use the library to measure the time complexity of sorting algorithms.

use time_complexity_plot::{
    input::{distribution::Exponential, InputBuilder},
    measurements::measure_mut,
    plot::time_plot,
};
use time_complexity_plot::plot::PlotConfig;

use crate::algorithms::{merge_sort_input, quick_sort_input};

mod algorithms;
mod input;

fn main() {
    // Create a distribution for the length of the vectors
    // Here we use an exponential distribution with a minimum of 1000 and a maximum of 500_000
    let length_distribution = Exponential::new(1000..=500_000);

    // Create the builder for the vectors
    let vector_builder = InputBuilder::new(length_distribution, ());

    // Build the vectors
    // Here we build 2000 vectors, 10 of each length
    let mut vectors = vector_builder.build_with_repetitions(200, 10);

    // Create a slice of the algorithms we want to measure
    let algorithms: &[(fn(&mut input::InputVec), &str); 2] = &[
        (merge_sort_input, "Merge sort"),
        (quick_sort_input, "Quick sort"),
    ];

    // Measure the algorithms on the vectors, given a relative error of 0.001
    let results = measure_mut(&mut vectors, algorithms, 0.001);

    let result_clone = results.clone();
    // Serialize the results to a json file
    result_clone.serialize_json("results.json");

    let file_name = "plotters-doc-data/tick_control.svg";

    // Plot the results
    let config = PlotConfig::default()
        .with_title("Sorting algorithms")
        .with_caption("The time plot of sorting algorithms");

    time_plot(file_name, results, &config);
}
