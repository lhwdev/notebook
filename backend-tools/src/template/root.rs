use super::{deps::DepsVisitor, utils::map_toml_visitor, Data};
use crate::toml_visitor::MapDeserializeSeed;

pub struct RootVisitor<'a>(pub &'a Data);

map_toml_visitor! {
    impl Visitor for RootVisitor {
        fn map_each_entry(data, key, value) {
            match key.as_str() {
                "dependencies" => (key, value.with_seed(MapDeserializeSeed::new(DepsVisitor(data)))?),
                _ => (key, value.get()?),
            }
        }
    }
}
