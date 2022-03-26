use round::round;
use rs_algo_shared::helpers::date::{Local, DateTime,Datelike};
use rs_algo_shared::models::{CompactInstrument, PatternType, PatternDirection};
use yew::{function_component, html, Callback, use_state, Properties};
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn open_modal(modal: &str);
    fn close_modal(modal: &str);
}

pub enum IndicatorStatus {
    Bullish,
    Bearish,
    Neutral
}


#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub instruments: Vec<CompactInstrument>,
    pub on_symbol_click: Callback<String>,
}

#[function_component(InstrumentsList)]
pub fn instrument_list(props: &Props
) -> Html {
        let Props { instruments, on_symbol_click } = props;
    let base_url = "http://192.168.1.10/api/instruments?symbol=";
    let use_url = use_state(|| String::from(""));

      
                 fn get_status_class<'a>(status: IndicatorStatus) -> &'a str {
                     let class = match status {
                         IndicatorStatus::Bullish => "has-background-success-light", 
                         IndicatorStatus::Neutral => "has-background-warning-light", 
                         IndicatorStatus::Bearish => "has-background-danger-light", 
                     };
                     class

                 }

    instruments
        .iter()
        .map(|instrument| {
            let on_instrument_select = {
                let on_symbol_click = on_symbol_click.clone();
                let instrument = instrument.clone();
                let url = [base_url,&instrument.symbol].concat();
                open_modal("modal");
                Callback::from(move |_| on_symbol_click.emit(url.clone()))
            };
            

            let patterns = instrument.patterns.local_patterns.get(0); 
            let pattern_type = match patterns {
                Some(val) => val.pattern_type.clone(),
                None   => PatternType::None,
            };

            let break_direction = match patterns {
                Some(val) => val.active.break_direction.clone(),
                None   => PatternDirection::None,
            };

            let pattern_date = match patterns {
                Some(val) => val.active.date.to_chrono(),
                None   => DateTime::from(Local::now())
            };
            
            let pattern_change = match patterns {
                Some(val) => round(val.active.change, 2),
                None   => 0.,
            };

            //FIXME move this to server???? strategies?
            let macd = instrument.indicators.macd.clone();
            let macd_status: IndicatorStatus= match macd {
                _x if macd.current_a > macd.current_b && macd.current_a > 0. =>IndicatorStatus::Bullish,
                _x if macd.current_a < macd.current_b => IndicatorStatus::Bearish,
                _x if macd.current_a > macd.current_b && macd.current_a < 0. => IndicatorStatus::Neutral,
                _ => IndicatorStatus::Neutral,
            };

            let stoch = instrument.indicators.stoch.clone();
            let stoch_status: IndicatorStatus= match stoch {
                _x if stoch.current_a > stoch.current_b && stoch.current_a > 20. && stoch.current_a < 30. =>IndicatorStatus::Bullish,
                _x if stoch.current_a < stoch.current_b => IndicatorStatus::Bearish,
                _x if stoch.current_a > stoch.current_b  && stoch.current_a > 40. => IndicatorStatus::Neutral,
                _ => IndicatorStatus::Neutral,
            };

            let rsi = instrument.indicators.rsi.clone();
            let rsi_status: IndicatorStatus= match rsi {
                _x if rsi.current_a < 30. => IndicatorStatus::Bullish,
                _x if rsi.current_a > 60. =>IndicatorStatus::Bearish,
                _x if rsi.current_a > 40. && rsi.current_a < 60. => IndicatorStatus::Neutral,
                _ => IndicatorStatus::Neutral,
            };

            let ema_a = instrument.indicators.ema_a.clone(); //50
            let ema_b = instrument.indicators.ema_b.clone(); //21
            let ema_c = instrument.indicators.ema_c.clone(); //9
            let ema_status: IndicatorStatus= match ema_a {
                _x if ema_c.current_a > ema_b.current_a && ema_b.current_a > ema_a.current_a  => IndicatorStatus::Bullish,
                _x if ema_c.current_a < ema_b.current_a && ema_b.current_a > ema_a.current_a  => IndicatorStatus::Bearish,
                _x if ema_c.current_a < ema_b.current_a && ema_b.current_a > ema_a.current_a  => IndicatorStatus::Neutral,
                _ => IndicatorStatus::Neutral,
            };

            let date = instrument.date.to_chrono();

            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>{format!("{}", instrument.symbol)}</a></td>
                    //<td> <a href={format!("{}{}", base_url, instrument.symbol)}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td> {format!("{:?}", instrument.current_candle)}</td>
                    <td> {format!("{:?}", pattern_type)}</td>
                    <td> {format!("{:?}", break_direction)}</td>
                    <td> {format!("{}%", pattern_change)}</td>
                    <td> {format!("{}", pattern_date.format("%d/%m/%Y"))}</td>
                    <td class={get_status_class(stoch_status)}> {format!("{:?} / {:?}", round(instrument.indicators.stoch.current_a, 1), round(instrument.indicators.stoch.current_b, 2))}</td>
                    <td class={get_status_class(macd_status)}> {format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 1), round(instrument.indicators.macd.current_b, 2))}</td>
                    <td class={get_status_class(rsi_status)}>  {format!("{:?}", round(instrument.indicators.rsi.current_a, 1))}</td>
                    <td class={get_status_class(ema_status)}> {format!("{:?} / {:?}", round(instrument.indicators.ema_a.current_a, 1), round(instrument.indicators.ema_b.current_a, 2))}</td>
                    <td> {format!("{}", date.format("%R"))}</td>
                </tr>
            }
        })
        .collect()
}
