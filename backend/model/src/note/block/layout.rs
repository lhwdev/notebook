use utils::note_node;

use crate::note::style;

use super::{
    block,
    core::{Content, Tree},
};

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
