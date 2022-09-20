macro_rules! note_node {
    (
        $vis:vis struct $name:ident {
            $($field_name:ident : $field_type:ty),+
        }
    ) => {
        #[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
        $vis struct $name {
            $($field_name : $field_type),+
        }
    };
}


note_node! {
pub struct Note {
    pub name: String,

    pub root: Tree
}
}

note_node! {
pub enum Block {
    Tree(Tree),
}
}

note_node! {
pub struct Tree {
    blocks: Vec<Block>,
}
}

note_node! {
pub struct Text {
    spans: Vec<Span>
}
}

note_node! {
pub enum Span {

}
}

