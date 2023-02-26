use crate::helpers::status::*;

use rs_algo_shared::scanner::instrument::*;
use rs_algo_shared::models::status::Status;
use rs_algo_shared::scanner::divergence::*;
use rs_algo_shared::helpers::date::{Local, DateTime, Utc, Duration};
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::status::*;
use rs_algo_shared::scanner::pattern::{PatternType, Pattern, PatternDirection};

use std::env;
use yew::{function_component, html, Callback, Properties, Html};
use wasm_bindgen::prelude::*;
use round::{round};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn open_modal(modal: &str);
    #[wasm_bindgen(js_name = get_base_url)]
    fn get_base_url() -> String;
    fn close_modal(modal: &str);
}


#[derive(Clone, Debug, PartialEq)]
pub enum ListType {
    PortFolio,
    WatchList,
    Strategy,
    NewPatterns,
    Activated,
    Commodities,
    Crypto,
    forex,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    PortfolioAdd,
    PortfolioDelete,
    WatchListAdd,
    WatchListDelete,
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

pub fn pattern_info(pattern: &Option<&Pattern>) -> PatternInfo  {
    
            // let max_pattern_days = env::var("MAX_PATTERN_DAYS")
            // .unwrap()
            // .parse::<i64>()
            // .unwrap();
   
            let max_pattern_days = 5;
            
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
            
            let target = match pattern {
                Some(val) => match active {
                   true => round(val.active.target,0),
                   false => round(val.target,0),
                }
                None   => 0.,
            };

            let status = match pattern {
                Some(val) => val.active.status.clone(),
                None   => Status::Default 
            };

            let activate_string = match active {
                _x if active_date > Local::now() - Duration::days(max_pattern_days) =>  active_date.format("%d/%m/%y").to_string(),
                _ =>  "".to_string()
            };

            let pattern_direction_string = match active {
                false => "".to_string(),
                true => pattern_direction.to_string()
            };

            let target_string = match active {
                true => match active_date {
                    _x if active_date > Local::now() - Duration::days(max_pattern_days) =>  [target.to_string(),"%".to_string()].concat() , 
                    _ =>  "".to_string() 
                } 
                false => match date {
                    _x if date > Local::now() - Duration::days(max_pattern_days) =>  [target.to_string(),"%".to_string()].concat() , 
                    _ =>  "".to_string() 
                } 
            };

    
            let info: (String, String, String, String) = match date {
                _x if active_date > Local::now() - Duration::days(max_pattern_days) => (pattern_type.to_string(), pattern_direction_string, target_string, activate_string), 
                _x if date > Local::now() - Duration::days(max_pattern_days) && !active => (pattern_type.to_string(),pattern_direction.to_string(),target_string,"".to_string()),
                //_x if date < Local::now() - Duration::days(max_pattern_days) && !active => (pattern_type.to_string(),pattern_direction.to_string(),[change.to_string(),"%".to_string()].concat(),"".to_string()),
                _x if status == Status::Bullish => (pattern_type.to_string(), pattern_direction_string, target_string, activate_string),
                _x if status == Status::Neutral => (pattern_type.to_string(), pattern_direction_string, target_string, activate_string),
                _x if status == Status::Bearish => (pattern_type.to_string(), pattern_direction_string, target_string, activate_string),
                _x if status == Status::ChangeUp || status == Status::ChangeDown => (pattern_type.to_string(), pattern_direction_string, target_string, ("").to_string()),
                _ => (pattern_type.to_string(),"".to_string(),"".to_string(),"".to_string()),
            };

            PatternInfo{
                pattern_type,
                active,
                date,
                active_date,
                status,
                change: target,
                pattern_direction,
                info
            }
        }


#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub instruments: Vec<CompactInstrument>,
    pub list_type: ListType, 
    pub on_symbol_click: Callback<String>,
    pub on_action_click: Callback<(ActionType, ListType, CompactInstrument)>,
}

#[function_component(InstrumentsList)]
pub fn instrument_list(props: &Props
) -> Html {
    let Props { instruments, list_type, on_symbol_click, on_action_click } = props;
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
                let on_action_click = on_action_click.clone();
                let action = match list_type {
                    ListType::WatchList => ActionType::WatchListDelete,
                    _ => ActionType::WatchListAdd
                };

                let res = (action, list_type.clone(), instrument);
                Callback::from(move |_| on_action_click.emit(res.clone()))
            };

            let on_portfolio_select = {
                let instrument = instrument.clone();
                let on_action_click = on_action_click.clone();
                let action = match list_type {
                    ListType::PortFolio => ActionType::PortfolioDelete,
                    _ => ActionType::PortfolioAdd
                };
                let res = (action, list_type.clone(), instrument);
                Callback::from(move |_| on_action_click.emit(res.clone()))
            };
         
            let current_pattern = instrument.patterns.local_patterns.last();
            let local_pattern = pattern_info(&current_pattern); 

            let macd = instrument.indicators.macd.clone();
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
                Some(val) => {
                    if val.date.to_chrono() < Local::now() - Duration::days(25) {
                        &DivergenceType::None
                    } else {
                       &val.divergence_type
                    } } ,
                None   => &DivergenceType::None
            }; 

            let divergence_status = get_divergence_status(divergence_type);

            let target_status = get_target_status(current_pattern.unwrap().target); 

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

            let leches = instrument.symbol.split(".").collect::<Vec<_>>();

            let instrument_image = match instrument.symbol.contains(".") {
                true => {
                    let leches = instrument.symbol.split(".").collect::<Vec<_>>();
                    (true, format!("https://eodhistoricaldata.com/img/logos/{}/{}.png",leches[1],leches[0]))
                }
                _ => (false, "".to_owned())
            };

            let icon_portfolio = match list_type {
                ListType::PortFolio => "img/delete.png",
                _ => "img/add.png",
            };

            let icon_watchlist = match list_type {
                ListType::WatchList => "img/delete.png",
                _ => "img/add.png",
            };

            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>
                        if instrument_image.0 {
                        <img loading="lazy" class="instrument_icon" src={instrument_image.1} />
                    }
                        {format!("{}", instrument.symbol)}</a>
                    </td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td> {format!("{}%", price_display)}</td>
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
                    <td  onclick={ on_watch_select }><a href={"javascript:void(0);"}><img src={ icon_watchlist } class="action_icon" /></a></td>
                    <td  onclick={ on_portfolio_select }><a href={"javascript:void(0);"}><img src={ icon_portfolio } class="action_icon" /></a></td>
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
                <th>{ "P" }</th>
                </tr>
            </thead>
            <tbody>
                { instrument_list }
            </tbody>
        </table>
    };

           table
}
