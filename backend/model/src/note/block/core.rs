use utils::note_node;

use crate::note::{span, style};

use super::{block, Block};

#[note_node(block)]
pub struct Tree {
    pub blocks: Vec<Block>,
    pub style: Option<style::BoxStyle>,
}

#[note_node]
pub struct Content {
    pub spans: Vec<span::Span>,
}

#[note_node(block)]
pub struct Empty {}

#[note_node(block)]
pub struct Text {
    pub content: Content,
}

#[note_node(block)]
pub struct Indent {
    pub content: Vec<Block>,
    pub level: u8,
}
