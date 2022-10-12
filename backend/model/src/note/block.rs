use utils::{deref_enum, note_node};

use super::{component, span, style};

pub type BlockId = u32;

pub trait BlockKind {
    fn block_kind(&self) -> &dyn BlockKind;

    fn id(&self) -> BlockId {
        self.block_kind().id()
    }
}

macro_rules! block {
    (
        $( #[$attr:meta] )*
        pub struct $Name:ident {
            $( pub $field_name:ident : $FieldType:ty ),* $(,)?
        }
    ) => {
        $( #[$attr] )*
        pub struct $Name {
            pub id: BlockId,
            $( pub $field_name: $FieldType, )*
        }

        impl BlockKind for $Name {
            fn block_kind(&self) -> &dyn BlockKind {
                self
            }

            fn id(&self) -> BlockId {
                self.id
            }
        }
    };
}

macro_rules! block_kind {
    (
        $( #[$meta:meta] )+
        pub enum $name:ident $body:tt
    ) => {
        #[deref_enum(BlockKind)]
        $( #[$meta] )+
        pub enum $name $body

        impl BlockKind for $name {
            fn block_kind(&self) -> &dyn BlockKind {
                std::ops::Deref::deref(self).block_kind()
            }
        }
    };
}

#[note_node(block_kind)]
pub enum Block {
    Tree(Tree),
    Text(Text),
    Heading(Heading),
    List(List),
    Divider(Divider),
    Callout(Callout),
}

#[note_node(block)]
pub struct Tree {
    pub blocks: Vec<Block>,
    pub style: Option<style::BoxStyle>,
}

#[note_node(block)]
pub struct Indent {
    pub content: Vec<Block>,
    pub level: u8,
}

#[note_node(block)]
pub struct Empty {}

#[note_node(block)]
pub struct Text {
    pub content: Content,
}

#[note_node]
pub struct Content {
    pub spans: Vec<span::Span>,
}

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
    pub bullet: Option<span::Span>,
}

#[note_node(block)]
pub struct NumberedList {
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
pub struct Divider {}

#[note_node(block)]
pub struct Column {
    pub columns: Vec<ColumnItem>,
}

#[note_node]
pub struct ColumnItem {
    pub content: Tree,
    pub weight: Option<f32>,
}

#[note_node(block)]
pub struct Quote {
    pub content: Tree,
}

#[note_node(block)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub cell_option: CellOption,
}

#[note_node]
pub struct TableRow {
    pub cells: Vec<TableCellOrMerge>,
    pub cell_option: CellOption,
    pub line_fill: Option<style::Fill>,
}

#[note_node]
pub enum TableCellOrMerge {
    Cell(TableCell),
    MergeWithLeft,
}

#[note_node]
pub struct TableCell {
    pub content: Content,
    pub cell_option: CellOption,
}

#[note_node]
#[derive(Default)]
pub struct CellOption {
    pub background_fill: Option<style::Fill>,
    pub fill: Option<style::Fill>,
    // pub align: Option<HorizontalAlignment>, // LEFT
}

#[note_node]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[note_node(block)]
pub struct Callout {
    pub content: Tree,
    pub trailing_icon: style::Icon,
}

#[note_node(block)]
pub struct Component {
    pub component_id: component::ComponentId,
    pub parameters: serde_json::Value,
}

// macro_rules! block_with_content {
//     (
//         $meta:meta
//         pub struct $name:ident $body:tt
//     ) => {
//         $meta
//         pub struct $name $body
//
//         impl IntoContent for $name
//     };
// }
