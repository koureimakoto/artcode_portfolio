// Yew standards
use yew::prelude::*;

// Intern
use crate::pages::pages_paths::ArtworkCodeLayout;

// Home Button Props
// I will maybe remove the callback
#[derive(Clone, PartialEq, Properties)]
pub struct Data {
    pub class_name: String,
    pub name      : String,
    pub route     : ArtworkCodeLayout
}