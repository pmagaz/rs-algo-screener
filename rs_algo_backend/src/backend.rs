use crate::error::Result;
use crate::models::*;
use plotters::prelude::*;
use std::env;

#[derive(Debug, Clone)]
pub struct Backend;

impl Backend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, instrument: &Instrument) -> Result<()> {
        let to_date = instrument.data.last().unwrap().date;
        let from_date = instrument.data.first().unwrap().date;

        let font = env::var("PLOTTER_FONT").unwrap();

        let peaks_marker_distance = env::var("MARKERS_DISTANCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let output_file = [
            &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
            &instrument.symbol,
            ".png",
        ]
        .concat();
        println!("BACKEND PATH {}", output_file);
        let min_price = instrument.min_price;
        let max_price = instrument.max_price;

        let data = instrument.data.clone();
        let local_maxima = &instrument.peaks.local_maxima;
        let local_minima = &instrument.peaks.local_minima;
        let extrema_maxima = &instrument.peaks.extrema_maxima;
        let extrema_minima = &instrument.peaks.extrema_minima;
        // let horizontal_levels = instrument.horizontal_levels.horizontal_levels;

        let local_patterns = &instrument.patterns.local_patterns;
        let local_pattern_breaks: Vec<usize> = instrument
            .patterns
            .local_patterns
            .iter()
            .map(|x| x.active.index)
            .collect();

        let extrema_patterns = &instrument.patterns.extrema_patterns;
        let extrema_pattern_breaks: Vec<usize> = instrument
            .patterns
            .extrema_patterns
            .iter()
            .map(|x| x.active.index)
            .collect();
        let stoch = &instrument.indicators.stoch;
        let stoch_a = &stoch.data_a;
        let stoch_b = &stoch.data_b;

        let macd = &instrument.indicators.macd;
        let macd_a = &macd.data_a;
        let macd_b = &macd.data_b;

        let rsi = &instrument.indicators.rsi.data_a;

        let ema_a = &instrument.indicators.ema_a.data_a;
        let ema_b = &instrument.indicators.ema_b.data_a;
        let ema_c = &instrument.indicators.ema_c.data_a;

