use utils::note_node;

use super::{span, Span};

#[note_node(span)]
pub struct Code {
    pub content: Vec<Span>,
    pub language: Option<String>, // inline highlighting is available!
}

#[note_node(span)]
pub struct Math {
    pub expression: String,
}
