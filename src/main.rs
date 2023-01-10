#![warn(clippy::all)]

use random::{Exp, StringGen};
use crate::algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART};
use crate::plot::time_plot;

mod algorithms;
mod measurements;
mod random;
mod plot;

fn main() {

    // Create new exponential distribution
    let rnd = Exp::new(1000, 500000, vec!['a', 'b']);

    let strings = rnd.create_random_strings(StringGen::Method2, 100);

    let algorithms = vec![PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];

    let file_name = "plotters-doc-data/tick_control.svg";

    time_plot(file_name, strings, algorithms, 0.00001);
}
