//! This module implements the bytecode interpreter that actually renders the templates.

use super::compiler::TemplateCompiler;
use super::error::Error::*;
use super::instruction::{Instruction, PathSlice, PathStep};
use super::{error::*, Ctx, Value};
use std::collections::BTreeMap;
use std::fmt::Write;

// impl <'a> Index<usize> for Value<'a> {
//     type Output = Option<Self>;

//     fn index(&self, index: usize) -> &Self::Output {
//         if let Tree(tree) = self {
//             &Some()
//         } else {
//             &None
//         }
//     }
// }

/// Enum defining the different kinds of records on the context stack.
enum ContextElement<'render, 'template> {
    /// Object contexts shadow everything below them on the stack, because every name is looked up
    /// in this object.
    Object(Value<'render>),
    /// Named contexts shadow only one name. Any path that starts with that name is looked up in
    /// this object, and all others are passed on down the stack.
    Named(&'template str, Value<'render>),
    /// Iteration contexts shadow one name with the current value of the iteration. They also
    /// store the iteration state. The two usizes are the index of the current value and the length
    /// of the array that we're iterating over.
    Iteration(
        &'template str,
        Value<'render>,
        usize,
        usize,
        std::collections::btree_map::Values<'render, PathStep<'render>, Value<'render>>,
    ),
}

/// Helper struct which mostly exists so that I have somewhere to put functions that access the
/// rendering context stack.
struct RenderContext<'render, 'template> {
    context: &'render Ctx,
    original_text: &'template str,
    context_stack: Vec<ContextElement<'render, 'template>>,
}
impl<'render, 'template> RenderContext<'render, 'template> {
    /// Look up the given path in the context stack and return the value (if found) or an error (if
    /// not)
    fn lookup(&self, path: PathSlice) -> Result<&'render Value> {
        for stack_layer in self.context_stack.iter().rev() {
            match stack_layer {
                ContextElement::Object(obj) => return self.lookup_in(path, obj),
                ContextElement::Named(name, obj) => {
                    if *name == &*path[0] {
                        return self.lookup_in(&path[1..], obj);
                    }
                }
                ContextElement::Iteration(name, obj, _, _, _) => {
                    if *name == &*path[0] {
                        return self.lookup_in(&path[1..], obj);
                    }
                }
            }
        }
        panic!("Attempted to do a lookup with an empty context stack. That shouldn't be possible.")
    }

    /// Look up a path within a given value object and return the resulting value (if found) or
    /// an error (if not)
    fn lookup_in(&self, path: PathSlice, object: &'render Value) -> Result<&'render Value> {
        let mut current = object;
        for step in path.iter() {
            match current[*step] {
                Some(next) => current = next,
                None => return Err(lookup_error(self.original_text, step, path, self.context)),
            }
        }
        Ok(current)
    }

    /// Look up the index and length values for the top iteration context on the stack.
    fn lookup_index(&self) -> Result<(usize, usize)> {
        for stack_layer in self.context_stack.iter().rev() {
            match stack_layer {
                ContextElement::Iteration(_, _, index, length, _) => return Ok((*index, *length)),
                _ => continue,
            }
        }
        Err(GenericError {
            msg: "Used @index outside of a foreach block.".to_string(),
        })
    }

    /// Look up the root context object
    fn lookup_root(&self) -> Result<&'render Value<'render>> {
        match self.context_stack.get(0) {
            Some(ContextElement::Object(obj)) => {
                Ok(obj)
            },
            Some(_) => {
                panic!("Expected Object value at root of context stack, but was something else.")
            }
            None => panic!(
                "Attempted to do a lookup with an empty context stack. That shouldn't be possible."
            ),
        }
    }
}

/// Structure representing a parsed template. It holds the bytecode program for rendering the
/// template as well as the length of the original template string, which is used as a guess to
/// pre-size the output string buffer.
pub struct Template<'template> {
    original_text: &'template str,
    instructions: Vec<Instruction<'template>>,
    template_len: usize,
}
impl<'template> Template<'template> {
    /// Create a Template from the given template string.
    pub fn compile(text: &'template str) -> Result<Template> {
        Ok(Template {
            original_text: text,
            template_len: text.len(),
            instructions: TemplateCompiler::new(text).compile()?,
        })
    }

    /// Render this template into a string and return it (or any error if one is encountered).
    pub fn render(&self, context: &Ctx) -> Result<String> {
        // The length of the original template seems like a reasonable guess at the length of the
        // output.
        let mut output = String::with_capacity(self.template_len);
        self.render_into(context, &mut output)?;
        Ok(output)
    }

