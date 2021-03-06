use yew::{function_component, html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub loading: bool,
}

#[function_component(Loading)]
pub fn loading(props: &Props) -> Html {
    let Props { loading } = props;
    log::info!("[CLIENT] props {:?}", &props);
    html! {
        if *loading {
          <progress class="progress is-small is-link" max="100">{ "Loading..."} </progress>
        } else {
          <div style="height: 12px;"></div>
        }
    }
}
