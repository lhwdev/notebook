mod template;
mod toml_visitor;

use std::{
    collections::BTreeMap,
    env::current_dir,
    fs::{self, read_to_string, write},
    path::{PathBuf, Path},
};

fn convert(value: toml::Value) -> liquid::model::Value {
    match value {
        toml::Value::String(x) => liquid::model::Value::scalar(x),
        toml::Value::Integer(x) => liquid::model::Value::scalar(x),
        toml::Value::Float(x) => liquid::model::Value::scalar(x),
        toml::Value::Boolean(x) => liquid::model::Value::scalar(x),
        toml::Value::Datetime(_x) => todo!(),
        toml::Value::Array(x) => liquid::model::Value::array(x.into_iter().map(convert)),
        toml::Value::Table(x) => {
            let mut obj = liquid::Object::new();
            for (key, value) in x {
                obj.insert(liquid::model::KString::from_string(key), convert(value));
            }
            liquid::model::Value::Object(obj)
        }
    }
}

fn convert_root(env: BTreeMap<String, toml::Value>, backend_dir: &Path) -> liquid::Object {
    let mut obj = liquid::Object::new();
    for (key, value) in env {
        let key_map = liquid::model::KString::from_ref(key.as_str());
        match key.as_str() {
            "deps" => {
                if let toml::Value::Table(table) = value {
                    let mut deps = liquid::Object::new();
                    for (key, value) in table {
                        let value = if let toml::Value::String(string) = value { string } else { panic!() };
                        let scalar = match value.chars().nth(0) {
                            Some('@') => liquid::model::Value::scalar(value.chars().skip(1).collect::<String>()),
                            Some('{') => liquid::model::Value::scalar(format!("{} = {}", key, value)),
                            _ => liquid::model::Value::scalar(format!("{} = '{}'", key, value))
                        };
                        deps.insert(liquid::model::KString::from_string(key), scalar);
                    }

                    fn project_deps_for(path: &Path) -> liquid::Object {
                        let mut projects = liquid::Object::new();
                        for module in path.read_dir().unwrap() {
                            let module = module.unwrap();
                            if module.file_type().unwrap().is_dir() {
                                if module.path().join("Cargo.toml.template").exists() {
                                    let name = module.file_name().to_str().unwrap().to_string();
                                    projects.insert(name.clone().into(), liquid::model::Value::scalar(format!("{} = {{ path = \"../{}\" }}", &name, &name)));
                                }
                            }
                        }
                        projects
                    }

                    let mut projects = liquid::Object::new();
                    for module in backend_dir.read_dir().unwrap() {
                        let module = module.unwrap();
                        if module.file_type().unwrap().is_dir() {
                            let name = module.file_name().to_str().unwrap().to_string();
                            projects.insert(name.clone().into(), liquid::model::Value::scalar(format!("{} = {{ path = \"../{}\" }}", &name, &name)));
                        }
                    }
                    deps.insert("projects".into(), liquid::model::Value::Object(projects));

                    obj.insert(key_map, liquid::model::Value::Object(deps));
                } else {
                    panic!("deps shape does not look good");
                }
            }
            _ => {
                obj.insert(key_map, convert(value));
            }
        };
    }
    obj
}

fn main() {
    let mut backend = PathBuf::new();
    backend.extend(current_dir().unwrap().parent().unwrap());
    backend.push("backend");

    let mut env_path = backend.clone();
    env_path.push("env.toml");
    let env: toml::Value = fs::read_to_string(env_path).unwrap().parse().unwrap();
    let env = if let toml::Value::Table(table) = env {
        table
    } else {
        panic!()
    };

    let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
    let context = convert_root(env, &backend);

    for module in backend.read_dir().unwrap() {
        let module = module.unwrap();
        let metadata = module.metadata().unwrap();
        if metadata.is_dir() && module.file_name() != "target" {
            let mut path = module.path();
            path.push("Cargo.toml.template"); // intentionally changed extension: toml does not allow {{ variable }} syntax

            let file = read_to_string(path);
            let file = if let Ok(file) = file { file } else { continue };

            let template = parser.parse(file.as_str()).expect("Parse error");

            let mut output = module.path();
            output.push("Cargo.toml");
            let result = template.render(&context).expect("Text error");
            write(output, format!(
                "# This file was auto-generated.
# To modify, see Cargo.toml.template and backend-tools.
{}",
                result
            )).unwrap();
        }
    }
}
