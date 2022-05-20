use super::strategy::Strategy;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;
pub struct BollingerBands<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "Bollinger_Bands_RiddingBands",
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let current_close = instrument.data.get(index).unwrap().close;
        let high_price = instrument.data.get(index).unwrap().high;

        let prev_close = instrument.data.get(prev_index).unwrap().close;
        let prev_high = instrument.data.get(prev_index).unwrap().high;

        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let mid_band = instrument.indicators.bb.data_c.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();
        let prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let max = 5;
        let mut last_prices = vec![];
        let mut top_occurrences: usize = 0;
        let mut low_occurrences: usize = 0;
        let mut price_change: f64 = 0.;

        #[derive(Debug, PartialEq)]
        enum Direction {
            Up,
            Down,
            Side,
        };

        if index > max {
            for x in (index - max..index).rev() {
                let close_price = instrument.data.get(x).unwrap().close;
                if close_price > *mid_band {
                    top_occurrences += 1;
                }
                if close_price < *mid_band {
                    low_occurrences += 1;
                }
            }
        }

        if index > 10 {
            for x in (index - 10..index).rev() {
                let close_price = instrument.data.get(x).unwrap().close;
                last_prices.push(close_price);
            }
        }

        let max_price_change = last_prices
            .iter()
            .map(|x| percentage_change(*x, current_close))
            .fold(0. / 0., f64::max);

        let direction = match max_price_change {
            _x if top_occurrences > low_occurrences * 2 && max_price_change > 5. => Direction::Up,
            _x if low_occurrences < top_occurrences * 2 => Direction::Down,
            _ => Direction::Side,
        };

        println!(
            "1111 {} {} {}",
            top_occurrences, low_occurrences, max_price_change
        );

        let entry_condition = direction == Direction::Up && top_occurrences >= max;
        //let entry_condition = close_price < low_band && prev_close >= prev_low_band;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;
        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let exit_condition = close_price < low_band && prev_close >= prev_low_band;

        resolve_trade_out(index, instrument, trade_in, exit_condition)
    }

    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
        equity: f64,
        commission: f64,
    ) -> BackTestResult {
        resolve_backtest(
            instrument, trades_in, trades_out, self.name, equity, commission,
        )
    }
}
