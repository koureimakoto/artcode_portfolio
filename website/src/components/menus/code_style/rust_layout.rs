use yew::{
    Html,
    html, 
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

/// This function formats the menu link with rust syntax style.
fn prepare_menu(props: &Props, greatest: usize) -> Html {


    html! {
        {
        props.data.iter().map(|el| {
            let link_classes = classes!(&el.link_class_name, "mainCodeMenu");
            let span_class   = &el.span_class_name;
            let content      = html! {
                <>{&el.name}{count_space(el.name.len(), greatest)}</>
            };

            html!{
            <div> 
            <Link<CodeRoute> 
                classes={link_classes}
                to     ={el.route.clone()}
            >   // Link Content format: [8-whitespace]link::$name => _,
                <span>{"        link::"}</span>
                <span class={span_class}>
                    {content}
                </span>
                <span>{"=> _,"}</span>   
            </Link<CodeRoute>>
            </div>
        }}).collect::<Html>() // Collect Html format and return
        }
    }
}

#[function_component(RustCodeLayout)]
pub fn rust_layout(props: &Props) -> Html {
    let greatest: usize = get_greatest_word(&props.data);

    html!{
        <div class={"specialCodeArea rs"}>
        {add_line_array(vec![
            "pub fn",
            "code_menu(_) {",
            "    match _ {"
        ])}
        {prepare_menu(props, greatest)}
        {add_line("    }")}
        {add_line("}")}
        </div>
    }
}