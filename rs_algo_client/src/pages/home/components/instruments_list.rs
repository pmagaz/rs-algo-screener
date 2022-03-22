use round::round;
use rs_algo_shared::models::{CompactInstrument, PatternType};
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
            
            let pattern_change = match local_pattern {
                Some(val) => round(val.active.change, 2),
                None   => 0.,
            };

            html! {
                <tr>
                    <td> <a href={format!("{}{}", base_url, instrument.symbol)}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td> {format!("{:?}", instrument.current_candle)}</td>
                    <td> {format!("{:?}", pattern_type)}</td>
                    <td> {format!("{:?}%", pattern_change)}</td>
                    <td> {format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 2), round(instrument.indicators.macd.current_b, 2))}</td>
                    <td> {format!("{:?} / {:?}", round(instrument.indicators.stoch.current_a, 2), round(instrument.indicators.stoch.current_b, 2))}</td>
                    <td> {format!("{:?}", instrument.date)}</td>
                </tr>
            }
        })
        .collect()
}
