mod template;

use std::{fs::{self, read_to_string}, path::PathBuf, env::current_dir};
use template::template::Template;
use toml::Value;

fn index_segments<'a>(query: &'a str, value: &'a Value) -> &'a str {
  let mut value = value;
  for item in query.split(".") {
    value = value.get(item).unwrap();
  }
  value.as_str().unwrap()
}

struct TemplateContext;



fn main() {
  let mut backend = PathBuf::new();
  backend.extend(current_dir().unwrap().parent().unwrap());
  backend.push("backend");

  let mut env_path = backend.clone();
  env_path.push("env.toml");
  let env: Value = fs::read_to_string(env_path).unwrap().parse().unwrap();

  // let context = 

  for module in backend.read_dir().unwrap() {
    let module = module.unwrap();
    let metadata = module.metadata().unwrap();
    if metadata.is_dir() && module.file_name() != "target" {
      let mut path = module.path();
      path.push("Cargo.template.toml");

      let file = read_to_string(path).expect("Cargo.template.toml does not exist");

      let template = Template::compile(file.as_str()).expect("Error! file!");
      // template.render(context)
    }
  }
}
