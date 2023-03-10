use yew::{
    Html,
    html,
    function_component
};

use super::rust_layout::RustCodeLayout;
use super::jsx_layout::JsxCodeLayout;
use super::SingleProps;

use crate::pages::pages_paths::CodeRoute;


#[function_component(MenuCodeStylized)]
pub fn component() -> Html {
    let data = vec![
        SingleProps {
            link_class_name: String::from("web"),
            span_class_name: String::from("linkCode web"),
            name: String::from("web"),
            route: CodeRoute::Web
        },
        SingleProps {
            link_class_name: String::from("games"),
            span_class_name: String::from("linkCode games"),
            name: String::from("games"),
            route: CodeRoute::Games
        },
        SingleProps {
            link_class_name: String::from("layout"),
            span_class_name: String::from("linkCode layout"),
            name: String::from("layout"),
            route: CodeRoute::Games
        }
    ];

    html! {
        <>
        <RustCodeLayout data={data}/>
        </>
    }
}