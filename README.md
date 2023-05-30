# TODOs
- [ ] Algorithms name: find a way to display algorithms name in the plot
- [ ] Statistic analysis: variance / standard deviation, etc.
- [ ] Documentation:
    - [ ] Crate and readme
    - [x] Input / distribution module
    - [x] Measurments module
    - [ ] Plot module




# Time complexity plot
A rust library to plot and estimate the time complexity of algorithms.

## Configuration
### Install Rust
If you don't have Rust installed, you can do so by running the following command:
``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Update your Cargo.toml
Add this to your Cargo.toml:
``` toml
[dependencies]
fractional_period = "git@github.com:ADS-laboratory/time-complexity-plot.git"
```

## Usage
### Example
// todo: update example with the new structure
``` rust
use time_complexity_plot::{random::{Distribution, METHOD1, EXPONENTIAL},
                            algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART},
                            plot::time_plot};
fn main() {

    // Create new exponential distribution
    let rnd = Distribution::new(EXPONENTIAL, 1000, 500_000);

    let strings = rnd.create_random_strings(METHOD1, vec!['a', 'b'], 100);

    let algorithms = vec![PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];

    let results = measurements::measure(&strings, &algorithms, 0.001);

    let file_name = "plotters-doc-data/tick_control.svg";

    let result_clone = results.clone();
    for result in result_clone.measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }

    time_plot(file_name, results);
}
```