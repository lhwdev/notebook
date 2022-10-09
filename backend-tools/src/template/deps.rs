use super::{Data, utils::map_toml_visitor};
use toml::Value;

pub struct DepsVisitor<'a>(pub &'a Data, );

map_toml_visitor! {
    impl Visitor for DepsVisitor {
        fn transform_each_entry(map, data, key, value) {
            match key.as_str() {
                "prelude" => {
                    if let Value::Boolean(new_prelude) = value.get()? {
                        
                    } else {
                        panic!("prelude is not type of boolean")
                    }
                }
                _ => {
                    let mut value = value.get()?;
                    if let Value::Table(table) = &value {
                        if table.is_empty() {
                            value = data.get_deps(&key)
                                .unwrap_or_else(|| panic!("env: deps.{} does not exist", key));
                        }
                    }
                    map.insert(key, value);
                }
            }
        }
    }
}
