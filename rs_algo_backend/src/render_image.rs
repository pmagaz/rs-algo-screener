use crate::error::Result;
use plotters::prelude::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::pattern::*;
use std::cmp::Ordering;
use std::env;

#[derive(Debug, Clone)]
pub struct Backend;

impl Backend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        instrument: &Instrument,
        trades: &(&Vec<TradeIn>, &Vec<TradeOut>),
        output_file: &str,
    ) -> Result<()> {
        let data = instrument.data.clone();
        let total_len = data.len();
        let from_date = data.first().unwrap().date;
        let to_date = data.last().unwrap().date;

        let price_source = env::var("PRICE_SOURCE").unwrap();

        let font = env::var("PLOTTER_FONT").unwrap();

        let display_points = env::var("DISPLAY_POINTS").unwrap().parse::<bool>().unwrap();

        let local_peaks_marker_pos = env::var("LOCAL_PEAKS_MARKERS_POS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let _extrema_peaks_marker_pos = env::var("EXTREMA_PEAKS_MARKERS_POS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        #[derive(Debug, PartialEq)]
        pub enum PointsMode {
            MaximaMinima,
            Trades,
        }

        let trades_in: &Vec<TradeIn> = trades.0;
        let trades_out: &Vec<TradeOut> = trades.1;
        let mut stop_loss: Vec<(usize, f64)> = vec![];

        let points_mode = match trades_out.len().cmp(&0) {
            Ordering::Greater => PointsMode::Trades,
            Ordering::Equal => PointsMode::MaximaMinima,
            Ordering::Less => PointsMode::MaximaMinima,
        };

        let min_price = data.iter().map(|candle| candle.low).fold(0. / 0., f64::min);
        let max_price = data
            .iter()
            .map(|candle| candle.high)
            .fold(0. / 0., f64::max);

        let _extrema_maxima = &instrument.peaks.extrema_maxima;
        let _extrema_minima = &instrument.peaks.extrema_minima;

        let local_patterns = &instrument.patterns.local_patterns;
        let local_pattern_breaks: Vec<usize> = instrument
            .patterns
            .local_patterns
            .iter()
            .map(|x| x.active.index)
            .collect();

        let top_points_set: Vec<(usize, f64)>;
        let low_points_set: Vec<(usize, f64)>;

        if !trades_out.is_empty() {
            low_points_set = trades_in.iter().map(|x| (x.index_in, x.price_in)).collect();

            top_points_set = trades_out
                .iter()
                .map(|x| (x.index_out, x.price_out))
                .collect();

            stop_loss = trades_out
                .iter()
                .filter(|x| x.trade_type == TradeType::StopLoss)
                .map(|x| (x.index_out, x.price_out))
                .collect();
        } else {
            top_points_set = instrument.peaks.local_maxima.clone();
            low_points_set = instrument.peaks.local_minima.clone();
        }

        let BACKGROUND = &RGBColor(208, 213, 222);
        let CANDLE_BEARISH = &RGBColor(71, 113, 181).mix(0.95);
        let CANDLE_BULLISH = &RGBColor(255, 255, 255).mix(0.95);
        let RED_LINE = &RGBColor(235, 69, 125).mix(0.8);
        let BLUE_LINE = &RGBColor(71, 113, 181).mix(0.3);
        let BLUE_LINE2 = &RGBColor(42, 98, 255).mix(0.3);
        let BLUE_LINE3 = &RGBColor(71, 113, 181).mix(0.8);
        let ORANGE_LINE = &RGBColor(245, 127, 22).mix(0.25);
        let _GREEN_LINE = &RGBColor(56, 142, 59);

        let bottom_point_color = match points_mode {
            PointsMode::MaximaMinima => BLUE.mix(0.15),
            PointsMode::Trades => BLUE.mix(0.6),
        };

        let top_point_color = match points_mode {
            PointsMode::MaximaMinima => BLUE.mix(0.15),
            PointsMode::Trades => RED_LINE.mix(1.),
        };

        let stop_loss_color = MAGENTA.mix(0.8);

        let rsi = &instrument.indicators.rsi.data_a;

        let patterns = local_patterns;
        let stoch = &instrument.indicators.stoch;
        let stoch_a = &stoch.data_a;
        let stoch_b = &stoch.data_b;

        let macd = &instrument.indicators.macd;
        let _macd_a = &macd.data_a;
        let _macd_b = &macd.data_b;

        let _rsi = &instrument.indicators.rsi.data_a;

        let _ema_a = &instrument.indicators.ema_a.data_a;
        let _ema_b = &instrument.indicators.ema_b.data_a;
        let _ema_c = &instrument.indicators.ema_c.data_a;

        let bb_a = &instrument.indicators.bb.data_a;
        let bb_b = &instrument.indicators.bb.data_b;
        let bb_c = &instrument.indicators.bb.data_c;

        //let root = BitMapBackend::new(&output_file, (1536, 1152)).into_drawing_area();
        let root = BitMapBackend::new(&output_file, (1361, 1021)).into_drawing_area();
        let (upper, lower) = root.split_vertically((85).percent());
        let (indicator_1, indicator_2) = lower.split_vertically((50).percent());

        root.fill(BACKGROUND).unwrap();

        let mut chart = ChartBuilder::on(&upper)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(15)
            .caption(&instrument.symbol, (font.as_ref(), 14.0).into_font())
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
            .draw_series(data.iter().enumerate().map(|(_id, candle)| {
                let (bullish, bearish): (ShapeStyle, ShapeStyle) = match candle {
                    _x if candle.close >= candle.open => {
                        (CANDLE_BULLISH.filled(), CANDLE_BULLISH.filled())
                    }
                    _x if candle.close <= candle.open => {
                        (CANDLE_BEARISH.filled(), CANDLE_BEARISH.filled())
                    }
                    _ => (CANDLE_BULLISH.filled(), CANDLE_BULLISH.filled()),
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

        // PATTERN NAME
        //if points_mode == PointsMode::MaximaMinima {
        for (_x, pattern) in patterns
            .iter()
            .filter(|pat| {
                pat.pattern_type != PatternType::HigherHighsHigherLows
                    && pat.pattern_type != PatternType::LowerHighsLowerLows
            })
            .enumerate()
        {
            chart
                .draw_series(PointSeries::of_element(
                    (0..)
                        .zip(pattern.data_points.iter())
                        .filter(|(_i, highs)| highs.0 < total_len)
                        .map(|(i, highs)| {
                            let idx = highs.0;
                            let value = highs.1;
                            let date = data[idx].date;
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
                                (font.as_ref(), 12),
                            )
                        } else {
                            pattern_name =
                                Text::new(format!("{:?}", ""), (0, 12), (font.as_ref(), 0))
                        }

                        EmptyElement::at(new_coord) + pattern_name
                    },
                ))
                .unwrap();
        }

        // PATTERN LINE
        for (_x, pattern) in local_patterns
            .iter()
            .filter(|pat| {
                pat.pattern_type != PatternType::HigherHighsHigherLows
                    && pat.pattern_type != PatternType::LowerHighsLowerLows
            })
            .enumerate()
        {
            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(pattern.data_points.iter())
                        .enumerate()
                        .filter(|(key, (_i, highs))| highs.0 < total_len && key % 2 == 0)
                        .map(|(_key, (_i, highs))| {
                            let idx = highs.0;
                            let value = highs.1;
                            let date = data[idx].date;
                            (date, value)
                        }),
                    RED_LINE.mix(0.40),
                ))
                .unwrap()
                .label(format!("{:?}", pattern.pattern_type));
        }

        for (_x, pattern) in local_patterns
            .iter()
            .filter(|pat| {
                pat.pattern_type != PatternType::HigherHighsHigherLows
                    && pat.pattern_type != PatternType::LowerHighsLowerLows
            })
            .enumerate()
        {
            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(pattern.data_points.iter())
                        .enumerate()
                        .filter(|(key, (_i, highs))| highs.0 < total_len && key % 2 != 0)
                        .map(|(_key, (_i, highs))| {
                            let idx = highs.0;
                            let value = highs.1;
                            let date = data[idx].date;
                            (date, value)
                        }),
                    RED_LINE.mix(0.30),
                ))
                .unwrap()
                .label(format!("{:?}", pattern.pattern_type));
            //    }
        }

        // LOCAL MAXIMA MINIMA
        if display_points {
            chart
                .draw_series(data.iter().enumerate().map(|(i, candle)| {
                    let price;
                    if points_mode == PointsMode::MaximaMinima {
                        price = match price_source.as_ref() {
                            "highs_lows" => candle.high,
                            "close" => candle.close,
                            &_ => candle.close,
                        };
                    } else {
                        price = candle.open;
                    }

                    if top_points_set.contains(&(i, price)) {
                        if stop_loss.contains(&(i, price)) {
                            TriangleMarker::new(
                                (candle.date, price + (price * local_peaks_marker_pos)),
                                -4,
                                stop_loss_color,
                            )
                        } else {
                            TriangleMarker::new(
                                (candle.date, price + (price * local_peaks_marker_pos)),
                                -4,
                                top_point_color,
                            )
                        }
                    } else {
                        TriangleMarker::new((candle.date, price), 0, &TRANSPARENT)
                    }
                }))
                .unwrap();

            chart
                .draw_series(data.iter().enumerate().map(|(i, candle)| {
                    let price;
                    if points_mode == PointsMode::MaximaMinima {
                        price = match price_source.as_ref() {
                            "highs_lows" => candle.low,
                            "close" => candle.close,
                            &_ => candle.close,
                        };
                    } else {
                        price = candle.open;
                    }

                    if low_points_set.contains(&(i, price)) {
                        TriangleMarker::new(
                            (candle.date, price - (price * local_peaks_marker_pos)),
                            4,
                            bottom_point_color,
                        )
                    } else {
                        TriangleMarker::new((candle.date, price), 0, &TRANSPARENT)
                    }
                }))
                .unwrap();

            if points_mode == PointsMode::MaximaMinima {
                chart
                    .draw_series(data.iter().enumerate().map(|(i, candle)| {
                        if local_pattern_breaks.contains(&(i)) {
                            let mut direction: (i32, f64) = (0, 0.);

                            for n in instrument.patterns.local_patterns.iter().filter(|pat| {
                                pat.pattern_type != PatternType::HigherHighsHigherLows
                                    && pat.pattern_type != PatternType::LowerHighsLowerLows
                            }) {
                                if n.active.index == i {
                                    let pos = match n.active.break_direction {
                                        PatternDirection::Bottom => (4, candle.low),
                                        PatternDirection::Top => (-4, candle.high),
                                        PatternDirection::None => (4, candle.close),
                                    };
                                    direction = pos;
                                }
                            }

                            TriangleMarker::new(
                                (
                                    candle.date,
                                    direction.1 - (direction.1 * local_peaks_marker_pos - 2.),
                                ),
                                direction.0,
                                RED_LINE.mix(0.3),
                            )
                        } else {
                            TriangleMarker::new((candle.date, candle.close), 0, &TRANSPARENT)
                        }
                    }))
                    .unwrap();
            }
        }

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, bb_a[id])),
                &BLUE_LINE2,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, bb_b[id])),
                &BLUE_LINE2,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, bb_c[id])),
                &ORANGE_LINE,
            ))
            .unwrap();

        let mut rsi_pannel = ChartBuilder::on(&indicator_1)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(from_date..to_date, -0f64..100f64)
            .unwrap();

        rsi_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, rsi[id])),
                RED_LINE,
            ))
            .unwrap();

        //STOCH PANNEL

        let mut stoch_pannel = ChartBuilder::on(&indicator_2)
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
                BLUE_LINE3,
            ))
            .unwrap();

        stoch_pannel
            .draw_series(LineSeries::new(
                (0..)
                    .zip(data.iter())
                    .map(|(id, candle)| (candle.date, stoch_b[id])),
                RED_LINE,
            ))
            .unwrap();

        root.present().expect("[BACKEND] Error. Can't save file!");
        println!("[BACKEND] File saved in {}", output_file);
        Ok(())
    }
}
