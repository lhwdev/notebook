mod text;
mod other;

pub use self::text::*;
pub use self::other::*;

use utils::note_node;

macro_rules! span {
    ($item:item) => {
        $item
    };
}
pub(self) use span;

macro_rules! span_kind {
    ($item:item) => {
        $item
    };
}

#[note_node(span_kind)]
pub enum Span {}
