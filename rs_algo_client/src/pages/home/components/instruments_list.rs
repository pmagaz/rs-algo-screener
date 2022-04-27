use round::{round};
use rs_algo_shared::models::*;
use rs_algo_shared::helpers::date::{Local, DateTime, Utc, Duration};
use rs_algo_shared::helpers::comp::{percentage_change, price_change, is_equal};
use yew::{function_component, html, Callback, use_state, Properties, Html};
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
   break_direction: PatternDirection,
   info: (String, String, String, String)
}

pub fn pattern_info(pattern: Option<&Pattern>) -> PatternInfo  {
        
            let break_direction = match pattern {
                Some(val) => val.active.break_direction.clone(),
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

            let info: (String, String, String, String) = match date {
                _x if pattern_type == PatternType::None  => ("".to_string(),"".to_string(),"".to_string(),"".to_string()),
                _x if status == Status::Bullish => (pattern_type.to_string(), break_direction.to_string(), [change.to_string(),"%".to_string()].concat(), active_date.format("%d/%m/%Y").to_string()),
                _x if status == Status::Neutral => (pattern_type.to_string(), break_direction.to_string(), [change.to_string(),"%".to_string()].concat(), ("").to_string()),
                _x if status == Status::Default =>  ("".to_string(),"".to_string(),"".to_string(),"".to_string()),
                _ => ("".to_string(),"".to_string(),"".to_string(),"".to_string()),
            };

            PatternInfo{
                pattern_type,
                active,
                date,
                active_date,
                status,
                change,
                break_direction,
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
    let url = [base_url.as_str(), "api/instruments?symbol="].concat();
    let use_url = use_state(|| String::from(""));

      
    fn get_status_class<'a>(status: &Status) -> &'a str {
        let class = match status {
            Status::Default => "", 
            //Status::Neutral => "", 
            Status::Bullish => "has-background-primary-light", 
            Status::Bearish => "has-background-danger-light", 
            Status::Neutral => "has-background-warning-light", 
        };
        class
    }


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
            //let extrema_pattern = pattern_info(instrument.patterns.extrema_patterns.last()); 
           
            let candle_status = match instrument.current_candle {
                CandleType::Karakasa => Status::Bullish,
                CandleType::MorningStar => Status::Bullish,
                CandleType::BullishGap => Status::Bullish,
                CandleType::BearishKarakasa => Status::Bearish,
                CandleType::BearishGap => Status::Bearish,
                CandleType::BearishStar => Status::Bearish,
                _ => Status::Default,
            };



            let macd = instrument.indicators.macd.clone();
            let stoch = instrument.indicators.stoch.clone();
            let rsi = instrument.indicators.rsi.clone();
            let tema_a = instrument.indicators.tema_a.clone(); //9
            let date = instrument.date.to_chrono();



            let ema_style: (&str, &str, &str) = match tema_a.status {
               Status::Bullish => ("has-text-primary","has-text-primary","has-text-primary"), 
               Status::Neutral=> ("has-text-warning","has-text-primary","has-text-primary"),
               Status::Bearish => ("has-text-warning","has-text-primary","has-text-primary"),
               Status::Default=> ("has-text-danger","has-text-danger","has-text-dangery") 
            };


            let horizontal_lows: Vec<&HorizontalLevel> =  instrument.horizontal_levels.lows.iter().filter(|h| h.occurrences >= 3 && is_equal(h.price, instrument.current_price,1.5)).map(|x| x).collect();
            
            let horizontal_info = match horizontal_lows.get(0) {
                Some(val) => val.occurrences.to_string(),
                None   => "".to_string(),
            };
            
              let horizontal_status = match horizontal_lows.get(0) {
                Some(_val) => Status::Bullish,
                None   => Status::Default 
            };
            
            let divergence_type = match instrument.divergences.data.last() {
                Some(val) => &val.divergence_type,
                None   => &DivergenceType::None
            }; 

            let divergence_status = match divergence_type {
                DivergenceType::Bullish => Status::Bullish,
                DivergenceType::Bearish => Status::Bearish,
                DivergenceType::None => Status::Default,
            };

            let divergence_str = match divergence_type {
                DivergenceType::Bullish => divergence_type.to_string(),
                DivergenceType::Bearish => divergence_type.to_string(),
                DivergenceType::None   => "".to_owned() 
            }; 
          
            let price_display = round(price_change(instrument.prev_price, instrument.current_price),2);

            let price_change_status = match price_display {
                   _x if price_display >= 0.0 => {
                    Status::Bullish
                }
                _x if price_display < 0.0 => {
                    Status::Bearish
                }
                _ => Status::Neutral 
            };


            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td class={get_status_class(&price_change_status)}> {format!("{}%", price_display)}</td>
                    <td class={get_status_class(&candle_status)}> {format!("{:?}", instrument.current_candle)}</td>
                    <td class={get_status_class(&local_pattern.status)}> {local_pattern.info.0}</td>
                    <td class={get_status_class(&local_pattern.status)}> {local_pattern.info.1}</td>
                    <td class={get_status_class(&local_pattern.status)}> {local_pattern.info.2}</td>
                    <td class={get_status_class(&local_pattern.status)}> {local_pattern.info.3}</td>
                    // <td class={get_status_class(&extrema_pattern.status)}> {format!("{}", extrema_pattern.info.0)}</td>
                    // <td class={get_status_class(&extrema_pattern.status)}> {format!("{}", extrema_pattern.info.2)}</td>
                    // <td class={get_status_class(&extrema_pattern.status)}> {format!("{}", extrema_pattern.info.3)}</td>
                    <td class={get_status_class(&stoch.status)}> {format!("{:?} / {:?}", round(instrument.indicators.stoch.current_a, 1), round(instrument.indicators.stoch.current_b, 1))}</td>
                    <td class={get_status_class(&macd.status)}>{format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 1), round(instrument.indicators.macd.current_b, 1))}</td>
                    <td class={get_status_class(&rsi.status)}>  {format!("{:?}", round(instrument.indicators.rsi.current_a, 1))}</td>
                    <td class={get_status_class(&tema_a.status)}> {format!("{:?} / {:?}", round(instrument.indicators.tema_a.current_a, 1), round(instrument.indicators.tema_b.current_a, 1))}</td>
                    <td class={get_status_class(&divergence_status)}> {divergence_str}</td>
                    <td> {format!("{}", date.format("%R"))}</td>
                    <td  onclick={ on_watch_select }><a href={"javascript:void(0);"}>{ "x" }</a></td>
                </tr>
            }
        })
        .collect();


    let table = html! {
        <table class="table is-bordered">
            <thead class="has-background-grey-lighter">
                <tr>
                <th><abbr>{ "Symbol" }</abbr></th>
                <th><abbr>{ "Price" }</abbr></th>
                <th><abbr>{ "Chg%" }</abbr></th>
                <th><abbr>{ "Candle" }</abbr></th>
                <th><abbr>{ "Pattern" }</abbr></th>
                <th><abbr>{ "Band" }</abbr></th>
                <th><abbr>{ "Target" }</abbr></th>
                <th><abbr>{ "Activated" }</abbr></th>
                // <th><abbr>{ "E. Target" }</abbr></th>
                // <th><abbr>{ "E. Activated" }</abbr></th>
                <th><abbr>{ "Stoch" }</abbr></th>
                <th><abbr>{ "MacD" }</abbr></th>
                <th><abbr>{ "Rsi" }</abbr></th>
                <th><abbr>{ "Tema (8 / 21)" }</abbr></th>
                <th><abbr>{ "Divergence" }</abbr></th>
                <th><abbr>{ "Updated" }</abbr></th>
                <th><abbr>{ "W" }</abbr></th>
                </tr>
            </thead>
            <tbody>
                { instrument_list }
            </tbody>
        </table>
    };

           table
}
