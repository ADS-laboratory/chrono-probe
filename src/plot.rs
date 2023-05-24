use plotters::prelude::*;

use crate::input::{Input, InputBuilder};
use crate::input::distribution::Distribution;
use crate::measurements::{Measurements, Point};

/// Configuration for plotting.
///
pub struct PlotConfig<'a, I: Input, D: Distribution> {
    title: &'a str,
    caption: &'a str,
    builder: Option<&'a InputBuilder<I, D>>,
    x_label: &'a str,
    y_label: &'a str,
}

impl<'a, I: Input, D: Distribution> PlotConfig<'a, I, D> {
    /// Crate a new [`PlotConfig`].
    ///
    /// Prefer using [`PlotConfig::default`] and then setting the desired values.
    pub fn new(
        title: &'a str,
        caption: &'a str,
        builder: Option<&'a InputBuilder<I, D>>,
        x_label: &'a str,
        y_label: &'a str,
    ) -> PlotConfig<'a, I, D> {
        PlotConfig {
            title,
            caption,
            builder,
            x_label,
            y_label,
        }
    }

    /// Sets the builder to be used for the plot.
    pub fn with_builder(mut self, builder: &'a InputBuilder<I, D>) -> PlotConfig<'a, I, D> {
        self.builder = Some(builder);
        self
    }

    /// Sets the x label for the plot.
    pub fn with_x_label(mut self, x_label: &'a str) -> PlotConfig<'a, I, D> {
        self.x_label = x_label;
        self
    }

    /// Sets the y label for the plot.
    pub fn with_y_label(mut self, y_label: &'a str) -> PlotConfig<'a, I, D> {
        self.y_label = y_label;
        self
    }

    /// Sets the title for the plot.
    pub fn with_title(mut self, title: &'a str) -> PlotConfig<'a, I, D> {
        self.title = title;
        self
    }

    /// Sets the caption for the plot.
    pub fn with_caption(mut self, caption: &'a str) -> PlotConfig<'a, I, D> {
        self.caption = caption;
        self
    }
}

impl<'a, I: Input, D: Distribution> Default for PlotConfig<'a, I, D> {
    fn default() -> PlotConfig<'a, I, D> {
        PlotConfig::new("Measurements plot", "Caption", None, "Size", "Time")
    }
}

/// Plots the data from the [`Measurements`] using [plotters].
/// The plot is saved to the file specified by `file_name`, the file created will be an SVG file.
///
/// # Arguments
///
/// * `file_name` - The name of the file to save the plot to
/// * `measurements` - The measurements to plot
/// * `builder` - The builder that was used to generate the measurements
///
pub fn time_plot<I: Input, D: Distribution>(
    file_name: &str,
    measurements: Measurements,
    config: &PlotConfig<I, D>,
) {
    let x_min = measurements.min_length() as u32;
    let x_max = measurements.max_length() as u32;
    let y_min = measurements.min_time().as_micros() as u32;
    let y_max = measurements.max_time().as_micros() as u32;

    let mut measurements = measurements.measurements;

    // plot setup
    let root = SVGBackend::new(file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (upper, lower) = root.split_vertically(750);

    lower
        .titled(
            config.title,
            ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
        )
        .unwrap();

    let caption = match config.builder {
        None => config.caption.to_string(),
        Some(b) => format!("{}\n{:?}", config.caption, b.distribution),
    };

    let mut chart = ChartBuilder::on(&upper)
        .caption(&caption, ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(config.x_label)
        .y_desc(config.y_label)
        .draw()
        .unwrap();

    // draw data for each algorithm
    for (i, measurement) in measurements.iter_mut().enumerate() {
        measurement.measurement.sort_by_key(|a| a.size);

        let color = Palette99::pick(i).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                measurement
                    .measurement
                    .iter()
                    .map(|&Point { size, time, .. }| (size as u32, time.as_micros() as u32)),
                color.stroke_width(3),
            ))
            .unwrap()
            .label(&measurement.algorithm_name)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {file_name}");
}
