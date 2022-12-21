use crate::error::Result;

use plotters::style::full_palette::RED;
use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::stop_loss::StopLossType;
use rs_algo_shared::models::trade::{TradeIn, TradeOut, TradeType};
use rs_algo_shared::scanner::instrument::*;
use rs_algo_shared::scanner::pattern::{PatternDirection, PatternType};

use plotters::prelude::*;
use round::round;
use std::cmp::Ordering;
use std::env;

#[derive(Debug, Clone)]
pub struct Backend;

#[derive(PartialEq)]

pub enum BackendMode {
    Instrument,
    BackTest,
    Bot,
}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        mode: BackendMode,
        instrument: &Instrument,
        htf_instrument: &HigherTMInstrument,
        trades: &(&Vec<TradeIn>, &Vec<TradeOut>),
        output_file: &str,
    ) -> Result<()> {
        let data = instrument.data.clone();
        let total_len = data.len();
        let from_date = data.first().unwrap().date;
        let to_date = data.last().unwrap().date;

        let current_candle = instrument.data.last().unwrap();

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
        let last_trade_in: Option<&TradeIn>;
        let last_trade_out: Option<&TradeOut>;
        let mut stop_loss: Vec<(usize, f64)> = vec![];
        let mut stop_loss_types: Vec<(usize, StopLossType)> = vec![];
        let peaks = instrument.peaks();

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

        let mut prices_in_indexes: Vec<usize> = vec![];
        let mut prices_out_indexes: Vec<usize> = vec![];
        let mut stop_loss_indexes: Vec<usize> = vec![];
        let mut top_peaks_indexes: Vec<usize> = vec![];
        let mut low_peaks_indexes: Vec<usize> = vec![];

        last_trade_in = trades_in.last();
        last_trade_out = trades_out.last();
        if mode == BackendMode::BackTest || mode == BackendMode::Bot {
            //if !trades_out.is_empty() {
            low_points_set = trades_in.iter().map(|x| (x.index_in, x.price_in)).collect();
            prices_in_indexes = trades_in.iter().map(|x| x.index_in).collect();
            prices_out_indexes = trades_out.iter().map(|x| x.index_out).collect();
            //top_peaks_indexes = peaks.local_maxima().iter().map(|x| x.).collect();
            stop_loss_indexes = trades_out
                .iter()
                .filter(|x| x.trade_type == TradeType::StopLoss)
                .map(|x| x.index_out)
                .collect();

            top_points_set = trades_out
                .iter()
                .map(|x| (x.index_out, x.price_out))
                .collect();

            stop_loss = trades_out
                .iter()
                .filter(|x| x.trade_type == TradeType::StopLoss)
                .map(|x| (x.index_out, x.price_out))
                .collect();

            stop_loss_types = trades_in
                .iter()
                .map(|x| (x.index_in, x.stop_loss.stop_type.to_owned()))
                .collect();
        } else {
            top_points_set = instrument.peaks.local_maxima.clone();
            low_points_set = instrument.peaks.local_minima.clone();
        }

        let BACKGROUND = &RGBColor(208, 213, 222);
        let BLACK_LINE = &RGBColor(0, 0, 0).mix(0.25);
        let CANDLE_BEARISH = &RGBColor(71, 113, 181).mix(0.95);
        let CANDLE_BULLISH = &RGBColor(255, 255, 255).mix(0.95);
        let RED_LINE = &RGBColor(235, 69, 125).mix(0.8);
        let RED_LINE2 = &RGBColor(235, 69, 125).mix(0.20);
        let BLUE_LINE = &RGBColor(71, 113, 181).mix(0.25);
        let BLUE_LINE2 = &RGBColor(42, 98, 255).mix(0.20);
        let BLUE_LINE3 = &RGBColor(71, 113, 181).mix(0.8);
        let ORANGE_LINE = &RGBColor(245, 127, 22).mix(0.18);
        let YELLOW_LINE = &RGBColor(255, 229, 0).mix(0.18);
        let GREEN_LINE = &RGBColor(56, 142, 59).mix(0.8);
        let GREEN_LINE2 = &RGBColor(56, 142, 59).mix(0.16);

        let bottom_point_color = match points_mode {
            PointsMode::MaximaMinima => BLUE.mix(0.15),
            PointsMode::Trades => BLUE.mix(0.8),
        };

        let top_point_color = match points_mode {
            PointsMode::MaximaMinima => BLUE.mix(0.10),
            PointsMode::Trades => RED_LINE.mix(1.),
        };

        let _stop_loss_color = MAGENTA.mix(0.8);

        let rsi = &instrument.indicators.rsi.get_data_a();

        let patterns = local_patterns;
        let stoch = &instrument.indicators.stoch;
        let stoch_a = &stoch.get_data_a();
        let stoch_b = &stoch.get_data_b();

        let macd = &instrument.indicators.macd;
        let macd_a = &macd.get_data_a();
        let macd_b = &macd.get_data_b();

        let _rsi = &instrument.indicators.rsi.get_data_a();

        let _ema_a = &instrument.indicators.ema_a.get_data_a();
        let ema_b = &instrument.indicators.ema_b.get_data_a();
        let ema_c = &instrument.indicators.ema_c.get_data_a();

        let bb_a = &instrument.indicators.bb.get_data_a();
        let bb_b = &instrument.indicators.bb.get_data_b();
        let bb_c = &instrument.indicators.bb.get_data_c();

        //let root = BitMapBackend::new(&output_file, (1536, 1152)).into_drawing_area();
        let root = BitMapBackend::new(&output_file, (1321, 765)).into_drawing_area();
        let (upper, lower) = root.split_vertically((90).percent());
        // let (indicator_1, indicator_2) = lower.split_vertically((50).percent());

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
            .x_label_formatter(&|v| format!("{:.3}", v))
            .y_label_formatter(&|v| format!("{:.3}", v))
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
                    RED_LINE.mix(0.40),
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
                        if stop_loss.contains(&(i, price))
                            && stop_loss_types.contains(&(i, StopLossType::Trailing))
                        {
                            TriangleMarker::new(
                                (candle.date, price + (price * local_peaks_marker_pos)),
                                -4,
                                MAGENTA.mix(0.8),
                            )
                        } else if stop_loss.contains(&(i, price))
                            && stop_loss_types.contains(&(i, StopLossType::Atr))
                        {
                            TriangleMarker::new(
                                (candle.date, price + (price * local_peaks_marker_pos)),
                                -4,
                                RED.mix(0.8),
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

            // MARKERS

            match mode {
                BackendMode::Instrument => {
                    if points_mode == PointsMode::MaximaMinima {
                        chart
                            .draw_series(data.iter().enumerate().map(|(i, candle)| {
                                if local_pattern_breaks.contains(&(i)) {
                                    let mut direction: (i32, f64) = (0, 0.);

                                    for n in
                                        instrument.patterns.local_patterns.iter().filter(|pat| {
                                            pat.pattern_type != PatternType::HigherHighsHigherLows
                                                && pat.pattern_type
                                                    != PatternType::LowerHighsLowerLows
                                        })
                                    {
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
                                            direction.1
                                                - (direction.1 * local_peaks_marker_pos - 2.),
                                        ),
                                        direction.0,
                                        RED_LINE.mix(0.3),
                                    )
                                } else {
                                    TriangleMarker::new(
                                        (candle.date, candle.close),
                                        0,
                                        &TRANSPARENT,
                                    )
                                }
                            }))
                            .unwrap();
                    }
                }
                BackendMode::BackTest => {
                    chart
                        .draw_series(data.iter().enumerate().map(|(i, candle)| {
                            let price;
                            price = candle.open;
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
                }
                BackendMode::Bot => {
                    chart
                        .draw_series(data.iter().enumerate().map(|(_i, candle)| {
                            let index = candle.date().timestamp_millis() as usize;
                            let price = candle.close;

                            if prices_in_indexes.contains(&index) {
                                Circle::new(
                                    (candle.date, price),
                                    5,
                                    Into::<ShapeStyle>::into(&BLUE_LINE3).filled(),
                                )
                                .into_dyn()
                            } else {
                                Circle::new(
                                    (candle.date, price),
                                    0,
                                    Into::<ShapeStyle>::into(&TRANSPARENT).filled(),
                                )
                                .into_dyn()
                            }
                        }))
                        .unwrap();

                    //TRADES_IN

                    chart
                        .draw_series(PointSeries::of_element(
                            (0..)
                                .zip(data.iter())
                                .filter(|(i, candle)| {
                                    let index = candle.date().timestamp_millis() as usize;
                                    prices_in_indexes.contains(&index)
                                })
                                .map(|(i, candle)| {
                                    let date = candle.date();
                                    let price = candle.close();
                                    (date, price)
                                }),
                            5,
                            ShapeStyle::from(&BLUE_LINE3).filled(),
                            &|coord, size: i32, style| {
                                let new_coord = (coord.0, coord.1);
                                let (date, price) = coord;
                                let index = date.timestamp_millis() as usize;
                                let trade_in_index =
                                    prices_in_indexes.iter().position(|&x| x == index).unwrap();

                                let trade_in = trades_in.get(trade_in_index).unwrap();

                                EmptyElement::at(coord)
                                    + Circle::new((0, 0), size, style)
                                    + Text::new(
                                        format!(
                                            "{:?} / {:?} / {:?}",
                                            round(trade_in.price_in, 3),
                                            round(trade_in.ask, 3),
                                            round(trade_in.spread, 3)
                                        ),
                                        (0, 20),
                                        ("sans-serif", 12),
                                    )
                            },
                        ))
                        .unwrap();

                    // TRADES OUT

                    chart
                        .draw_series(PointSeries::of_element(
                            (0..)
                                .zip(data.iter())
                                .filter(|(i, candle)| {
                                    let index = candle.date().timestamp_millis() as usize;
                                    prices_out_indexes.contains(&index)
                                })
                                .map(|(i, candle)| {
                                    let date = candle.date();
                                    let price = candle.close();
                                    (date, price)
                                }),
                            5,
                            ShapeStyle::from(&RED_LINE).filled(),
                            &|coord, size: i32, style| {
                                let (date, price) = coord;
                                let index = date.timestamp_millis() as usize;
                                let trade_out_index =
                                    prices_out_indexes.iter().position(|&x| x == index).unwrap();

                                let trade_out = trades_out.get(trade_out_index).unwrap();

                                let style = match trade_out.trade_type {
                                    TradeType::StopLoss => ShapeStyle::from(&RED_LINE).filled(),
                                    _ => match trade_out.profit_per {
                                        _ if trade_out.profit_per > 0. => {
                                            ShapeStyle::from(&GREEN_LINE).filled()
                                        }
                                        _ if trade_out.profit_per < 0. => {
                                            ShapeStyle::from(&RED_LINE).filled()
                                        }
                                        _ => ShapeStyle::from(&BLUE_LINE).filled(),
                                    },
                                };

                                EmptyElement::at(coord)
                                    + Circle::new((0, 0), size, style)
                                    + Text::new(
                                        format!(
                                            "{:?} / {:?} %",
                                            round(trade_out.price_out, 3),
                                            round(trade_out.profit_per, 2)
                                        ),
                                        (0, 20),
                                        ("sans-serif", 12),
                                    )
                            },
                        ))
                        .unwrap();

                    //ACTIVE STOP LOSS
                    chart
                        .draw_series(LineSeries::new(
                            (0..)
                                .zip(data.iter())
                                .map(|(id, candle)| match last_trade_in {
                                    Some(trade_in) => match last_trade_out {
                                        Some(trade_out) => {
                                            if trade_out.index_in != trade_in.index_in {
                                                (candle.date, trade_in.stop_loss.price)
                                            } else {
                                                (candle.date, 0.)
                                            }
                                        }
                                        None => (candle.date, trade_in.stop_loss.price),
                                    },
                                    None => (candle.date, 0.),
                                }),
                            &RED_LINE,
                        ))
                        .unwrap();
                }
            };
        }

        //BOLLINGER BANDS

        if bb_a.len() > 0 {
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
                    &GREEN_LINE2,
                ))
                .unwrap();
        }

        //EMAS

        if ema_c.len() > 0 && ema_b.len() <= 0 {
            chart
                .draw_series(LineSeries::new(
                    data.iter()
                        .enumerate()
                        .map(|(id, candle)| (candle.date, ema_c[id])),
                    RED_LINE2,
                ))
                .unwrap();
        } else if ema_c.len() > 0 && ema_b.len() > 0 {
            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(data.iter())
                        .filter(|(id, candle)| ema_c[*id] <= ema_b[*id])
                        .map(|(id, candle)| (candle.date, ema_c[id])),
                    ORANGE_LINE,
                ))
                .unwrap();

            chart
                .draw_series(LineSeries::new(
                    (0..)
                        .zip(data.iter())
                        .filter(|(id, candle)| ema_b[*id] < ema_c[*id])
                        .map(|(id, candle)| (candle.date, ema_c[id])),
                    RED_LINE2,
                ))
                .unwrap();
        }

        //OPEN POSITION

        // let mut rsi_pannel = ChartBuilder::on(&indicator_1)
        //     .x_label_area_size(40)
        //     .y_label_area_size(40)
        //     .build_cartesian_2d(from_date..to_date, -0f64..100f64)
        //     .unwrap();

        // rsi_pannel
        //     .draw_series(LineSeries::new(
        //         (0..)
        //             .zip(data.iter())
        //             .map(|(id, candle)| (candle.date, rsi[id])),
        //         RED_LINE,
        //     ))
        //     .unwrap();

        // //STOCH PANNEL

        let mut indicator_panel = ChartBuilder::on(&lower)
            .x_label_area_size(40)
            .y_label_area_size(40)
            // .margin(2)
            //.caption("MACD", (font.as_ref(), 8.0).into_font())
            .build_cartesian_2d(from_date..to_date, -0f64..100f64)
            .unwrap();
        //indicator_panel.configure_mesh().light_line_style(&WHITE).draw().unwrap();

        match htf_instrument {
            HigherTMInstrument::HigherTMInstrument(htf_instrument) => {
                let macd = &htf_instrument.indicators.macd;
                let macd_a = &macd.get_data_a();
                let macd_b = &macd.get_data_b();
                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(htf_instrument.data.iter())
                            .map(|(id, candle)| (candle.date, macd_a[id])),
                        BLUE_LINE3,
                    ))
                    .unwrap();

                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(htf_instrument.data.iter())
                            .map(|(id, candle)| (candle.date, macd_b[id])),
                        RED_LINE,
                    ))
                    .unwrap();
            }
            HigherTMInstrument::None => {
                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(data.iter())
                            .map(|(id, candle)| (candle.date, stoch_a[id])),
                        BLUE_LINE3,
                    ))
                    .unwrap();

                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(data.iter())
                            .map(|(id, candle)| (candle.date, stoch_b[id])),
                        RED_LINE,
                    ))
                    .unwrap();
            }
        };

        root.present().expect(" Error. Can't save file!");
        log::info!(" File saved in {}", output_file);
        Ok(())
    }
}
