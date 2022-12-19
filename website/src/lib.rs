use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    pages_paths::Route,
    switch
};

pub mod pages;
pub mod components;

#[function_component(App)]
#[allow(non_snake_case)]
pub fn
Run() -> Html {
    // Mesmo conceito do aplicado no jsx
    html! {
    <>
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
        <h1>{"Evitando Trunk"}</h1>
    </>
    }
}
