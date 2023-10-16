use crate::error::Result;

use chrono::{DateTime, Local};
use rs_algo_shared::helpers::date::{from_dbtime, to_dbtime};

use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::mode::*;
use rs_algo_shared::models::order::{Order, OrderType};
use rs_algo_shared::models::stop_loss::StopLossType;
use rs_algo_shared::models::time_frame;
use rs_algo_shared::models::trade::{TradeIn, TradeOut};
use rs_algo_shared::scanner::instrument::*;
use rs_algo_shared::scanner::pattern::{PatternDirection, PatternType};

use chrono::Duration;
use plotters::prelude::*;

use std::cmp::Ordering;
use std::env;

#[derive(Debug, Clone)]
pub struct Backend;

// #[derive(PartialEq)]

// pub enum ExecutionMode {
//     Instrument,
//     BackTest,
//     Bot,
// }

impl Backend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        mode: ExecutionMode,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        trades: &(&Vec<TradeIn>, &Vec<TradeOut>, &Vec<Order>),
        output_file: &str,
    ) -> Result<()> {
        let data = instrument.data.clone();

        let current_candle = data.last().unwrap();
        let total_len = data.len();
        let from_date = data.first().unwrap().date;
        let to_date = current_candle.date();
        let _current_price = current_candle.close();

        let _price_source = env::var("PRICE_SOURCE").unwrap();

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
        let orders: &Vec<Order> = trades.2;

        let mut stop_loss: Vec<(usize, f64)> = vec![];
        let _stop_loss_types: Vec<(usize, StopLossType)> = vec![];
        let _peaks = instrument.peaks();

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

        // let mut prices_in_ids: Vec<usize> = vec![];
        // let mut prices_out_ids: Vec<usize> = vec![];
        // let mut orders_ids: Vec<usize> = vec![];
        // let mut stop_loss_ids: Vec<usize> = vec![];
        // let mut top_peaks_ids: Vec<usize> = vec![];
        // let mut low_peaks_ids: Vec<usize> = vec![];

        let _last_trade_in: Option<&TradeIn> = trades_in.last();
        let _last_trade_out: Option<&TradeOut> = trades_out.last();
        if mode == ExecutionMode::BackTest || mode == ExecutionMode::Bot {
            //if !trades_out.is_empty() {
            low_points_set = trades_in.iter().map(|x| (x.index_in, x.price_in)).collect();
            // prices_in_ids = trades_in.iter().map(|x| x.id).collect();
            // prices_out_ids = trades_out.iter().map(|x| x.id).collect();
            // orders_ids = orders.iter().map(|x| x.id).collect();
            // //top_peaks_ids = peaks.local_maxima().iter().map(|x| x.).collect();
            // stop_loss_ids = trades_out
            //     .iter()
            //     .filter(|x| x.trade_type.is_stop())
            //     .map(|x| x.id)
            //     .collect();

            top_points_set = trades_out
                .iter()
                .map(|x| (x.index_out, x.price_out))
                .collect();

            stop_loss = trades_out
                .iter()
                .filter(|x| x.trade_type.is_stop())
                .map(|x| (x.id, x.price_out))
                .collect();
        } else {
            top_points_set = instrument.peaks.local_maxima.clone();
            low_points_set = instrument.peaks.local_minima.clone();
        }

        let BACKGROUND = &RGBColor(208, 213, 222);
        let _BLACK_LINE = &RGBColor(0, 0, 0).mix(0.25);
        let CANDLE_BEARISH = &RGBColor(71, 113, 181).mix(0.95);
        let CANDLE_BULLISH = &RGBColor(255, 255, 255).mix(0.95);
        let RED_LINE = &RGBColor(235, 69, 125).mix(0.8);
        let RED_LINE2 = &RGBColor(235, 69, 125).mix(0.20);
        let _BLUE_LINE = &RGBColor(71, 113, 181).mix(0.25);
        let BLUE_LINE2 = &RGBColor(42, 98, 255).mix(0.20);
        let BLUE_LINE3 = &RGBColor(71, 113, 181).mix(0.8);
        let ORANGE_LINE = &RGBColor(245, 127, 22).mix(0.18);
        let _YELLOW_LINE = &RGBColor(255, 229, 0).mix(0.18);
        let _GREEN_LINE = &RGBColor(56, 142, 59).mix(0.8);
        let GREEN_LINE2 = &RGBColor(56, 142, 59).mix(0.16);

        let _bottom_point_color = match points_mode {
            PointsMode::MaximaMinima => BLUE.mix(0.15),
            PointsMode::Trades => BLUE.mix(0.8),
        };

        let _top_point_color = match points_mode {
            PointsMode::MaximaMinima => BLUE.mix(0.10),
            PointsMode::Trades => RED_LINE.mix(1.),
        };

        let _stop_loss_color = MAGENTA.mix(0.8);

        let _rsi = &instrument.indicators.rsi.get_data_a();

        let patterns = local_patterns;

        let macd = &instrument.indicators.macd;
        let macd_a = &macd.get_data_a();
        let macd_b = &macd.get_data_b();

        let _rsi = &instrument.indicators.rsi.get_data_a();

        let _ema_a = &instrument.indicators.ema_a.get_data_a();
        let _ema_b = &instrument.indicators.ema_b.get_data_a();
        let _ema_c = &instrument.indicators.ema_c.get_data_a();

        let bb_a = &instrument.indicators.bb.get_data_a();
        let bb_b = &instrument.indicators.bb.get_data_b();
        let bb_c = &instrument.indicators.bb.get_data_c();

        let root = BitMapBackend::new(&output_file, (1821, 865)).into_drawing_area();
        let (upper, lower) = root.split_vertically((90).percent());

        root.fill(BACKGROUND).unwrap();

        let htf_str = match htf_instrument {
            HTFInstrument::HTFInstrument(htf_ins) => htf_ins.time_frame().to_string(),
            HTFInstrument::None => "".to_owned(),
        };
        let mut chart = ChartBuilder::on(&upper)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .margin(30)
            .caption(
                &[
                    &instrument.symbol,
                    " ",
                    &instrument.time_frame().to_string(),
                    " ",
                    &htf_str,
                ]
                .concat(),
                (font.as_ref(), 14.0).into_font(),
            )
            .build_cartesian_2d(from_date..to_date, min_price..max_price)
            .unwrap();

        chart
            .configure_mesh()
            .light_line_style(BACKGROUND)
            .x_label_formatter(&|v| {
                format!("{}", {
                    match instrument.time_frame() {
                        time_frame::TimeFrameType::D | time_frame::TimeFrameType::W => {
                            v.format("%d-%m-%Y")
                        }
                        time_frame::TimeFrameType::H1 | time_frame::TimeFrameType::H4 => {
                            v.format("%H:%M:%S")
                        }
                        _ => v.format("%H:%M:%S"),
                    }
                })
            })
            .y_label_formatter(&|v| format!("{:.5}", v))
            .draw()
            .unwrap();

        let candle_with = match mode {
            ExecutionMode::Scanner => 3,
            ExecutionMode::Bot => 4,
            _ => 4,
        };

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
                    candle_with,
                )
            }))
            .unwrap();

        // PATTERN NAME
        if mode == ExecutionMode::Scanner {
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
                        ShapeStyle::from(&RED_LINE).filled(),
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
                    RED_LINE2,
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
                    RED_LINE2,
                ))
                .unwrap()
                .label(format!("{:?}", pattern.pattern_type));
            //    }
        }

        // LOCAL MAXIMA MINIMA
        if display_points {
            // chart
            //     .draw_series(data.iter().enumerate().map(|(i, candle)| {
            //         let price;
            //         if points_mode == PointsMode::MaximaMinima {
            //             price = match price_source.as_ref() {
            //                 "highs_lows" => candle.high,
            //                 "close" => candle.close,
            //                 &_ => candle.close,
            //             };
            //         } else {
            //             price = candle.open;
            //         }

            //         if top_points_set.contains(&(i, price)) {
            //             if stop_loss.contains(&(i, price))
            //                 && stop_loss_types.contains(&(i, StopLossType::Trailing))
            //             {
            //                 TriangleMarker::new(
            //                     (candle.date, price + (price * local_peaks_marker_pos)),
            //                     -4,
            //                     MAGENTA.mix(0.8),
            //                 )
            //             } else if stop_loss.contains(&(i, price))
            //                 && stop_loss_types.contains(&(i, StopLossType::Atr))
            //             {
            //                 TriangleMarker::new(
            //                     (candle.date, price + (price * local_peaks_marker_pos)),
            //                     -4,
            //                     RED.mix(0.8),
            //                 )
            //             } else {
            //                 TriangleMarker::new(
            //                     (candle.date, price + (price * local_peaks_marker_pos)),
            //                     -4,
            //                     top_point_color,
            //                 )
            //             }
            //         } else {
            //             TriangleMarker::new((candle.date, price), 0, &TRANSPARENT)
            //         }
            //     }))
            //     .unwrap();

            // MARKERS

            match mode {
                ExecutionMode::Scanner => {
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
                                    TriangleMarker::new((candle.date, candle.close), 0, TRANSPARENT)
                                }
                            }))
                            .unwrap();
                    }
                }

                _ => {}
            };
        }

        let orders_size = match mode {
            ExecutionMode::Scanner => 0,
            ExecutionMode::Bot => 6,
            _ => 5,
        };

        let trades_size = match mode {
            ExecutionMode::Scanner => 0,
            ExecutionMode::Bot => 8,
            _ => 6,
        };

        let stops_size = match mode {
            ExecutionMode::Scanner => 0,
            ExecutionMode::Bot => 5,
            _ => 5,
        };

        //TRADES_IN

        chart
            .draw_series(
                trades_in
                    .iter()
                    .filter(|x| x.date_in >= to_dbtime(data.first().unwrap().date()))
                    .enumerate()
                    .map(|(_i, trade_in)| {
                        let date = from_dbtime(&trade_in.date_in);
                        let _price = trade_in.price_in;

                        match trade_in.trade_type.is_long() {
                            true => TriangleMarker::new(
                                (date, trade_in.price_in - trade_in.spread),
                                trades_size,
                                ORANGE_LINE.mix(5.),
                            ),
                            false => TriangleMarker::new(
                                (date, trade_in.price_in),
                                -trades_size,
                                ORANGE_LINE.mix(5.),
                            ),
                        }
                    }),
            )
            .unwrap();

        //TRADES_IN SPREAD

        chart
            .draw_series(
                trades_in
                    .iter()
                    .filter(|x| x.date_in >= to_dbtime(data.first().unwrap().date()))
                    .enumerate()
                    .map(|(_i, trade_in)| {
                        let date = from_dbtime(&trade_in.date_in);
                        let price = trade_in.price_in;
                        match trade_in.trade_type.is_entry() {
                            true => match trade_in.trade_type.is_long() {
                                true => TriangleMarker::new(
                                    (date, price),
                                    trades_size,
                                    ORANGE_LINE.mix(1.8),
                                ),
                                false => TriangleMarker::new(
                                    (date, price),
                                    -trades_size,
                                    ORANGE_LINE.mix(1.8),
                                ),
                            },
                            false => todo!(),
                        }
                    }),
            )
            .unwrap();

        //TRADES_OUT

        chart
            .draw_series(
                trades_out
                    .iter()
                    .filter(|x| {
                        x.date_out > to_dbtime(data.first().unwrap().date())
                            && !x.trade_type.is_stop()
                    })
                    .enumerate()
                    .map(|(i, trade_out)| {
                        let date = from_dbtime(&trade_out.date_out);
                        let trade_in = trades_in.get(i).unwrap();
                        let price = trade_out.price_out;
                        match trade_out.profit > 0. {
                            true => match trade_out.trade_type.is_long() {
                                true => TriangleMarker::new(
                                    (date, price),
                                    -trades_size,
                                    GREEN_LINE2.mix(5.),
                                ),
                                false => TriangleMarker::new(
                                    (date, price),
                                    trades_size,
                                    GREEN_LINE2.mix(5.),
                                ),
                            },
                            false => match trade_in.trade_type.is_long() {
                                true => TriangleMarker::new(
                                    (date, price),
                                    trades_size,
                                    RED_LINE2.mix(5.),
                                ),
                                false => TriangleMarker::new(
                                    (date, price),
                                    -trades_size,
                                    RED_LINE2.mix(5.),
                                ),
                            },
                        }
                    }),
            )
            .unwrap();

        //ORDERS

        chart
            .draw_series(
                orders
                    .iter()
                    .filter(|x| x.created_at > to_dbtime(data.first().unwrap().date()))
                    .enumerate()
                    .map(|(_i, order)| {
                        let date = from_dbtime(&order.created_at);

                        // let order_opacity = match order.status {
                        //     OrderStatus::Pending => 1.1,
                        //     OrderStatus::Fulfilled => 1.5,
                        //     _ => 0.6,
                        // };

                        let order_opacity = 0.8;
                        match order.order_type {
                            OrderType::BuyOrderLong(_, _, _) => TriangleMarker::new(
                                (date, order.target_price),
                                orders_size,
                                ORANGE_LINE.mix(order_opacity),
                            ),
                            OrderType::BuyOrderShort(_, _, _) => TriangleMarker::new(
                                (date, order.target_price),
                                -orders_size,
                                ORANGE_LINE.mix(order_opacity),
                            ),
                            OrderType::SellOrderLong(_, _, _) => TriangleMarker::new(
                                (date, order.target_price),
                                -orders_size,
                                ORANGE_LINE.mix(order_opacity),
                            ),
                            OrderType::SellOrderShort(_, _, _) => TriangleMarker::new(
                                (date, order.target_price),
                                orders_size,
                                ORANGE_LINE.mix(order_opacity),
                            ),
                            OrderType::TakeProfitLong(_, _, _)
                            | OrderType::TakeProfitShort(_, _, _) => TriangleMarker::new(
                                (date, order.target_price),
                                -orders_size,
                                ORANGE_LINE.mix(order_opacity),
                            ),
                            _ => TriangleMarker::new(
                                (date, order.target_price),
                                orders_size,
                                TRANSPARENT.mix(0.0),
                            ),
                            // OrderType::StopLossLong(_, _) => TriangleMarker::new(
                            //     (date, order.target_price),
                            //     orders_size,
                            //     RED_LINE.mix(0.5),
                            // ),

                            // OrderType::StopLossShort(_, _) => TriangleMarker::new(
                            //     (date, order.target_price),
                            //     -orders_size,
                            //     RED_LINE.mix(0.5),
                            // ),
                        }
                    }),
            )
            .unwrap();

        //STOPLOSS
        chart
            .draw_series(
                orders
                    .iter()
                    .filter(|x| x.order_type.is_stop())
                    //.filter(|x| x.created_at > to_dbtime(data.first().unwrap().date()))
                    .enumerate()
                    .map(|(_i, order)| {
                        let date = from_dbtime(&order.created_at);
                        let price = order.target_price;
                        Circle::new((date, price), stops_size, RED_LINE2.mix(0.8).filled())
                    }),
            )
            .unwrap();

        //ACTIVE STOPLOSS
        chart
            .draw_series(
                trades_out
                    .iter()
                    .filter(|x| {
                        x.date_out > to_dbtime(data.first().unwrap().date())
                            && x.trade_type.is_stop()
                    })
                    .enumerate()
                    .map(|(_i, trade_out)| {
                        let date = from_dbtime(&trade_out.date_out);
                        let price = trade_out.price_out;
                        Circle::new((date, price), stops_size, RED_LINE2.mix(4.8).filled())
                    }),
            )
            .unwrap();

        //BOLLINGER BANDS

        if !bb_a.is_empty() {
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
                    &BLUE_LINE2,
                ))
                .unwrap();
        }

        // //HTF INDICATORS
        match htf_instrument {
            HTFInstrument::HTFInstrument(htf_instrument) => {
                let macd = &htf_instrument.indicators().macd();
                let htf_ema_a = &htf_instrument.indicators().ema_a().get_data_a();
                let htf_ema_b = &htf_instrument.indicators().ema_b().get_data_a();
                let _htf_ema_c = &htf_instrument.indicators().ema_c().get_data_a();
                let htf_macd_a = macd.get_data_a();
                let htf_macd_b = macd.get_data_b();
                let max_macd = htf_macd_a
                    .iter()
                    .max_by(|x, y| x.partial_cmp(y).unwrap())
                    .unwrap();
                let min_macd = htf_macd_a
                    .iter()
                    .min_by(|x, y| x.partial_cmp(y).unwrap())
                    .unwrap();

                let mut indicator_panel = ChartBuilder::on(&lower)
                    .x_label_area_size(40)
                    .y_label_area_size(40)
                    .build_cartesian_2d(from_date..to_date, *min_macd..*max_macd)
                    .unwrap();

                let mut result: Vec<(DateTime<Local>, usize)> = vec![];

                for (htf_id, htf) in htf_instrument.data().iter().enumerate() {
                    let htf_instrument_date = htf.date();
                    let next_htf_id = htf_id + 1;
                    let next_htf_instrument = htf_instrument.data().get(next_htf_id);
                    let next_htf_instrument_date = match next_htf_instrument {
                        Some(x) => x.date(),
                        None => Local::now() - Duration::days(1000),
                    };

                    for (_id, candle) in instrument.data().iter().enumerate() {
                        let instrument_date = candle.date();
                        if htf_instrument_date <= instrument_date
                            && next_htf_instrument_date > instrument_date
                        {
                            result.push((instrument_date, htf_id));
                        }
                    }
                }

                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(result.iter())
                            .map(|(_id, data)| (data.0, htf_macd_a[data.1])),
                        BLUE_LINE3.mix(0.6),
                    ))
                    .unwrap();

                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(result.iter())
                            .map(|(_id, data)| (data.0, htf_macd_b[data.1])),
                        RED_LINE.mix(0.6),
                    ))
                    .unwrap();

                if !htf_ema_a.is_empty() {
                    chart
                        .draw_series(LineSeries::new(
                            result
                                .iter()
                                .enumerate()
                                .map(|(_id, data)| (data.0, htf_ema_a[data.1])),
                            ORANGE_LINE.mix(1.1),
                        ))
                        .unwrap();
                }

                if !htf_ema_b.is_empty() {
                    chart
                        .draw_series(LineSeries::new(
                            result
                                .iter()
                                .enumerate()
                                .map(|(_id, data)| (data.0, htf_ema_b[data.1])),
                            RED_LINE2.mix(1.1),
                        ))
                        .unwrap();
                }

                // if htf_ema_c.len() > 0 {
                //     chart
                //         .draw_series(LineSeries::new(
                //             result
                //                 .iter()
                //                 .enumerate()
                //                 .map(|(id, data)| (data.0, htf_ema_c[data.1])),
                //             RED_LINE2.mix(1.5),
                //         ))
                //         .unwrap();
                // }
            }
            HTFInstrument::None => {
                let mut indicator_panel = ChartBuilder::on(&lower)
                    .x_label_area_size(40)
                    .y_label_area_size(40)
                    .build_cartesian_2d(from_date..to_date, -0f64..100f64)
                    .unwrap();
                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(data.iter())
                            .map(|(id, candle)| (candle.date, macd_a[id])),
                        BLUE_LINE3.mix(0.5),
                    ))
                    .unwrap();

                indicator_panel
                    .draw_series(LineSeries::new(
                        (0..)
                            .zip(data.iter())
                            .map(|(id, candle)| (candle.date, macd_b[id])),
                        RED_LINE.mix(0.5),
                    ))
                    .unwrap();
            }
        };

        root.present().expect(" Error. Can't save file!");
        log::info!(" File saved in {}", output_file);
        Ok(())
    }
}
