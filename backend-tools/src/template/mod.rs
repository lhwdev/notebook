// Original: https://github.com/bheisler/TinyTemplate
// Copied from commit 141836defdc4ffedc7f3beec9a1dd7e06e7cbf8e

// Modified:
// - Changed { }, {{ }} into {{ }}, {{{ }}} to avoid collision
// - Custom context for maximizing extensibility

//! ## TinyTemplate
//!
//! TinyTemplate is a minimal templating library originally designed for use in [Criterion.rs].
//! It deliberately does not provide all of the features of a full-power template engine, but in
//! return it provides a simple API, clear templating syntax, decent performance and very few
//! dependencies.
//!
//! ## Features
//!
//! The most important features are as follows (see the [syntax](syntax/index.html) module for full
//! details on the template syntax):
//!
//! * Rendering values - `{ myvalue }`
//! * Conditionals - `{{ if foo }}Foo is true{{ else }}Foo is false{{ endif }}`
//! * Loops - `{{ for value in row }}{value}{{ endfor }}`
//! * Customizable value formatters `{ value | my_formatter }`
//! * Macros `{{ call my_template with foo }}`
//!
//! ## Restrictions
//!
//! TinyTemplate was designed with the assumption that the templates are available as static strings,
//! either using string literals or the `include_str!` macro. Thus, it borrows `&str` slices from the
//! template text itself and uses them during the rendering process. Although it is possible to use
//! TinyTemplate with template strings loaded at runtime, this is not recommended.
//!
//! Additionally, TinyTemplate can only render templates into Strings. If you need to render a
//! template directly to a socket or file, TinyTemplate may not be right for you.
//!
//! ## Example
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate tinytemplate;
//!
//! use tinytemplate::TinyTemplate;
//! use std::error::Error;
//!
//! #[derive(Serialize)]
//! struct Context {
//!     name: String,
//! }
//!
//! static TEMPLATE : &'static str = "Hello {name}!";
//!
//! pub fn main() -> Result<(), Box<Error>> {
//!     let mut tt = TinyTemplate::new();
//!     tt.add_template("hello", TEMPLATE)?;
//!
//!     let context = Context {
//!         name: "World".to_string(),
//!     };
//!
//!     let rendered = tt.render("hello", &context)?;
//! #   assert_eq!("Hello World!", &rendered);
//!     println!("{}", rendered);
//!
//!     Ok(())
//! }
//! ```
//!
//! [Criterion.rs]: https://github.com/bheisler/criterion.rs
//!

mod compiler;
pub mod error;
mod instruction;
pub mod syntax;
pub mod template;

use std::{ops::Index, collections::BTreeMap};

use self::instruction::{PathSlice, PathStep};

pub(crate) type Ctx = Box<dyn Context>;
pub(crate) type Tree<'a> = Box<dyn ValueTree<'a>>;
pub type Function = Box<dyn Fn(&str) -> &str>;

pub trait Context {
    fn value_tree(&self) -> &Tree;
}

pub trait ValueTree<'a> : Index<PathStep<'a>, Output = Value<'a>> {
    fn list(&self) -> &BTreeMap<PathStep, Value>;

    fn get_value(&self, path_step: PathStep) -> Option<&str> {
        if let Value::Value(value) = self[path_step] {
            Some(value)
        } else {
            None
        }
    }

    fn get_function(&self, path_step: PathStep) -> Option<&Function> {
        if let Value::Function(function) = self[path_step] {
            Some(function)
        } else {
            None
        }
    }

    fn get_tree(&self, path_step: PathStep) -> Option<&Tree> {
        if let Value::Tree(tree) = self[path_step] {
            Some(tree)
        } else {
            None
        }
    }

    fn list_nested(&self, path: PathSlice) -> Option<&BTreeMap<PathStep, Value>> {
        Some(self.get_tree_nested(&path[0..path.len() - 1])?
            .list())
    }

    fn get_value_nested(&self, path: PathSlice) -> Option<&str> {
        self.get_tree_nested(&path[0..path.len() - 1])?
            .get_value(*path.last().unwrap())
    }

    fn get_function_nested(&self, path: PathSlice) -> Option<&Function> {
        self.get_tree_nested(&path[0..path.len() - 1])?
            .get_function(*path.last().unwrap())
    }

    fn get_tree_nested(&self, path: PathSlice) -> Option<&Tree> {
        let mut tree: &Tree = self.get_tree(*path.first().unwrap())?;
        for path_step in &path[1..path.len()] {
            tree = tree.get_tree(*path_step)?;
        }
        Some(tree)
    }
}

#[derive(Clone, Copy)]
pub enum Value<'a> {
    Tree(&'a Tree<'a>),
    Value(&'a str),
    Function(&'a Function)
}

impl <'a> Index<PathStep<'a>> for Value<'a> {
    type Output = Option<&'a Value<'a>>;

    fn index(&self, index: PathStep) -> &Self::Output {
        if let Value::Tree(tree) = self {
            &Some(tree.index(index))
        } else {
            &None
        }
    }
}
