use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

#[function_component(Plotter)]
pub fn plotter(props: &Props) -> Html {
    let Props { url } = props;
    log::info!("[CLIENT] Plotter {:?}", &props);
    html! {
      <img src={format!("{}", url)} />
    }
}
