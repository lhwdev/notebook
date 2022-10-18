use utils::note_node;

use crate::note::{span, style};

use super::{
    block, block_kind,
    core::{Content, Tree},
    Block,
};

// Markdown blocks

#[note_node(block)]
pub struct Heading {
    pub content: Content,
    pub level: u8,
}

#[note_node(block_kind)]
pub enum List {
    Bulleted(BulletedList),
    Numbered(NumberedList),
}

#[note_node(block)]
pub struct BulletedList {
    pub items: Vec<Block>,
    pub bullet: Option<span::Span>,
}

#[note_node(block)]
pub struct NumberedList {
    pub items: Vec<Block>,
    pub format: NumberFormat,
    pub starts_with: Option<u32>,
}

#[note_node]
pub enum NumberFormat {
    Arabic,       // 1, 2, 3
    Roman,        // I, II, III
    Alphabet,     // a, b, c
    Repeat(char), // i, ii, iii / x, xx, xxx
}

#[note_node(block)]
pub struct Todo {
    pub content: Tree,
    pub checked: bool,
}

#[note_node(block)]
pub struct Divider {
    pub fill: Option<style::Fill>
}

#[note_node(block)]
pub struct Quote {
    pub content: Tree,
}

// Other blocks

#[note_node(block)]
pub struct Callout {
    pub content: Tree,
    pub trailing_icon: style::Icon,
}

#[note_node(block)]
pub struct Toggle {
    pub content: Tree,
    pub expanded: bool,
}
