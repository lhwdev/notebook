pub mod block;
pub mod span;
pub mod style;
pub mod component;

use utils::note_node;

pub type PageId = u32;

#[note_node]
pub struct Page {
    pub id: PageId,
    pub name: String,

    pub root: block::Tree,
}
