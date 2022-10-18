use utils::note_node;

use crate::note::span;

use super::block;

#[note_node(block)]
pub struct Code {
    pub content: Vec<span::Span>,
    pub language: String,
    pub options: Option<CodeOptions>, // default = false
}

#[note_node]
pub struct CodeOptions {
    pub word_wrap: bool,
    pub line_number: bool,
    pub line_number_start: Option<u32>,
}

#[note_node(block)]
pub struct Math {
    pub expression: String, // TODO: styles in math?
}
