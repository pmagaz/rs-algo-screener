use crate::routes::Route;
use yew::{classes, function_component, html};
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    //let Self { navbar_active, .. } = *self;
    let navbar_active = false;
    let active_class = if !navbar_active { "is-active" } else { "" };

    html! {
    <nav class="navbar is-link" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
          <h1 class="navbar-item is-size-3">{ "RS SCREENER" }</h1>
        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div id="navbarBasicExample" class="navbar-menu">
        <div class="navbar-start">
          <a class="navbar-item is-hoverable">
          <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
            { "Screener" }
            </Link<Route>>
          </a>

          <a class="navbar-item">
        <Link<Route> classes={classes!("navbar-item")} to={Route::Strategies}>
            { "Strategies" }
            </Link<Route>>
          </a>

          // <div class="navbar-item has-dropdown is-hoverable">
          //   <a class="navbar-link">
          //     { "More" }
          //   </a>

          //   <div class="navbar-dropdown">
          //     <a class="navbar-item">
          //       { "More" }
          //     </a>
          //     <a class="navbar-item">
          //       { "More" }
          //     </a>
          //     <a class="navbar-item">
          //       { "More" }
          //     </a>
          //   </div>
          // </div>
        </div>

      </div>
    </nav>
        }
}
