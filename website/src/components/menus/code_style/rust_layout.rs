use yew::{
    Html,
    html, 
    Classes,
    classes,
    function_component
};

use yew_router::prelude::Link;

use crate::pages::pages_paths::CodeRoute;
use super::{
    Props,
    add_line,
    count_space,
    add_line_array,
    get_greatest_word
};



#[inline]
fn prepare_link_menu_content(name: &String, span_class: &String, space_string_len: usize) -> Html {
    html! {
    <>
        <span>{"        link::"}</span>
        <span class={span_class}>
            {name}{count_space(name.len(), space_string_len)}
        </span>
        <span>{"=> _,"}</span>
    </>
    }
}

#[inline]
fn prepare_link_menu(route: CodeRoute, link_classes: Classes, content: Html) -> Html {
    html!{
        <Link<CodeRoute> classes={link_classes} to={route.clone()}>
        // Link Content format: [8-whitespace]link::$name => _,
            {content}
        </Link<CodeRoute>>
    }
}

#[inline]
/// This function formats the menu link with rust syntax style.
fn prepare_menu(props: &Props) -> Html {
    let space_string_len: usize = get_greatest_word(&props.data);

    html! {
        {
        props.data.iter().map(|el| {
            let link_classes = classes!(&el.link_class_name, "mainCodeMenu");
            let content      = prepare_link_menu_content(
                &el.name,
                &el.span_class_name,
                space_string_len
            );

            html!{
            <div> 
                {prepare_link_menu(el.route.clone(), link_classes, content)}
            </div>
        }}).collect::<Html>()
        }
    }
}

#[function_component(RustCodeLayout)]
pub fn rust_layout(props: &Props) -> Html {
    html!{
        <div class={"specialCodeArea rs"}>
            {add_line_array(vec![
                "pub fn",
                "code_menu(_) {",
                "    match _ {"
            ])}
            {prepare_menu(props)}
            {add_line("    }")}
            {add_line("}")}
        </div>
    }
}