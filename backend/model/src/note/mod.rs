pub mod block;
pub mod component;
pub mod span;
pub mod style;

use utils::note_node;

pub type PageId = u32;

#[note_node]
pub struct Page {
    pub id: PageId,
    pub name: String,

    pub root: block::Tree,
}
