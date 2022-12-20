use yew::{Html, html, function_component};

use crate::{
    components::buttons::{
        home::Props,
        home::HomeButton,
    },
    components::menus::{
        code_style::Props as CodeStyleProps,
        code_style::MenuCodeStylized
    },

    pages::ArtworkCodeLayout::{
        CodeRoot, 
        ArtworkRoot
    }
};

#[function_component(Home)]
pub fn get() -> Html {

    let lhs = Props {
        class_name: String::from("codes"),
        name      : String::from("Code"),
        route     : CodeRoot,
        callback  : ||{}
    };

    let rhs = Props {
        class_name: String::from("artworks"),
        name      : String::from("Artwork"),
        route     : ArtworkRoot,
        callback  : ||{}
    };


    html! {
        <>
        <MenuCodeStylized/>
        <nav class={String::from("homeMenu")} >
            // Code Side
            <HomeButton
                class_name={lhs.class_name}
                name={lhs.name}
                route={lhs.route} 
                callback = {lhs.callback}
            />
            // Artwork Site
            <HomeButton
                class_name={rhs.class_name}
                name={rhs.name}
                route={rhs.route} 
                callback = {rhs.callback}
            />
        </nav>
        </>
    }
}
