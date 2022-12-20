use yew::{Html, html};
use yew_router::prelude::*;

use self::{
    home::home::Home, 
    error_404::NotFound,
    code::Code,
    artwork::Artwork, 
    pages_paths::{ArtworkCodeLayout, Route, CodeRoute}, 
};

pub mod home;
pub mod code;
pub mod artwork;
pub mod error_404;

#[path ="pages_paths.rs"]
pub mod pages_paths;


pub fn
switch(routes: Route) -> Html {
    html! {{
        match &routes {
            Route::Home     => html! {<Home    />},
            Route::Layout   => html! {
                <Switch<ArtworkCodeLayout> render={switch_layout} />
            },
            Route::NotFound => html! {<NotFound/>}
    }}}
}

fn
switch_layout(route: ArtworkCodeLayout) -> Html {
    html! {
        <>
        <h1>{"Oi dois"}</h1>
        {
        match route {
            ArtworkCodeLayout::CodeRoot    => html! {<Code   />},
            ArtworkCodeLayout::Code        => html! {
                <Switch<CodeRoute> render={switch_code_layout} />
            },
            ArtworkCodeLayout::ArtworkRoot => html! {<Artwork/>},
            ArtworkCodeLayout::Artwork     => html! {<h1>{"ArtworkLayout"}</h1>},
            ArtworkCodeLayout::NotFound    => html! {<NotFound/>}
        }}
        </>
    }
}

fn
switch_code_layout(route: CodeRoute) -> Html {
    html! {
        {
            match route {
                CodeRoute::Web      =>  html! {<h1>{"Web"}</h1>},
                CodeRoute::Games    =>  html! {<h1>{"Games"}</h1>},
                CodeRoute::NotFound =>  html! {<h1>{"NotFound"}</h1>}
            }
        }
    }
}