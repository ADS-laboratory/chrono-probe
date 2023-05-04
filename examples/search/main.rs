mod algorithms;
mod input;

use crate::algorithms::{binary_search_input, linear_search_input};
use crate::input::Generator;

use time_complexity_plot::{
    input::{distribution::Uniform, InputBuilder},
    measurements::measure,
    plot::time_plot,
};

fn main() {
    let length_distribution = Uniform::new(10..=100_000);

    let vector_builder = InputBuilder::new(length_distribution, Generator::Fast);

    let vectors = vector_builder.build(200);

    let algorithms = &[linear_search_input, binary_search_input];

    let results = measure(&vectors, algorithms, 0.001);

    let file_name = "plotters-doc-data/tick_control.svg";

    for result in results.clone().measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!(
            "{}: {} * x + {}",
            result.algorithm_name, log_linear_regression.0, log_linear_regression.1
        )
    }

    time_plot(file_name, results, &vector_builder);
}
