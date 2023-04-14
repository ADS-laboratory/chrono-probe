extern crate core;

mod algorithms;
mod input;

use crate::algorithms::{merge_sort_input, quick_sort_input};

use time_complexity_plot::{
    input::{distribution::DistributionBuilder, impls::distribution::EXPONENTIAL, InputBuilder},
    measurements::measure_mut,
    plot::time_plot,
};

fn main() {
    let length_distribution = DistributionBuilder::new(EXPONENTIAL, 1000, 500_000);

    let vector_builder = InputBuilder::new(length_distribution, ());

    let mut vectors = vector_builder.build_with_repetitions(200, 10);

    let algorithms = &[merge_sort_input, quick_sort_input];

    let results = measure_mut(&mut vectors, algorithms, 0.00001);

    let file_name = "plotters-doc-data/tick_control.svg";

    let result_clone = results.clone();
    result_clone.serialize_json("results.json");

    /*
    for result in result_clone.measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }
     */

    time_plot(file_name, results, vector_builder);
}
