use crate::candle::CandleType;
use crate::error::Result;
use crate::indicators::Indicator;
use crate::instrument::Instrument;

use plotters::prelude::*;

use std::env;

#[derive(Debug, Clone)]
pub struct Backend;

impl Backend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, instrument: &Instrument) -> Result<()> {
        let to_date = instrument.data().last().unwrap().date();
        let from_date = instrument.data().first().unwrap().date();
        let local_peaks_marker_pos = env::var("LOCAL_PEAKS_MARKERS_POS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let extrema_peaks_marker_pos = env::var("EXTREMA_PEAKS_MARKERS_POS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let output_file = [
            &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
            instrument.symbol(),
            ".png",
        ]
        .concat();
        let min_price = instrument.min_price();
        let max_price = instrument.max_price();

        let data = instrument.data();
        let local_maxima = instrument.peaks().local_maxima();
        let local_minima = instrument.peaks().local_minima();
        let extrema_maxima = instrument.peaks().extrema_maxima();
        let extrema_minima = instrument.peaks().extrema_minima();
        let horizontal_levels = instrument.horizontal_levels();

        let local_patterns = instrument.patterns().local_patterns.clone();
        let local_pattern_breaks: Vec<usize> = instrument
            .patterns()
            .local_patterns
            .iter()
            .map(|x| x.active.index)
            .collect();

        let extrema_patterns = instrument.patterns().extrema_patterns.clone();
        let extrema_pattern_breaks: Vec<usize> = instrument
            .patterns()
            .extrema_patterns
            .iter()
            .map(|x| x.active.index)
            .collect();

        let BACKGROUND = &RGBColor(192, 200, 212);
        let CANDLE_BULLISH = &RGBColor(71, 113, 181);
        let CANDLE_BEARISH = &RGBColor(255, 255, 255);
        let RED_LINE = &RGBColor(222, 110, 152);
        let BLUE_LINE = &RGBColor(71, 113, 181);

        let stoch = instrument.indicators().stoch();
        let stoch_a = stoch.get_data_a();
        let stoch_b = stoch.get_data_b();

        let macd = instrument.indicators().macd();
        let macd_a = macd.get_data_a();
        let macd_b = macd.get_data_b();

        let rsi = instrument.indicators().rsi().get_data_a();

        let tema_a = instrument.indicators().tema_a.get_data_a();
        let ema_b = instrument.indicators().ema_b().get_data_a();
        let tema_c = instrument.indicators().tema_c.get_data_a();

        let root = BitMapBackend::new(&output_file, (1024, 768)).into_drawing_area();
        let (upper, lower) = root.split_vertically((80).percent());
        let (indicator_1, indicator_2) = lower.split_vertically((50).percent());

        root.fill(BACKGROUND).unwrap();

        let mut chart = ChartBuilder::on(&upper)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(5)
            .caption(instrument.symbol(), ("sans-serif", 14.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_price..max_price)
            .unwrap();

        chart
            .configure_mesh()
            .light_line_style(BACKGROUND)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()
            .unwrap();
        chart
            .draw_series(data.iter().map(|candle| {
                let candle_color: (ShapeStyle, ShapeStyle) = match candle {
                    _x if candle.close() >= candle.open() => {
                        (CANDLE_BULLISH.filled(), CANDLE_BULLISH.filled())
                    }
                    _x if candle.close() <= candle.open() => {
                        (CANDLE_BEARISH.filled(), CANDLE_BEARISH.filled())
                    }
                    _ => (CANDLE_BULLISH.filled(), CANDLE_BULLISH.filled()),
                };

                let (bullish, bearish) = match candle.candle_type() {
                    CandleType::Engulfing => (RED_LINE.filled(), RED_LINE.filled()),
                    _ => candle_color,
                };

                CandleStick::new(
                    candle.date(),
                    candle.open(),
                    candle.high(),
                    candle.low(),
                    candle.close(),
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
        //                 let date = data[idx].date();
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
        //                         ("sans-serif", 15),
        //                     )
        //                 } else {
        //                     pattern_name = Text::new(format!("{:?}", ""), (0, 15), ("sans-serif", 0))
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
        //                 let date = data[idx].date();
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
                        let date = data[idx].date();
                        (date, value, i)
                    }),
                    0,
                    ShapeStyle::from(&RED).filled(),
                    &|coord, _size: i32, _style| {
                        let new_coord = (coord.0, coord.1);
                        let pattern_name;
                        if coord.2 == 0 {
                            pattern_name = Text::new(
                                format!("{:?}", pattern.pattern_type),
                                (0, 0),
                                ("sans-serif", 12),
                            )
                        } else {
                            pattern_name =
                                Text::new(format!("{:?}", ""), (0, 12), ("sans-serif", 0))
                        }

                        EmptyElement::at(new_coord) + pattern_name
                    },
                ))
                .unwrap();
        }

        //  for (x, pattern) in local_patterns.iter().enumerate() {
        //     if pattern.active.id
        //     // if local_minima.contains(&(i, candle.close())) {
        //     //     return TriangleMarker::new(
        //     //         (
        //     //             candle.date(),
        //     //             candle.high() + candle.high() / local_peaks_marker_pos - 10.,
        //     //         ),
        //     //         4,
        //     //         BLUE.filled(),
        //     //     );
        //     // } else {
        //     //     return TriangleMarker::new((candle.date(), candle.high()), 0, &TRANSPARENT);
        //     // }
        //     }
        for (x, pattern) in local_patterns.iter().enumerate() {
            chart
                .draw_series(LineSeries::new(
                    (0..).zip(pattern.data_points.iter()).map(|(_k, highs)| {
                        let idx = highs.0;
                        let value = highs.1;
                        let date = data[idx].date();
                        (date, value)
                    }),
                    if x < 1 {
                        BLACK.mix(0.2)
                    } else {
                        BLACK.mix(0.2)
                    },
                ))
                .unwrap()
                .label(format!("{:?}", pattern.pattern_type));
        }

        // LOCAL MAXIMA MINIMA

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_maxima.contains(&(i, candle.close())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.high() + (candle.high() * local_peaks_marker_pos),
                        ),
                        -4,
                        BLUE.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.close()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_minima.contains(&(i, candle.close())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() - (candle.low() * local_peaks_marker_pos),
                        ),
                        4,
                        BLUE.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.high()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // EXTREMA MAXIMA MINIMA

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_maxima.contains(&(i, candle.close())) {
                    println!("888888888 {:?}", extrema_maxima);

                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.high() + (candle.high() * extrema_peaks_marker_pos),
                        ),
                        -4,
                        RED.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.close()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_minima.contains(&(i, candle.close())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() - (candle.low() * extrema_peaks_marker_pos),
                        ),
                        4,
                        RED.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.high()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        //breaks out

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_pattern_breaks.contains(&(i)) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() + (candle.low() * (extrema_peaks_marker_pos + 0.1)),
                        ),
                        -4,
                        BLACK.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.close()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();
        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_pattern_breaks.contains(&(i)) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() + (candle.low() * (extrema_peaks_marker_pos + 0.2)),
                        ),
                        -4,
                        BLACK.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.close()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // for x in instrument.peaks().smooth_highs().iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..)
        //                 .zip(instrument.peaks().smooth_highs().iter())
        //                 .map(|(_k, highs)| {
        //                     let idx = highs.0;
        //                     let value = highs.1;
        //                     let date = data[idx].date();
        //                     (date, value)
        //                 }),
        //             &MAGENTA.mix(0.014),
        //         ))
        //         .unwrap();
        // }

        // for x in instrument.peaks().smooth_lows().iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..)
        //                 .zip(instrument.peaks().smooth_lows().iter())
        //                 .map(|(_k, highs)| {
        //                     let idx = highs.0;
        //                     let value = highs.1;
        //                     let date = data[idx].date();
        //                     (date, value)
        //                 }),
        //             MAGENTA.mix(0.014),
        //         ))
        //         .unwrap();
        // }

        for x in instrument.peaks().smooth_close().iter() {
            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(instrument.peaks().smooth_close().iter())
                        .map(|(_k, highs)| {
                            let idx = highs.0;
                            let value = highs.1;
                            let date = data[idx].date();
                            (date, value)
                        }),
                    MAGENTA.mix(0.014),
                ))
                .unwrap();
        }

        // for x in instrument.peaks().smooth_lows().iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..)
        //                 .zip(instrument.peaks().smooth_lows().iter())
        //                 .map(|(_k, highs)| {
        //                     let idx = highs.0;
        //                     let value = highs.1;
        //                     let date = data[idx].date();
        //                     (date, value)
        //                 }),
        //             MAGENTA.mix(0.014),
        //         ))
        //         .unwrap();
        // }

        //     for x in instrument.peaks().smooth_lows().iter() {
        // chart
        //     .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(instrument.peaks().smooth_lows().iter())
        //             .map(|(_k, highs)| {
        //                 let idx = highs.0;
        //                 let value = highs.1;
        //                 let date = data[idx].date();
        //                 (date, value)
        //             }),
        //         BLACK.mix(0.018),
        //     ))
        //     .unwrap();

        // for x in instrument.peaks().smooth_close().iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..)
        //                 .zip(instrument.peaks().smooth_close().iter())
        //                 .map(|(_k, highs)| {
        //                     let idx = highs.0;
        //                     let value = highs.1;
        //                     let date = data[idx].date();
        //                     (date, value)
        //                 }),
        //             &TRANSPARENT,
        //         ))
        //         .unwrap();
        // }

        // for x in local_maxima.iter() {
        //     chart
        //         .draw_series(LineSeries::new(
        //             (0..).zip(local_maxima.iter()).map(|(_k, highs)| {
        //                 let idx = highs.0;
        //                 let value = highs.1;
        //                 let date = data[idx].date();
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
        //       let date = data[idx].date();
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
        //         let date = data[value.0].date();
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
        //         let date = data[value.0].date();
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
                        .map(|(_id, candle)| (candle.date(), *x.1.price())),
                    color,
                ))
                .unwrap();
        }
        */
        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(_id, candle)| {
        //       let date = candle.date();
        //       //let value = tema_a.next(candle.close());
        //       (date, value)
        //     }),
        //     &BLUE,
        //   ))
        //   .unwrap();

        //         chart
        //   .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(data.iter())
        //             .map(|(id, candle)| (candle.date(), tema_a[id])),
        //         &RED,
        //     ))
        //     .unwrap();

        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(_id, candle)| {
        //       let date = candle.date();
        //       let value = tema_c.next(candle.close());
        //       (date, value)
        //     }),
        //     &RED,
        //   ))
        //   .unwrap();

        // chart
        //   .draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(_id, candle)| {
        //       let date = candle.date();
        //       let value = tema_a.next(candle.close());
        //       (date, value)
        //     }),
        //     &YELLOW,
        //   ))
        //   .unwrap();

        // PEAKS

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_maxima.contains(&(i, candle.high())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.high() + candle.high() / local_peaks_marker_pos,
                        ),
                        -4,
                        BLUE.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.high()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if local_minima.contains(&(i, candle.low())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() - candle.low() / local_peaks_marker_pos,
                        ),
                        4,
                        BLUE.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.low()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_maxima.contains(&(i, candle.high())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() - candle.low() / extrema_peaks_marker_pos,
                        ),
                        -4,
                        RED.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.low()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        chart
            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                if extrema_minima.contains(&(i, candle.low())) {
                    return TriangleMarker::new(
                        (
                            candle.date(),
                            candle.low() - candle.low() / local_peaks_marker_pos - 25.,
                        ),
                        4,
                        RED.mix(0.4),
                    );
                } else {
                    return TriangleMarker::new((candle.date(), candle.low()), 0, &TRANSPARENT);
                }
            }))
            .unwrap();

        // let mut rsi_pannel = ChartBuilder::on(&lower_1)
        //     .x_label_area_size(40)
        //     .y_label_area_size(40)
        //     //.margin(2)
        //     //.caption("RSI", ("sans-serif", 8.0).into_font())
        //     .build_cartesian_2d(from_date..to_date, -0f64..100f64)
        //     .unwrap();

        // INDICATORS

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date(), tema_a[id])),
                &RED_LINE,
            ))
            .unwrap();

        // chart
        //     .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(data.iter())
        //             .map(|(id, candle)| (candle.date(), ema_b[id])),
        //         &MAGENTA.mix(0.5),
        //     ))
        //     .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date(), tema_c[id])),
                &BLUE_LINE,
            ))
            .unwrap();

        // rsi_pannel
        //     .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(data.iter())
        //             .map(|(id, candle)| (candle.date(), rsi[id])),
        //         &RED,
        //     ))
        //     .unwrap();

        let mut stoch_pannel = ChartBuilder::on(&indicator_1)
            .x_label_area_size(40)
            .y_label_area_size(40)
            // .margin(2)
            .caption("MACD", ("sans-serif", 8.0).into_font())
            .build_cartesian_2d(from_date..to_date, -0f64..100f64)
            .unwrap();
        //stoch_pannel.configure_mesh().light_line_style(&WHITE).draw().unwrap();
        stoch_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date(), stoch_a[id])),
                &BLUE,
            ))
            .unwrap();

        stoch_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date(), stoch_b[id])),
                &RED,
            ))
            .unwrap();

        let mut macd_pannel = ChartBuilder::on(&indicator_2)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(from_date..to_date, -1f64..1f64)
            .unwrap();

        macd_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date(), macd_a[id])),
                &BLUE,
            ))
            .unwrap();

        macd_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date(), macd_b[id])),
                &RED,
            ))
            .unwrap();

        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("[BACKEND] File saved in {}", output_file);
        Ok(())
    }
}
