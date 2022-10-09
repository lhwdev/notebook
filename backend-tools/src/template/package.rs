use super::{Data, utils::map_toml_visitor};
use toml::Value;

pub struct PackageVisitor<'a>(pub &'a Data);

map_toml_visitor! {
    impl Visitor for PackageVisitor {
        fn extra(map, data) {
            map.extend(data.env.package_default.clone());
        }

        fn map_each_entry(data, key, value) {
            let mut value = value.get()?;
            if let Value::Table(table) = &value {
                if table.is_empty() {
                    value = data.get_deps(&key)
                        .unwrap_or_else(|| panic!("env: deps.{} does not exist", key));
                }
            }
            (key, value)
        }
    }
}
