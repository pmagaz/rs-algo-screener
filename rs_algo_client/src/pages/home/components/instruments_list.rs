use crate::helpers::status::*;

use round::{round};
use rs_algo_shared::models::pattern::*;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::status::Status;
use rs_algo_shared::models::divergence::*;
use rs_algo_shared::models::candle::*;

use rs_algo_shared::models::horizontal_level::*;
use rs_algo_shared::helpers::date::{Local, DateTime, Utc, Duration};
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::status::*;
use std::env;
use yew::{function_component, html, Callback, Properties, Html};
use wasm_bindgen::prelude::*;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn open_modal(modal: &str);
    #[wasm_bindgen(js_name = get_base_url)]
    fn get_base_url() -> String;
    fn close_modal(modal: &str);
}


pub struct PatternInfo {
   pattern_type: PatternType,
   active: bool,
   date: DateTime<Utc>,
   active_date: DateTime<Utc>,
   status: Status,
   change: f64,
   pattern_direction: PatternDirection,
   info: (String, String, String, String)
}

pub fn pattern_info(pattern: Option<&Pattern>) -> PatternInfo  {
    
            // let max_pattern_days = env::var("MAX_PATTERN_DAYS")
            // .unwrap()
            // .parse::<i64>()
            // .unwrap();
   
            let max_pattern_days = 3;
            
            let pattern_direction = match pattern {
                //Some(val) => val.active.pattern_direction.clone(),
                Some(val) => val.direction.clone(),
                None   => PatternDirection::None,
            };
            
            let pattern_type = match pattern {
                Some(val) => val.pattern_type.clone(),
                None   => PatternType::None,
            };

            let active = match pattern {
                Some(pattern) => pattern.active.active,
                None   => false 
            };

            let date = match pattern {
                Some(val) => val.date.to_chrono(),
                None   => DateTime::from(Local::now() - Duration::days(1000))
            };

            let active_date = match pattern {
                Some(val) => val.active.date.to_chrono(),
                None   => DateTime::from(Local::now() - Duration::days(1000))
            };
            
            let change = match pattern {
                Some(val) => round(val.active.change,0),
                None   => 0.,
            };

            let status = match pattern {
                Some(val) => val.active.status.clone(),
                None   => Status::Default 
            };

            let activate_string = match active_date {
                _x if active_date < Local::now() - Duration::days(max_pattern_days) => "".to_string(),
                _ =>  active_date.format("%d/%m/%y").to_string()
            };

            let info: (String, String, String, String) = match date {
                _x if active_date < Local::now() - Duration::days(max_pattern_days) => (pattern_type.to_string(), pattern_direction.to_string(), [change.to_string(),"%".to_string()].concat(), activate_string), 
                _x if date < Local::now() - Duration::days(max_pattern_days) => (pattern_type.to_string(),"".to_string(),"".to_string(),"".to_string()),
                _x if status == Status::Bullish => (pattern_type.to_string(), pattern_direction.to_string(), [change.to_string(),"%".to_string()].concat(), activate_string),
                _x if status == Status::Neutral => (pattern_type.to_string(), pattern_direction.to_string(), [change.to_string(),"%".to_string()].concat(), activate_string),
                _x if status == Status::Bearish => (pattern_type.to_string(), pattern_direction.to_string(), [change.to_string(),"%".to_string()].concat(), activate_string),
                _x if status == Status::ChangeUp || status == Status::ChangeDown => (pattern_type.to_string(), pattern_direction.to_string(), [change.to_string(),"%".to_string()].concat(), ("").to_string()),
                _ => ("".to_string(),"".to_string(),"".to_string(),"".to_string()),
            };

            PatternInfo{
                pattern_type,
                active,
                date,
                active_date,
                status,
                change,
                pattern_direction,
                info
            }
        }


#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub instruments: Vec<CompactInstrument>,
    pub on_symbol_click: Callback<String>,
    pub on_watch_click: Callback<CompactInstrument>,
}

