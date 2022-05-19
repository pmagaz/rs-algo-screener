use yew::{function_component, html, Properties};
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

#[function_component(Chart)]
pub fn chart(props: &Props) -> Html {
    let Props { url } = props;
    log::info!("[CLIENT] Chart {:?}", &props);
    html! {

      <div class="modal" id="modal">
      <div class="modal-background" id="modal-background"></div>
      <div class="modal-content">
          <img src={format!("{}", url)} width="1024" height="768"/>
      </div>
      <button class="modal-close is-large" aria-label="close" onClick={"javascript:close_modal();"}></button>
    </div>
        }
}
