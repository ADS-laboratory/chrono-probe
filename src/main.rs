#![warn(clippy::all)]

use random::{Exp, StringGen, LengthDistribution};
use crate::algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART};
use crate::plot::time_plot;

mod algorithms;
mod measurements;
mod random;
mod plot;

fn main() {

    // Create new exponential distribution
    let rnd = Exp::new(1000, 500_000, vec!['a', 'b']);

    let strings = rnd.create_random_strings(StringGen::Method2, LengthDistribution::Exponential, 100);

    let algorithms = vec![PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];

    let results = measurements::measure(&strings, &algorithms, 0.001);

    let file_name = "plotters-doc-data/tick_control.svg";

    let result_clone = results.clone();
    for result in result_clone {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }

    time_plot(file_name, results);
}
