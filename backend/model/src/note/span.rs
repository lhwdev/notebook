use url::Url;
use utils::{inherit_enum, note_node};

use super::{block, style, PageId};

macro_rules! span {
    ($item:item) => {
        $item
    };
}

#[note_node(span)]
pub enum Span {}

#[note_node(span)]
pub struct Text {
    pub value: String,
    pub styles: Vec<StyleModifier>,
}

#[note_node(span)]
pub struct TinyText {
    pub value: String,
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

    Link(Link),
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
}

#[note_node]
#[non_exhaustive]
#[inherit_enum(StyleModifier)]
pub enum TinyStyleModifier {
    Bold,
    Italic,
    Underlined,
    StrikeThrough(Option<style::Fill>),
    Decorate(DecorateStyle),

    Link(Link),
}

#[note_node]
pub enum Link {
    Web(Url),
    Page(PageLink),
}

#[note_node]
pub struct PageLink {
    pub page_id: PageId,
    pub block_id: block::BlockId,
}
