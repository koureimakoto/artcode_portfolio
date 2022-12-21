pub use props::Data                 as Props;
pub use props::SingleData           as SingleProps;
pub use component::MenuCodeStylized as MenuCodeStylized;

use yew::{
    Html,
    html,
    Classes
};

use yew_router::prelude::Link;
use crate::pages::pages_paths::CodeRoute;

pub mod component;
pub mod props;
pub mod jsx_layout;
pub mod rust_layout;

#[cfg(test)]
pub mod tests;

pub fn
add_line(content: &str) -> Html {
    html! {
        <div>
            <span>
                {content}
            </span>
        </div>
    }
}

pub fn 
add_line_array(contents: Vec<&str>) -> Html {
    html! {
        <>
            {contents.iter().map(|el| {add_line(el)}).collect::<Html>()}
        </>
    }
}

pub fn 
count_space( size: usize, max_size: usize) -> String{
    let mut size = size;
    let mut buffer = String::from("");
    while size < max_size {
        size += 1;
        buffer.push(' ');
    };

    buffer
}

pub fn
get_greatest_word(words: &Vec<SingleProps>) -> usize {
    let mut greatest: usize = 0;
    words.iter().for_each(|el| {
        let word_len = el.name.len();
        if word_len > greatest {
            greatest = word_len; 
        }
    });

    greatest
}

#[inline]
fn prepare_link_menu(route: CodeRoute, link_classes: Classes, content: Html) -> Html {
    html!{
        <>
        
        <Link<CodeRoute> classes={link_classes} to={route.clone()}>
            {content}
        </Link<CodeRoute>>
        
        </>
    }
}
