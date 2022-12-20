use yew::Properties;

use crate::pages::pages_paths::CodeRoute;

#[derive(Clone, PartialEq, Properties)]
pub struct SingleData {
    pub link_class_name: String,
    pub span_class_name: String,
    pub name           : String,
    pub route          : CodeRoute
}

#[derive(Clone, PartialEq, Properties)]

pub struct Data {
    pub data: Vec<SingleData> 
}