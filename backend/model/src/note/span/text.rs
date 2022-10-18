use url::Url;
use utils::{into_enum, note_node};

use crate::note::{style, PageId, block};

use super::{span, Span};

#[note_node(span)]
pub struct Text {
    pub value: String,
}

#[note_node(span)]
pub struct Styled {
    pub content: Vec<Span>,
    pub styles: Vec<StyleModifier>,
}

#[note_node(span)]
pub struct TinyStyled {
    pub content: Vec<Span>,
    pub styles: Vec<TinyStyleModifier>,
}

#[note_node]
#[non_exhaustive]
pub enum StyleModifier {
    Bold,
    Italic,
    Underlined,
    StrikeThrough(Option<style::Fill>),
    Decorate(DecorateStyle),

    Fill(style::Fill),
    Highlight(style::Fill),

    Link(LinkTarget),
}

#[note_node]
#[non_exhaustive]
#[into_enum(StyleModifier)]
pub enum TinyStyleModifier {
    Bold,
    Italic,
    Underlined,
    StrikeThrough(Option<style::Fill>),
    Decorate(DecorateStyle),

    Link(LinkTarget),
}

#[note_node]
pub struct DecorateStyle {
    box_type: DecorateType,
}

#[note_node]
pub enum DecorateType {
    Square,
    RoundSquare,
    Circle,
    Squircle,
}

#[note_node]
pub enum LinkTarget {
    Web(Url),
    Page(PageLink),
    Block(BlockLink),
}

#[note_node]
pub struct PageLink {
    pub page_id: PageId,
    pub block_id: block::BlockId,
}

#[note_node]
pub struct BlockLink {
    pub block_id: block::BlockId,
}
