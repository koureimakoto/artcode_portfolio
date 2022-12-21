use yew::{
    Html,
    html, 
    classes,
    function_component
};


use super::{
    Props,
    count_space,
    add_line_array,
    get_greatest_word,
    prepare_link_menu
};

#[inline]
fn prepare_link_menu_content(name: &String, span_class: &String, space_string_len: usize) -> Html {
    html! {
    <>
        <span>{"        <Route _=\"{"}</span>
        // Link Content format: [8-whitespace]link::$name => _,
        <span class={span_class}>
            {" < "}{name}{count_space(name.len(), space_string_len)}{"/>"}
        </span>
        <span>{"} />"}</span>
    </>
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

#[function_component(JsxCodeLayout)]
pub fn rust_layout(props: &Props) -> Html {
    html!{
        <div class={"specialCodeArea ts"}>
            {add_line_array(vec![
                "export function",
                "codeMenu(_) {",
                "    return (",
                "    <Routes>"
            ])}
            {prepare_menu(props)}
            {add_line_array(vec![
                "    </Routes>",
                "    )",
                "}",
            ])}
        </div>
    }
}