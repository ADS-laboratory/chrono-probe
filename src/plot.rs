use plotters::prelude::*;
use crate::measurements::{Measurements, Point};

pub fn time_plot(file_name: &str, measurements_struct: Measurements) {

    let mut measurements = measurements_struct.measurements;

    println!("\nPlotting...\n");

    // plot setup
    let root = SVGBackend::new(file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (upper, lower) = root.split_vertically(750);

    lower.titled(
        "Data Source: https://covid.ourworldindata.org/data/owid-covid-data.json",
        ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
    ).unwrap();

    let x_min = measurements.iter().map(|m| m.min_length()).min().unwrap() as u32;
    let x_max = measurements.iter().map(|m| m.max_length()).max().unwrap() as u32;
    let y_min = measurements.iter().map(|m| m.min_time()).min().unwrap().as_micros() as u32;
    let y_max = measurements.iter().map(|m| m.max_time()).max().unwrap().as_micros() as u32;


    let mut chart = ChartBuilder::on(&upper)
        .caption("fractional period test", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (x_min..x_max).log_scale(),
            (y_min..y_max).log_scale(),
        ).unwrap();

    chart
        .configure_mesh()
        .x_desc("size of string")
        .y_desc("Time")
        .draw().unwrap();

    // draw data for each algorithm
    for (i, measurement) in measurements.iter_mut().enumerate() {

        measurement.measurement.sort_by_key(|a| a.length_of_string);

        let color = Palette99::pick(i).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                measurement.measurement.iter().map(
                    |&Point {
                        length_of_string,
                        time,
                        ..
                    }| (length_of_string as u32, time.as_micros() as u32),
                ),
                color.stroke_width(3),
            )).unwrap()
            .label(measurement.algorithm_name)
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