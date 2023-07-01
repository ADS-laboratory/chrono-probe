//! # Search in an ordered vector
//!
//! This example shows how to use the library to measure the time complexity of
//! searching algorithms in an ordered vector.

use time_complexity_plot::{
    input::{distribution::Uniform, InputBuilder},
    measurements::measure,
    plot::time_plot,
};
use time_complexity_plot::plot::PlotConfig;

use crate::algorithms::{binary_search_input, linear_search_input};
use crate::input::Generator;

mod algorithms;
mod input;

fn main() {
    // Create a distribution for the length of the vectors
    // Here we use an uniform distribution with a minimum of 10 and a maximum of 100_000
    let length_distribution = Uniform::new(10..=100_000);

    // Create the builder for the vectors
    // Here we choose to use the fast generator method in order to generate ordered vectors
    let vector_builder = InputBuilder::new(length_distribution, Generator::Fast);

    // Build 200 vectors
    let vectors = vector_builder.build(200);

    // Create a slice of the algorithms we want to measure
    let algorithms: &[(fn(&input::SearchInput) -> Option<usize>, &str); 2] = &[
        (linear_search_input, "Linear search"),
        (binary_search_input, "Binary search"),
    ];

    // Measure the algorithms on the vectors, given a relative error of 0.001
    let results = measure(&vectors, algorithms, 0.001);

    let file_name = "results/tick_control.svg";

    // Here we print the linear regression of the log-log scale of the results
    for result in results.clone().measurements {
        let log_linear_regression = result.log_log_scale().linear_regression();
        println!(
            "{}: {} * x + {}",
            result.algorithm_name, log_linear_regression.0, log_linear_regression.1
        )
    }

    let config = PlotConfig::default()
        .with_title("Search in an ordered vector")
        .with_caption("The time plot of searching algorithms in an ordered vector");

    // Plot the results
    time_plot(file_name, results, &config);
}
