// Yew standards
use yew::prelude::*;
use yew_router::prelude::*;

// Intern
use crate::pages::pages_paths::ArtworkCodeLayout;

use super::props::Data;


///
/// ```
///  Component(HomeButton)
///     >> &home::Props
///     << Html<Link<ArtworkCodeLayout>>
/// ```
#[function_component(HomeButton)]
pub fn home_button(props: &Data) -> Html {
    html! {
        <Link<ArtworkCodeLayout> 
            classes={classes!(&props.class_name, " homeNavSide")}
            to={props.route.clone()}
        >
            <div>
                <h1>{&props.name}</h1>
            </div>
        </Link<ArtworkCodeLayout>>
    }
}