#[function_component(InstrumentsList)]
pub fn instrument_list(props: &Props
) -> Html {
    let Props { instruments, on_symbol_click, on_watch_click } = props;
    let base_url = get_base_url();
    let url = [base_url.as_str(), "api/instruments/chart/"].concat();

    let instrument_list: Html = instruments
        .iter()
        .map(|instrument| {

            let on_instrument_select = {
                let on_symbol_click = on_symbol_click.clone();
                let symbol = instrument.symbol.clone();
                let url = [url.clone(),symbol].concat();
                Callback::from(move |_| on_symbol_click.emit(url.clone()))
            };

            let on_watch_select = {
                let instrument = instrument.clone();
                let on_watch_click = on_watch_click.clone();
                Callback::from(move |_| on_watch_click.emit(instrument.clone()))
            };
           
            let local_pattern = pattern_info(instrument.patterns.local_patterns.last()); 

            let macd = instrument.indicators.macd.clone();
            let stoch = instrument.indicators.stoch.clone();
            let rsi = instrument.indicators.rsi.clone();
            let bb = instrument.indicators.bb.clone(); //9
            let date = instrument.date.to_chrono();

            let candle_status = get_candle_status(&instrument.current_candle);

            // let ema_style: (&str, &str, &str) = match bb.status {
            //    Status::Bullish => ("has-text-primary","has-text-primary","has-text-primary"), 
            //    Status::Neutral=> ("has-text-warning","has-text-primary","has-text-primary"),
            //    Status::Bearish => ("has-text-warning","has-text-primary","has-text-primary"),
            //    Status::Default=> ("has-text-danger","has-text-danger","has-text-dangery") 
            // };


            // let horizontal_lows: Vec<&HorizontalLevel> =  instrument.horizontal_levels.lows.iter().filter(|h| h.occurrences >= 3 && is_equal(h.price, instrument.current_price,1.5)).map(|x| x).collect();
            
            // // let horizontal_info = match horizontal_lows.get(0) {
            // //     Some(val) => val.occurrences.to_string(),
            // //     None   => "".to_string(),
            // // };
            
            // //   let horizontal_status = match horizontal_lows.get(0) {
            // //     Some(_val) => Status::Bullish,
            // //     None   => Status::Default 
            // // };
            
            let divergence_type = match instrument.divergences.data.last() {
                Some(val) => &val.divergence_type,
                None   => &DivergenceType::None
            }; 

            let divergence_status = get_divergence_status(divergence_type);

            let divergence_str = match divergence_type {
                DivergenceType::Bullish => divergence_type.to_string(),
                DivergenceType::Bearish => divergence_type.to_string(),
                DivergenceType::None   => "".to_owned() 
            };
            
            let band_direction_status = match local_pattern.pattern_direction {
                PatternDirection::Top => Status::Default,
                PatternDirection::Bottom => Status::Default,
                _ => Status::Default,
            };
          
            let price_display = round(price_change(instrument.prev_price, instrument.current_price),2);

            let price_change_status = get_price_change_status(price_display);

            let bb_size = percentage_change(instrument.indicators.bb.current_b, instrument.indicators.bb.current_a);
            let bb_width = (instrument.indicators.bb.current_a - instrument.indicators.bb.current_b) / instrument.indicators.bb.current_c;

            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td class={get_status_class(&price_change_status)}> {format!("{}%", price_display)}</td>
                    <td class={get_status_class(&candle_status)}> {format!("{:?}", instrument.current_candle)}</td>
                    <td class={get_status_class(&local_pattern.status)}> {local_pattern.info.0}</td>
                    <td class={get_status_class(&band_direction_status)}> {local_pattern.info.1}</td>
                    <td> {local_pattern.info.2}</td>
                    <td> {local_pattern.info.3}</td>
                    <td class={get_status_class(&bb.status)}> {format!("{} / {}%", round(bb_width,2), round(bb_size, 1))}</td>
                    //<td class={get_status_class(&macd.status)}>{format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 1), round(instrument.indicators.macd.current_b, 1))}</td>
                    <td class={get_status_class(&rsi.status)}>  {format!("{:?}", round(instrument.indicators.rsi.current_a, 1))}</td>
                    <td class={get_status_class(&divergence_status)}> {divergence_str}</td>
                    <td> {format!("{}", date.format("%d/%m %H:%M"))}</td>
                    <td  onclick={ on_watch_select }><a href={"javascript:void(0);"}>{ "x" }</a></td>
                </tr>
            }
        })
        .collect();


    let table = html! {
        <table class="table is-bordered">
            <thead class="has-background-grey-lighter">
                <tr>
                <th>{ "Symbol" }</th>
                <th>{ "Price" }</th>
                <th>{ "Chg%" }</th>
                <th>{ "Candle" }</th>
                <th>{ "Pattern" }</th>
                <th>{ "Band" }</th>
                <th>{ "Target" }</th>
                <th>{ "Activated" }</th>
                <th>{ "B.Bands" }</th>
               // <th>{ "Stoch" }</th>
                <th>{ "Rsi" }</th>
                <th>{ "Divergence" }</th>
                <th>{ "Updated" }</th>
                <th>{ "W" }</th>
                </tr>
            </thead>
            <tbody>
                { instrument_list }
            </tbody>
        </table>
    };

           table
}
