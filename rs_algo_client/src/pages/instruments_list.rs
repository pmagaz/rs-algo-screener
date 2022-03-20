#[function_component(InstrumentsList)]
fn instrument_list(
    InstrumentsProps {
        instruments,
        on_click,
    }: &InstrumentsProps,
) -> Html {
    let on_click = on_click.clone();
    log::info!("[INSTRUMENTS] Size {}", instruments.len());
    instruments
        .iter()
        .map(|instrument| {
            let on_instrument_select = {
                let on_click = on_click.clone();
                let instrument = instrument.clone();
                Callback::from(move |_| on_click.emit(instrument.clone()))
            };

            html! {
                <p onclick={on_instrument_select}>{format!("{}: {}", instrument.symbol, instrument.symbol)}</p>
            }
        })
        .collect()
}
