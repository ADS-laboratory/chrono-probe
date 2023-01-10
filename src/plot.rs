use std::time::Duration;
use plotters::prelude::*;
use crate::measurements::get_times;
use crate::algorithms::{Algorithm};


struct Point {
    length_of_string: u32,
    time: u32,
}

fn generate_data(times: Vec<Duration>, strings: &[String]) -> Vec<Point> {
    let mut data = Vec::with_capacity(times.len());
    for i in 0..times.len() {
        let point = Point {
            length_of_string: strings[i].len() as u32,
            time: times[i].as_nanos() as u32,
        };
        data.push(point);
    }
    data
}

pub fn time_plot(file_name: &str, strings: Vec<String>, algorithms: Vec<Algorithm>, relative_error: f32) {

    // plot setup
    let root = SVGBackend::new(file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (upper, lower) = root.split_vertically(750);

    lower.titled(
        "Data Source: https://covid.ourworldindata.org/data/owid-covid-data.json",
        ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
    ).unwrap();

    let mut chart = ChartBuilder::on(&upper)
        .caption("fractional period test", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (1000u32..500_000u32).log_scale(),
            (1500u32..10_000_000u32).log_scale(),
        ).unwrap();

    chart
        .configure_mesh()
        .x_desc("size of string")
        .y_desc("Time")
        .draw().unwrap();

    // draw data for each algorithm
    for (i, algorithm) in algorithms.iter().enumerate() {
        println!("\n\nProcessing {} ({}/{})...\n", algorithm.name, i+1, algorithms.len());
        let durations = get_times(algorithm, &strings, relative_error);
        let mut data = generate_data(durations, &strings);
        data.sort_by_key(|k| k.length_of_string);

        let color = Palette99::pick(i).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                data.iter().map(
                    |&Point {
                        length_of_string,
                        time,
                        ..
                    }| (length_of_string, time),
                ),
                color.stroke_width(3),
            )).unwrap()
            .label(algorithm.name)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw().unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", file_name);
}