#![warn(clippy::all)]

use measurements::get_resolution;
use random::{Exp, StringGen};

mod algorithms;
mod measurements;
mod random;

fn main() {
    // print the resolution of the clock
    println!("Clock resolution: {:?}", get_resolution());

    // Create new exponential distribution
    let rnd = Exp::new(1000, 500000, vec!['a', 'b']);

    let strings = rnd.create_random_strings(StringGen::Method1, 100);

    // todo: measure time (memory?)

    // todo: graph the results
}