    /// Render this template into a given string. Used for calling other templates.
    pub fn render_into(&self, context: &Ctx, output: &mut String) -> Result<()> {
        let mut program_counter = 0;
        let mut render_context = RenderContext {
            context,
            original_text: self.original_text,
            context_stack: vec![ContextElement::Object(Value::Tree(context.value_tree()))],
        };

        while program_counter < self.instructions.len() {
            match &self.instructions[program_counter] {
                Instruction::Literal(text) => {
                    output.push_str(text);
                    program_counter += 1;
                }
                Instruction::Value(path) => {
                    let first = path.first().unwrap();
                    if first.starts_with('@') {
                        // Currently we just hard-code the special @-keywords and have special
                        // lookup functions to use them because there are lifetime complexities with
                        // looking up values that don't live for as long as the given context object.
                        let first: &str = &*first;
                        match first {
                            "@index" => {
                                write!(output, "{}", render_context.lookup_index()?.0).unwrap()
                            }
                            "@first" => {
                                write!(output, "{}", render_context.lookup_index()?.0 == 0).unwrap()
                            }
                            "@last" => {
                                let (index, length) = render_context.lookup_index()?;
                                write!(output, "{}", index == length - 1).unwrap()
                            }
                            "@root" => {
                                let value_to_render = render_context.lookup_root()?;
                                if let Value::Value(value) = value_to_render {
                                    output.push_str(value);
                                } else {
                                    panic!()
                                }
                            }
                            _ => panic!(), // This should have been caught by the parser.
                        }
                    } else {
                        let value_to_render = render_context.lookup(path)?;
                        if let Value::Value(value) = value_to_render {
                            output.push_str(value);
                        } else {
                            panic!()
                        }
                    }
                    program_counter += 1;
                }
                Instruction::FormattedValue(path, name) => {
                    // // The @ keywords aren't supported for formatted values. Should they be?
                    // let value_to_render = render_context.lookup(path)?;
                    // match formatter_registry.get(name) {
                    //     Some(formatter) => {
                    //         let formatter_result = formatter(value_to_render, output);
                    //         if let Err(err) = formatter_result {
                    //             return Err(called_formatter_error(self.original_text, name, err));
                    //         }
                    //     }
                    //     None => return Err(unknown_formatter(self.original_text, name)),
                    // }
                    // program_counter += 1;
                    todo!("FormattedValue wow")
                }
                Instruction::Branch(path, negate, target) => {
                    let first = path.first().unwrap();
                    let mut truthy = if first.starts_with('@') {
                        let first: &str = &*first;
                        match &*first {
                            "@index" => render_context.lookup_index()?.0 != 0,
                            "@first" => render_context.lookup_index()?.0 == 0,
                            "@last" => {
                                let (index, length) = render_context.lookup_index()?;
                                index == (length - 1)
                            }
                            "@root" => self.value_is_truthy(render_context.lookup_root()?, path)?,
                            other => panic!("Unknown keyword {}", other), // This should have been caught by the parser.
                        }
                    } else {
                        let value_to_render = render_context.lookup(path)?;
                        self.value_is_truthy(value_to_render, path)?
                    };
                    if *negate {
                        truthy = !truthy;
                    }

                    if truthy {
                        program_counter = *target;
                    } else {
                        program_counter += 1;
                    }
                }
                Instruction::PushNamedContext(path, name) => {
                    let context_value = render_context.lookup(path)?;
                    render_context
                        .context_stack
                        .push(ContextElement::Named(name, *context_value));
                    program_counter += 1;
                }
                Instruction::PushIterationContext(path, name) => {
                    // We push a context with an invalid index and no value and then wait for the
                    // following Iterate instruction to set the index and value properly.
                    let first = path.first().unwrap();
                    let context_value = match first {
                        PathStep::Name("@root") => render_context.lookup_root()?,
                        PathStep::Name(other) if other.starts_with('@') => {
                            return Err(not_iterable_error(self.original_text, path))
                        }
                        _ => render_context.lookup(path)?,
                    };
                    match context_value {
                        Value::Tree(ref arr) => {
                            let arr: &BTreeMap<PathStep, Value> = arr.list();
                            render_context.context_stack.push(ContextElement::Iteration(
                                name,
                                Value::Value(""),
                                ::std::usize::MAX,
                                arr.len(),
                                arr.values(),
                            ))
                        }
                        _ => return Err(not_iterable_error(self.original_text, path)),
                    };
                    program_counter += 1;
                }
                Instruction::PopContext => {
                    render_context.context_stack.pop();
                    program_counter += 1;
                }
                Instruction::Goto(target) => {
                    program_counter = *target;
                }
                Instruction::Iterate(target) => {
                    match render_context.context_stack.last_mut() {
                        Some(ContextElement::Iteration(_, val, index, _, iter)) => {
                            match iter.next() {
                                Some(new_val) => {
                                    *val = *new_val;
                                    // On the first iteration, this will be usize::MAX so it will
                                    // wrap around to zero.
                                    *index = index.wrapping_add(1);
                                    program_counter += 1;
                                }
                                None => {
                                    program_counter = *target;
                                }
                            }
                        }
                        _ => panic!("Malformed program."),
                    };
                }
                Instruction::Call(template_name, path) => {
                    // let context_value = render_context.lookup(path)?;
                    // match context.value_tree().get_function_nested() {
                    //     Some(templ) => {
                    //         let called_templ_result = templ.render_into(
                    //             context_value,
                    //             template_registry,
                    //             formatter_registry,
                    //             default_formatter,
                    //             output,
                    //         );
                    //         if let Err(err) = called_templ_result {
                    //             return Err(called_template_error(
                    //                 self.original_text,
                    //                 template_name,
                    //                 err,
                    //             ));
                    //         }
                    //     }
                    //     None => return Err(unknown_template(self.original_text, template_name)),
                    // }
                    // program_counter += 1;
                    todo!("Call")
                }
            }
        }
        Ok(())
    }

    fn value_is_truthy(&self, value: &Value, path: PathSlice) -> Result<bool> {
        let truthy = match value {
            Value::Tree(tree) => true,
            Value::Value(value) => !value.is_empty(),
            Value::Function(function) => true,
            
            // Value::Null => false,
            // Value::Bool(b) => *b,
            // Value::Number(n) => match n.as_f64() {
            //     Some(float) => float != 0.0,
            //     None => {
            //         return Err(truthiness_error(self.original_text, path));
            //     }
            // },
            // Value::String(s) => !s.is_empty(),
            // Value::Array(arr) => !arr.is_empty(),
            // Value::Object(_) => true,

        };
        Ok(truthy)
    }
}
