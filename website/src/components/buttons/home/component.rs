// Yew standards
use yew::prelude::{
    Html,
    html,
    classes,
    function_component
};
use yew_router::prelude::Link;

// Intern
use crate::pages::pages_paths::ArtworkCodeLayout;
use super::Props;

fn prepare_button_content(name: &String) -> Html {
    html! {
        <div>
            <h1>{name}</h1>
        </div>
    }
}

#[function_component(HomeButton)]
pub fn home_button(props: &Props) -> Html {
    html! {
        <Link<ArtworkCodeLayout> 
            classes={classes!(&props.class_name, " homeNavSide")}
            to={props.route.clone()}
        >
            {prepare_button_content(&props.name)}
        </Link<ArtworkCodeLayout>>
    }
}
