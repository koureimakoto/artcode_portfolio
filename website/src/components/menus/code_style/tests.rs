use wasm_bindgen_test::*;
use yew::{html, virtual_dom::VNode};

use super::{
    add_line,
    count_space,
    get_greatest_word,
    SingleProps
};

use crate::pages::pages_paths::CodeRoute::{
    Web,
    Games,
    NotFound
};

// To enable testing by browser, for example: wasm-pack test --headless --firefox --node [chrome, saffari]
// For more information acess: https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html
wasm_bindgen_test_configure!(run_in_browser);

fn single_props_vector_fillled<'a>(greatest: &'a str) -> Vec<SingleProps>{
    vec![
        SingleProps{
            link_class_name: "web".to_string(),
            span_class_name: "none".to_string(),
            name : "web_name".to_string(),
            route: Web
        }, 
        SingleProps{
            link_class_name: "web".to_string(),
            span_class_name: "none".to_string(),
            name : "game_name".to_string(),
            route: Games
        },
        SingleProps{
            link_class_name: "web".to_string(),
            span_class_name: "none".to_string(),
            name : greatest.to_string(),
            route: NotFound
        }
    ]
}

#[wasm_bindgen_test]
fn greatest_word() {
    let greatest         : &str             = "Makoto World";
    let fake_single_props: Vec<SingleProps> = single_props_vector_fillled(&greatest);

    assert_eq!(
        get_greatest_word(&fake_single_props),
        greatest.len()
    );

    assert_ne!(
        get_greatest_word(&fake_single_props),
        greatest.len() - 1
    )

}

#[wasm_bindgen_test]
fn whitespace_necessary_to_reach_the_largest_word() {
    let max_size = 10;
    let two_counted_whitespaces  = count_space(8usize, max_size);
    let four_counted_whitespaces = count_space(6usize, max_size);

    let two_manual_whitespaces  = "  ".to_string();
    let four_manual_whitespaces = "    ".to_string();

    assert_eq!(two_counted_whitespaces , two_manual_whitespaces );
    assert_eq!(four_counted_whitespaces, four_manual_whitespaces);
}

#[wasm_bindgen_test]
fn add_one_line() {
    let content: &str = "Content";
    let line   : VNode = add_line(&content);

    let fake_line = html! {
        <div>
            <span>
                {content}
            </span>
        </div>
    };

    assert_eq!(line, fake_line);
}