        let root = BitMapBackend::new(&output_file, (1024, 768)).into_drawing_area();
        let (upper, lower) = root.split_vertically((75).percent());
        let (indicator_1, indicator_2) = lower.split_vertically((50).percent());

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&upper)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(5)
            .caption(&instrument.symbol, (font.as_ref(), 14.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_price..max_price)
            .unwrap();

        chart
            .configure_mesh()
            .light_line_style(&WHITE)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()
            .unwrap();
        chart
            .draw_series(data.iter().map(|candle| {
                let candle_color: (ShapeStyle, ShapeStyle) = match candle {
                    _x if candle.close >= candle.open => (GREEN.filled(), GREEN.filled()),
                    _x if candle.close <= candle.open => (RED.filled(), RED.filled()),
                    _ => (GREEN.filled(), GREEN.filled()),
                };

                let (bullish, bearish) = match candle.candle_type {
                    CandleType::Engulfing => candle_color,
                    _ => candle_color,
                };

                CandleStick::new(
                    candle.date,
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    bullish,
                    bearish,
                    2,
                )
            }))
            .unwrap();

        // for (x, pattern) in extrema_patterns.iter().enumerate() {
        //     chart
        //         .draw_series(PointSeries::of_element(
        //             (0..).zip(pattern.data_points.iter()).map(|(i, highs)| {
        //                 let idx = highs.0;
        //                 let value = highs.1;
        //                 let date = data[idx].date;
        //                 (date, value, i)
        //             }),
        //             0,
        //             ShapeStyle::from(&RED).filled(),
        //             &|coord, _size: i32, _style| {
        //                 let new_coord = (coord.0, coord.1);
        //                 let mut pattern_name;
        //                 if coord.2 == 4 {
        //                     pattern_name = Text::new(
        //                         format!("{:?}", pattern.pattern_type),
        //                         (0, 0),
        //                         (font.as_ref(), 15),
        //                     )
        //                 } else {
        //                     pattern_name = Text::new(format!("{:?}", ""), (0, 15), (font.as_ref(), 0))
        //                 }

        //                 EmptyElement::at(new_coord) + pattern_name
        //             },
        //         ))
        //         .unwrap();
        // }

        // for (x, pattern) in extrema_patterns.iter().enumerate() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..).zip(pattern.data_points.iter()).map(|(_k, highs)| {
        //                 let idx = highs.0;
        //                 let value = highs.1;
        //                 let date = data[idx].date;
        //                 (date, value)
        //             }),
        //             (if x < 1 { &BLACK } else { &BLACK }),
        //         ))
        //         .unwrap()
        //         .label(format!("{:?}", pattern.pattern_type));
        // }

        for (x, pattern) in local_patterns.iter().enumerate() {
            chart
                .draw_series(PointSeries::of_element(
                    (0..).zip(pattern.data_points.iter()).map(|(i, highs)| {
                        let idx = highs.0;
                        let value = highs.1;
                        let date = data[idx].date;
                        (date, value, i)
                    }),
                    0,
                    ShapeStyle::from(&RED).filled(),
                    &|coord, _size: i32, _style| {
                        let new_coord = (coord.0, coord.1);
                        let mut pattern_name;
                        if coord.2 == 4 {
                            pattern_name = Text::new(
                                format!("{:?}", pattern.pattern_type),
                                (0, 0),
                                (font.as_ref(), 15),
                            )
                        } else {
                            pattern_name =
                                Text::new(format!("{:?}", ""), (0, 15), (font.as_ref(), 0))
                        }

                        EmptyElement::at(new_coord) + pattern_name
                    },
                ))
                .unwrap();
        }

        //  for (x, pattern) in local_patterns.iter().enumerate() {
        //     if pattern.active.id
        //     // if local_minima.contains(&(i, candle.close)) {
        //     //     return TriangleMarker::new(
        //     //         (
        //     //             candle.date,
        //     //             candle.high + candle.high / peaks_marker_distance - 10.,
        //     //         ),
        //     //         4,
        //     //         BLUE.filled(),
        //     //     );
        //     // } else {
        //     //     return TriangleMarker::new((candle.date, candle.high), 0, &TRANSPARENT);
        //     // }
        //     }
        for (x, pattern) in local_patterns.iter().enumerate() {
            chart
                .draw_series(LineSeries::new(
                    (0..).zip(pattern.data_points.iter()).map(|(_k, highs)| {
                        let idx = highs.0;
                        let value = highs.1;
                        let date = data[idx].date;
                        (date, value)
                    }),
                    if x < 1 { &BLACK } else { &BLACK },
                ))
                .unwrap()
                .label(format!("{:?}", pattern.pattern_type));
        }

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_pattern_breaks.contains(&(i)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.close / peaks_marker_distance + 5.,
                        ),
                        -4,
                        BLACK.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.close), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // LOCAL MAXIMA MINIMA

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_maxima.contains(&(i, candle.close)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.close / peaks_marker_distance + 5.,
                        ),
                        -4,
                        BLUE.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.close), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_minima.contains(&(i, candle.close)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.high / peaks_marker_distance - 10.,
                        ),
                        4,
                        BLUE.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.high), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // EXTREMA MAXIMA MINIMA

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_maxima.contains(&(i, candle.close)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.close / peaks_marker_distance + 6.,
                        ),
                        -4,
                        RED.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.close), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_minima.contains(&(i, candle.close)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.high / peaks_marker_distance - 11.,
                        ),
                        4,
                        RED.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.high), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_pattern_breaks.contains(&(i)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.close / peaks_marker_distance + 5.,
                        ),
                        -4,
                        BLACK.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.close), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // for x in instrument.peaks.smooth_highs().iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..)
        //                 .zip(instrument.peaks.smooth_highs().iter())
        //                 .map(|(_k, highs)| {
        //                     let idx = highs.0;
        //                     let value = highs.1;
        //                     let date = data[idx].date;
        //                     (date, value)
        //                 }),
        //             &YELLOW,
        //         ))
        //         .unwrap();
        // }

        // for x in instrument.peaks.smooth_lows().iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..)
        //                 .zip(instrument.peaks.smooth_lows().iter())
        //                 .map(|(_k, highs)| {
        //                     let idx = highs.0;
        //                     let value = highs.1;
        //                     let date = data[idx].date;
        //                     (date, value)
        //                 }),
        //             &YELLOW,
        //         ))
        //         .unwrap();
        // }

        for x in instrument.peaks.smooth_close.iter() {
            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(instrument.peaks.smooth_close.iter())
                        .map(|(_k, highs)| {
                            let idx = highs.0;
                            let value = highs.1;
                            let date = data[idx].date;
                            (date, value)
                        }),
                    &TRANSPARENT,
                ))
                .unwrap();
        }

        // for x in local_maxima.iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..).zip(local_maxima.iter()).map(|(_k, highs)| {
        //                 let idx = highs.0;
        //                 let value = highs.1;
        //                 let date = data[idx].date;
        //                 (date, value)
        //             }),
        //             &BLUE,
        //         ))
        //         .unwrap();
        // }
        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(upper_channel.lower_band()[2].iter()).map(|(_k, highs)| {
        //       let idx = highs.0;
        //       let value = highs.1;
        //       let date = data[idx].date;
        //       (date, value)
        //     }),
        //     &RED,
        //   ))
        //   .unwrap();

        // chart
        //   .draw_series(LineSeries::new(
        //     (0..)
        //       .zip(upper_channel.lower_band()[0].iter())
        //       .map(|(key, value)| {
        //         let date = data[value.0].date;
        //         (date, value.1)
        //       }),
        //     &RED,
        //   ))
        //   .unwrap();
        // // }

        // chart
        //   .draw_series(LineSeries::new(
        //     (0..)
        //       .zip(upper_channel.lower_band().iter())
        //       .map(|(key, value)| {
        //         let date = data[value.0].date;
        //         //println!("222222, {:?}, {:?}", date, value);
        //         (date, value.1)
        //       }),
        //     &RED,
        //   ))
        //   .unwrap();

        // HORIZONTAL LEVELS
        /*
        for x in horizontal_levels.iter() {
            let color = match x.1.level_type() {
                HorizontalLevelType::Support => BLUE.filled(),
                HorizontalLevelType::Resistance => RED.filled(),
                _ => TRANSPARENT.filled(),
            };
            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(data.iter())
                        .map(|(_id, candle)| (candle.date, *x.1.price())),
                    color,
                ))
                .unwrap();
        }
        */
        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(_id, candle)| {
        //       let date = candle.date;
        //       //let value = ema_a.next(candle.close);
        //       (date, value)
        //     }),
        //     &BLUE,
        //   ))
        //   .unwrap();

        //         chart
        //   .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(data.iter())
        //             .map(|(id, candle)| (candle.date, ema_a[id])),
        //         &RED,
        //     ))
        //     .unwrap();

        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(_id, candle)| {
        //       let date = candle.date;
        //       let value = ema_c.next(candle.close);
        //       (date, value)
        //     }),
        //     &RED,
        //   ))
        //   .unwrap();

        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(_id, candle)| {
        //       let date = candle.date;
        //       let value = ema_a.next(candle.close);
        //       (date, value)
        //     }),
        //     &YELLOW,
        //   ))
        //   .unwrap();

        // PEAKS

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_maxima.contains(&(i, candle.high)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.high + candle.high / peaks_marker_distance + 5.,
                        ),
                        -4,
                        BLUE.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.high), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_minima.contains(&(i, candle.low)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.low - candle.low / peaks_marker_distance - 5.,
                        ),
                        4,
                        BLUE.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.low), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_maxima.contains(&(i, candle.high)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.low - candle.low / peaks_marker_distance + 25.,
                        ),
                        -4,
                        RED.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.low), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_minima.contains(&(i, candle.low)) {
                    return TriangleMarker::new(
                        (
                            candle.date,
                            candle.low - candle.low / peaks_marker_distance - 25.,
                        ),
                        4,
                        RED.filled(),
                    );
                } else {
                    return TriangleMarker::new((candle.date, candle.low), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // let mut rsi_pannel = ChartBuilder::on(&lower_1)
        //     .x_label_area_size(40)
        //     .y_label_area_size(40)
        //     //.margin(2)
        //     //.caption("RSI", (font.as_ref(), 8.0).into_font())
        //     .build_cartesian_2d(from_date..to_date, -0f64..100f64)
        //     .unwrap();

        // INDICATORS

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, ema_a[id])),
                &RED,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, ema_b[id])),
                &BLUE,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, ema_c[id])),
                &YELLOW,
            ))
            .unwrap();

        // rsi_pannel
        //     .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(data.iter())
        //             .map(|(id, candle)| (candle.date, rsi[id])),
        //         &RED,
        //     ))
        //     .unwrap();

        let mut stoch_pannel = ChartBuilder::on(&indicator_1)
            .x_label_area_size(40)
            .y_label_area_size(40)
            // .margin(2)
            //.caption("MACD", (font.as_ref(), 8.0).into_font())
            .build_cartesian_2d(from_date..to_date, -0f64..100f64)
            .unwrap();
        //stoch_pannel.configure_mesh().light_line_style(&WHITE).draw().unwrap();
        stoch_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, stoch_a[id])),
                &BLUE,
            ))
            .unwrap();

        stoch_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, stoch_b[id])),
                &RED,
            ))
            .unwrap();

        let min = macd_a.iter().map(|x| *x as usize).min().unwrap() as f64;
        let max = macd_a.iter().map(|x| *x as usize).max().unwrap() as f64;
        let mut macd_pannel = ChartBuilder::on(&indicator_2)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(from_date..to_date, min..max)
            .unwrap();

        macd_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, macd_a[id])),
                &BLUE,
            ))
            .unwrap();

        macd_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, macd_b[id])),
                &RED,
            ))
            .unwrap();

        root.present().expect("[BACKEND] Error. Can't save file!");
        println!("[BACKEND] File saved in {}", output_file);
        Ok(())
    }
}
