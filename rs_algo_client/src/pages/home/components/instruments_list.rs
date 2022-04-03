use round::{round};
use rs_algo_shared::models::*;
use rs_algo_shared::helpers::date::{Local, DateTime,Datelike, Duration};
use rs_algo_shared::helpers::comp::is_equal;
use yew::{function_component, html, Callback, use_state, Properties};
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn open_modal(modal: &str);
    fn close_modal(modal: &str);
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
            

            let pattern = instrument.patterns.local_patterns.last(); 


            let break_direction = match pattern {
                Some(val) => val.active.break_direction.clone(),
                None   => PatternDirection::None,
            };
            
            let pattern_type = match pattern {
                Some(val) => val.pattern_type.clone(),
                None   => PatternType::None,
            };

            let candle_status = match instrument.current_candle {
                CandleType::Karakasa => Status::Bullish,
                CandleType::MorningStar => Status::Bullish,
                CandleType::BullishGap => Status::Bullish,
                CandleType::BearishKarakasa => Status::Bearish,
                CandleType::BearishGap => Status::Bearish,
                CandleType::BearishStar => Status::Bearish,
                _ => Status::Default,
            };

            let pattern_active = match pattern {
                Some(pattern) => pattern.active.active,
                None   => false 
            };


            let pattern_date = match pattern {
                Some(val) => val.date.to_chrono(),
                None   => DateTime::from(Local::now() - Duration::days(1000))
            };

            let pattern_active_date = match pattern {
                Some(val) => val.active.date.to_chrono(),
                None   => DateTime::from(Local::now() - Duration::days(1000))
            };
            
            let pattern_change = match pattern {
                Some(val) => round(val.active.change,0),
                None   => 0.,
            };

            let pattern_status = match pattern {
                _x if pattern_active && pattern_active_date > DateTime::<Local>::from(Local::now() - Duration::days(5)) => Status::Bullish,
                _x if pattern_active && pattern_active_date > DateTime::<Local>::from(Local::now() - Duration::days(10)) => Status::Neutral,
                _x if pattern_date > DateTime::<Local>::from(Local::now() - Duration::days(20)) => Status::Neutral,
                _x if pattern_date > DateTime::<Local>::from(Local::now() - Duration::days(30)) => Status::Default,
                _x if pattern_type == PatternType::None => Status::Default,
                 _  => Status::Default,
            };


            let macd = instrument.indicators.macd.clone();
            let stoch = instrument.indicators.stoch.clone();
            let rsi = instrument.indicators.rsi.clone();
            let ema_a = instrument.indicators.ema_a.clone(); //9
            // let _ema_b = instrument.indicators.ema_b.clone(); //21
            // let _ema_c = instrument.indicators.ema_c.clone(); //50

            let date = instrument.date.to_chrono();

             let pattern_info: (String, String, String, String) = match pattern_date {
                _x if pattern_type == PatternType::None  => ("".to_string(),"".to_string(),"".to_string(),"".to_string()),
                _x if pattern_status == Status::Bullish => (pattern_type.to_string(), break_direction.to_string(), [pattern_change.to_string(),"%".to_string()].concat(), pattern_active_date.format("%d/%m/%Y").to_string()),
                _x if pattern_status == Status::Neutral || pattern_status == Status::Default => (pattern_type.to_string(), break_direction.to_string(), [pattern_change.to_string(),"%".to_string()].concat(), "".to_string()),
                _ => ("".to_string(),"".to_string(),"".to_string(),"".to_string()),
            };

            let ema_style: (&str, &str, &str) = match ema_a.status {
               Status::Bullish => ("has-text-primary","has-text-primary","has-text-primary"), 
               Status::Neutral=> ("has-text-warning","has-text-primary","has-text-primary"),
               Status::Bearish => ("has-text-warning","has-text-primary","has-text-primary"),
               Status::Default=> ("has-text-danger","has-text-danger","has-text-dangery") 
            };


            let horizontal_lows: Vec<&HorizontalLevel> =  instrument.horizontal_levels.lows.iter().filter(|h| h.occurrences >= 3 && is_equal(h.price, instrument.current_price)).map(|x| x).collect();
            
            let horizontal_info = match horizontal_lows.get(0) {
                Some(val) => val.occurrences.to_string(),
                None   => "".to_string(),
            };
            
              let horizontal_status = match horizontal_lows.get(0) {
                Some(_val) => Status::Bullish,
                None   => Status::Default 
            }; 
            
            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td class={get_status_class(&candle_status)}> {format!("{:?}", instrument.current_candle)}</td>
                    <td class={get_status_class(&pattern_status)}> {format!("{}", pattern_info.0)}</td>
                    <td class={get_status_class(&pattern_status)}> {format!("{}", pattern_info.2)}</td>
                    <td class={get_status_class(&pattern_status)}> {format!("{}", pattern_info.3)}</td>
                    <td class={get_status_class(&horizontal_status)}>{format!("{}", horizontal_info)}</td>
                    <td class={get_status_class(&stoch.status)}> {format!("{:?} / {:?}", round(instrument.indicators.stoch.current_a, 1), round(instrument.indicators.stoch.current_b, 1))}</td>
                    <td class={get_status_class(&macd.status)}>{format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 1), round(instrument.indicators.macd.current_b, 1))}</td>
                    <td class={get_status_class(&rsi.status)}>  {format!("{:?}", round(instrument.indicators.rsi.current_a, 1))}</td>
                    // <td class={get_status_class(&ema_a.status)}>
                    //     <span class={ema_style.0}> {round(instrument.indicators.ema_a.current_a, 1)} </span>
                        
                    //     <span class={ema_style.1}> {round(instrument.indicators.ema_b.current_a, 1)} </span>

                    //     <span class={ema_style.2}> {round(instrument.indicators.ema_c.current_a, 1)}</span>
                    // </td>
                    <td class={get_status_class(&ema_a.status)}> {format!("{:?} / {:?} / {:?}", round(instrument.indicators.ema_a.current_a, 1), round(instrument.indicators.ema_b.current_a, 1), round(instrument.indicators.ema_c.current_a, 1))}</td>
                    <td> {format!("{}", date.format("%R"))}</td>
                </tr>
            }
        })
        .collect()
}
