mod root;
mod deps;
mod package;
mod utils;

use serde::Deserializer;
use std::collections::BTreeMap;
use toml::Value;
use self::root::RootVisitor;

pub fn transform_template(
    content: &str,
    data: &Data
) -> Result<toml::Value, toml::de::Error> {
    let mut deserializer = toml::Deserializer::new(content);
    let visitor = RootVisitor(&data);
    deserializer.deserialize_any(visitor)
}

pub struct Data {
    dir_nested_level: usize,
    env: Env
}

pub struct Env {
    package_default: BTreeMap<String, Value>,
    deps_default: BTreeMap<String, Value>,
    deps: BTreeMap<String, Value>
}

impl Data {
    pub fn get_deps(&self, key: &str) -> Option<Value> {
        match key {
            "projects" => {
                let mut map = BTreeMap::new();
                let path_prefix = "../".repeat(self.dir_nested_level);
                let path = format!("{}{}", path_prefix, key.replace('.', "/"));
                map.insert("path".to_string(), Value::String(path));
                Some(Value::Table(map))
            },
            _ => Some(self.env.deps.get(key)?.clone())
        }
    }
}
