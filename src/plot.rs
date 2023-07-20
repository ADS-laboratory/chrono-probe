//! # Plot
//!
//! This module contains the functions for plotting the results of the measurements.
//! This is done using the [`time_plot`] function which this module provides.
//!
//! The [`time_plot`] function takes as inputs:
//! * A path to save the plot to
//! * a [`Measurements`] struct, which contains the results of the measurements
//! * a [`PlotConfig`] struct, which contains the configuration for the plot
//!
//! The [`PlotConfig`] struct can be created using the builder pattern, configurable option are:
//! * [`PlotConfig::with_x_label`]: Sets the x label for the plot.
//! * [`PlotConfig::with_y_label`]: Sets the y label for the plot.
//! * [`PlotConfig::with_title`]: Sets the title for the plot.
//! * [`PlotConfig::with_caption`]: Sets the caption for the plot.
//! * [`PlotConfig::with_scale`]: Sets the scale for the plot.

use std::fmt::{Debug, Formatter};
use std::time::Duration;

use plotters::prelude::*;

use crate::measurements::{Measurements, Point};

/// Configuration for plotting.
///
pub struct PlotConfig<'a> {
    title: &'a str,
    caption: &'a str,
    x_label: &'a str,
    y_label: &'a str,
    scale: Scale,
}

/// The scale of the plot.
pub enum Scale {
    /// Linear scale
    Linear,
    /// Double logarithmic scale
    LogLog,
}

impl<'a> PlotConfig<'a> {
    /// Crate a new [`PlotConfig`].
    ///
    /// Prefer using [`PlotConfig::default`] and then setting the desired values.
    pub fn new(
        title: &'a str,
        caption: &'a str,
        x_label: &'a str,
        y_label: &'a str,
        scale: Scale,
    ) -> PlotConfig<'a> {
        PlotConfig {
            title,
            caption,
            x_label,
            y_label,
            scale,
        }
    }

    /// Sets the x label for the plot.
    pub fn with_x_label(mut self, x_label: &'a str) -> PlotConfig<'a> {
        self.x_label = x_label;
        self
    }

    /// Sets the y label for the plot.
    pub fn with_y_label(mut self, y_label: &'a str) -> PlotConfig<'a> {
        self.y_label = y_label;
        self
    }

    /// Sets the title for the plot.
    pub fn with_title(mut self, title: &'a str) -> PlotConfig<'a> {
        self.title = title;
        self
    }

    /// Sets the caption for the plot.
    pub fn with_caption(mut self, caption: &'a str) -> PlotConfig<'a> {
        self.caption = caption;
        self
    }

    /// Sets the scale for the plot.
    pub fn with_scale(mut self, scale: Scale) -> PlotConfig<'a> {
        self.scale = scale;
        self
    }
}

impl<'a> Default for PlotConfig<'a> {
    fn default() -> PlotConfig<'a> {
        PlotConfig::new(
            "Measurements plot",
            "Caption",
            "Size",
            "Time",
            Scale::Linear,
        )
    }
}

enum Precision {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
}

impl Precision {
    const MAX_U32: u128 = u32::MAX as u128;

    fn get_precision_u32(duration: Duration) -> Self {
        if duration.as_nanos() < Self::MAX_U32 {
            Precision::Nanoseconds
        } else if duration.as_micros() < Self::MAX_U32 {
            Precision::Microseconds
        } else if duration.as_millis() < Self::MAX_U32 {
            Precision::Milliseconds
        } else {
            Precision::Seconds
        }
    }

    fn as_u32(&self, duration: Duration) -> u32 {
        match self {
            Precision::Nanoseconds => duration.as_nanos() as u32,
            Precision::Microseconds => duration.as_micros() as u32,
            Precision::Milliseconds => duration.as_millis() as u32,
            Precision::Seconds => duration.as_secs() as u32,
        }
    }
}

impl Debug for Precision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Precision::Nanoseconds => write!(f, "ns"),
            Precision::Microseconds => write!(f, "μs"),
            Precision::Milliseconds => write!(f, "ms"),
            Precision::Seconds => write!(f, "s"),
        }
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
pub fn time_plot(file_name: &str, measurements: Measurements, config: &PlotConfig) {
    let x_min = measurements.min_length() as u32;
    let x_max = measurements.max_length() as u32;

    let max_time = measurements.max_time();
    let y_precision = Precision::get_precision_u32(max_time);
    let y_min = y_precision.as_u32(measurements.min_time());
    let y_max = y_precision.as_u32(max_time);

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

    let caption = config.caption.to_string();

    let mut binding = ChartBuilder::on(&upper);

    let chart_builder = binding
        .caption(&caption, ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent());

    match config.scale {
        Scale::Linear => {
            let mut chart = chart_builder
                .build_cartesian_2d(x_min..x_max, y_min..y_max)
                .unwrap();
            chart
                .configure_mesh()
                .x_desc(config.x_label)
                .y_desc(format!("{} ({:?})", config.x_label, y_precision))
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
                            .map(|&Point { size, time, .. }| {
                                (size as u32, y_precision.as_u32(time))
                            }),
                        color.stroke_width(3),
                    ))
                    .unwrap()
                    .label(&measurement.algorithm_name)
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled())
                    });
            }

            chart
                .configure_series_labels()
                .border_style(BLACK)
                .draw()
                .unwrap();
        }
        Scale::LogLog => {
            let mut chart = chart_builder
                .build_cartesian_2d((x_min..x_max).log_scale(), (y_min..y_max).log_scale())
                .unwrap();
            chart
                .configure_mesh()
                .x_desc(config.x_label)
                .y_desc(format!("{} ({:?})", config.x_label, y_precision))
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
                            .map(|&Point { size, time, .. }| {
                                (size as u32, y_precision.as_u32(time))
                            }),
                        color.stroke_width(3),
                    ))
                    .unwrap()
                    .label(&measurement.algorithm_name)
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled())
                    });
            }

            chart
                .configure_series_labels()
                .border_style(BLACK)
                .draw()
                .unwrap();
        }
    };

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect(
        "Unable to write result to file, please make sure 'results' dir exists under current dir",
    );
    println!("Result has been saved to {file_name}");
}
