use round::round;
use rs_algo_shared::helpers::date::{Local, DateTime,Datelike};
use rs_algo_shared::models::{CompactInstrument, PatternType, PatternDirection};
use yew::{function_component, html, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct InstrumentsProps {
    pub instruments: Vec<CompactInstrument>,
    //on_click: Callback<CompactInstrument>,
}

#[function_component(InstrumentsList)]
pub fn instrument_list(
    InstrumentsProps {
        instruments,
        // on_click,
    }: &InstrumentsProps,
) -> Html {
    //let on_click = on_click.clone();
    let base_url = "http://192.168.1.10/api/instruments?symbol=";

    instruments
        .iter()
        .map(|instrument| {
            // let on_instrument_select = {
            //     //let on_click = on_click.clone();
            //     let instrument = instrument.clone();
            //     //Callback::from(move |_| on_click.emit(instrument.clone()))
            // };

            let local_pattern = instrument.patterns.local_patterns.get(0); 
            let pattern_type = match local_pattern {
                Some(val) => val.pattern_type.clone(),
                None   => PatternType::None,
            };

            let break_direction = match local_pattern {
                Some(val) => val.active.break_direction.clone(),
                None   => PatternDirection::None,
            };

            let pattern_date = match local_pattern {
                Some(val) => val.active.date.to_chrono(),
                None   => DateTime::from(Local::now())
            };
            
            let pattern_change = match local_pattern {
                Some(val) => round(val.active.change, 2),
                None   => 0.,
            };

            let date = instrument.date.to_chrono();

            html! {
                <tr>
                    <td> <a href={format!("{}{}", base_url, instrument.symbol)}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td> {format!("{:?}", instrument.current_candle)}</td>
                    <td> {format!("{:?}", pattern_type)}</td>
                    <td> {format!("{:?}", break_direction)}</td>
                    <td> {format!("{:?}%", pattern_change)}</td>
                    <td> {format!("{:?}/{:?}/{:?}", pattern_date.day(), pattern_date.month(), pattern_date.year())}</td>
                    <td> {format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 2), round(instrument.indicators.macd.current_b, 2))}</td>
                    <td> {format!("{:?} / {:?}", round(instrument.indicators.stoch.current_a, 2), round(instrument.indicators.stoch.current_b, 2))}</td>
                    <td> {format!("{:?}", round(instrument.indicators.rsi.current_a, 2))}</td>
                    <td> {format!("{:?}", date)}</td>
                </tr>
            }
        })
        .collect()
}
