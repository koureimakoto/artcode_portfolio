pub use props::Data as Props;
pub use props::SingleData as SingleProps;
pub use component::MenuCodeStylized as MenuCodeStylized;
use yew::platform::fmt::buffer;
use yew::{Html, html};

pub mod component;
pub mod props;
pub mod rust_layout;


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
    let mut buffer = String::from(" ");
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