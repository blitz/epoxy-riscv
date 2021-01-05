use failure::Error;
use itertools::Itertools;

use crate::runtypes;

type Type = String;

#[derive(Clone)]
enum Expression {
    LiteralUnsigned(u64),
    LiteralString(String),
    Identifier(String),
    AddressOf(Box<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expression::LiteralUnsigned(i) => write!(f, "{:#x}", i),
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::AddressOf(e) => write!(f, "&({})", e),

            // TODO Quote string!
            Expression::LiteralString(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Clone)]
enum Statement {
    PragmaOnce,
    Include {
        header: String,
    },
    ArrayFwdDeclaration {
        r#type: Type,
        name: String,
        count: usize,
    },
    ArrayDefinition {
        r#type: Type,
        name: String,
        init_args: Vec<Expression>,
    },
    VariableDefinition {
        r#type: Type,
        name: String,
        init_args: Vec<Expression>,
    },
    AnonNamespace {
        statements: Vec<Statement>,
    },
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Statement::PragmaOnce => write!(f, "#pragma once"),
            Statement::Include { header } => write!(f, "#include \"{}\"", header),
            Statement::ArrayFwdDeclaration {
                r#type,
                name,
                count,
            } => write!(f, "extern {} {}[{}];", r#type, name, count),
            Statement::ArrayDefinition {
                r#type,
                name,
                init_args,
            } => write!(
                f,
                "{} {}[{}] {{{}}};",
                r#type,
                name,
                init_args.len(),
                init_args.iter().map(|e| e.to_string()).join(", ")
            ),
            Statement::VariableDefinition {
                r#type,
                name,
                init_args,
            } => write!(
                f,
                "{} {} {{{}}};",
                r#type,
                name,
                init_args.iter().map(|e| e.to_string()).join(", ")
            ),

            Statement::AnonNamespace { statements } => write!(
                f,
                "namespace {{\n{}\n}}",
                statements.iter().map(|s| s.to_string()).join("\n")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_expression() {
        assert_eq!(Expression::LiteralUnsigned(0x1234).to_string(), "0x1234");
        assert_eq!(
            Expression::AddressOf(Box::new(Expression::Identifier("foo".to_string()))).to_string(),
            "&(foo)"
        );
    }

    #[test]
    fn statement_display() {
        assert_eq!(
            Statement::Include {
                header: "state.hpp".to_string()
            }
            .to_string(),
            "#include \"state.hpp\""
        );
        assert_eq!(
            Statement::ArrayFwdDeclaration {
                r#type: "char".to_string(),
                name: "foo".to_string(),
                count: 7
            }
            .to_string(),
            "extern char foo[7];"
        );
        assert_eq!(
            Statement::ArrayDefinition {
                r#type: "object".to_string(),
                name: "foo".to_string(),
                init_args: vec![
                    Expression::LiteralUnsigned(7),
                    Expression::Identifier("id".to_string())
                ]
            }
            .to_string(),
            "object foo[2] {0x7, id};"
        );
        assert_eq!(
            Statement::AnonNamespace {
                statements: vec![Statement::VariableDefinition {
                    r#type: "char".to_string(),
                    name: "foo".to_string(),
                    init_args: vec![]
                },]
            }
            .to_string(),
            "namespace {
char foo {};
}"
        )
    }
}

#[derive(Default)]
struct IdentifierIterator {
    id: usize,
}

impl Iterator for IdentifierIterator {
    type Item = String;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let new_identifier = format!("id_{}", self.id);
        self.id = self.id + 1;
        Some(new_identifier)
    }
}

fn pointer_to(s: &str) -> Expression {
    Expression::AddressOf(Box::new(Expression::Identifier(s.to_string())))
}

fn process_entry(_process: &runtypes::Process) -> u64 {
    // TODO Returning bogus process entry point!
    0xDEADBEEF
}

fn process_stack_ptr(_process: &runtypes::Process) -> u64 {
    // TODO Returning bogus stack pointer!
    0xCAFED00D
}

/// Returns the name of the thread that is created in addition to all statements that need to go
/// into the state file to create the necessary kernel options.
fn process_kobjects(
    id_iter: &mut IdentifierIterator,
    pid: u64,
    process: &runtypes::Process,
) -> Result<(String, Vec<Statement>), Error> {
    let thread_name = id_iter.next().unwrap();
    let exit_name = id_iter.next().unwrap();
    let klog_name = id_iter.next().unwrap();
    let capset_name = id_iter.next().unwrap();
    let proc_name = id_iter.next().unwrap();

    Ok((
        thread_name.clone(),
        vec![
            Statement::VariableDefinition {
                r#type: "exit_kobject".to_string(),
                name: exit_name.to_string(),
                init_args: vec![],
            },
            Statement::VariableDefinition {
                r#type: "klog_kobject".to_string(),
                name: klog_name.to_string(),
                init_args: vec![Expression::LiteralString(process.name.to_string())],
            },
            Statement::ArrayDefinition {
                r#type: "kobject * const".to_string(),
                name: capset_name.to_string(),
                init_args: vec![pointer_to(&exit_name), pointer_to(&klog_name)],
            },
            Statement::VariableDefinition {
                r#type: "process".to_string(),
                name: proc_name.to_string(),
                init_args: vec![Expression::LiteralUnsigned(pid), pointer_to(&capset_name)],
            },
            Statement::VariableDefinition {
                r#type: "thread".to_string(),
                name: thread_name,
                init_args: vec![
                    pointer_to(&proc_name),
                    Expression::LiteralUnsigned(process_entry(&process)),
                    Expression::LiteralUnsigned(process_stack_ptr(&process)),
                ],
            },
        ],
    ))
}

/// Generate the C++ code for the kernel configuration.
///
/// The output will look like this:
///
/// ```c++
/// #include "state.hpp"
/// #include "kobject_all.hpp"
/// namespace {
/// extern exit_kobject kobject_0;
/// extern klog_kobject kobject_1;
/// extern process kobject_2;
/// extern thread kobject_3;
/// kobject * const p0_capability_set[2] {&(kobject_0),&(kobject_1)};
/// exit_kobject kobject_0 {};
/// klog_kobject kobject_1 {"hello"};
/// process kobject_2 {0,p0_capability_set};
/// thread kobject_3 {&(kobject_2),65716,536887288};
/// }
/// thread * const threads[1] {&(kobject_3)};
/// ```
pub fn generate_cpp(system: &runtypes::Configuration) -> Result<String, Error> {
    let mut id_iter = IdentifierIterator::default();
    let procs: Vec<(String, Vec<Statement>)> = system
        .processes
        .values()
        .zip(0..)
        .map(|(p, pid)| process_kobjects(&mut id_iter, pid, p))
        .collect::<Result<Vec<(String, Vec<Statement>)>, Error>>()?;

    let proc_stm: Vec<Statement> = procs.iter().map(|(_, s)| s).flatten().cloned().collect();

    Ok([
        Statement::Include {
            header: "state.hpp".to_string(),
        },
        Statement::Include {
            header: "kobject_all.hpp".to_string(),
        },
        Statement::AnonNamespace {
            statements: proc_stm,
        },
        Statement::ArrayDefinition {
            name: "threads".to_string(),
            r#type: "thread * const".to_string(),
            init_args: procs
                .iter()
                .map(|(t, _)| {
                    Expression::AddressOf(Box::new(Expression::Identifier(t.to_string())))
                })
                .collect(),
        },
    ]
    .iter()
    .map(|s| s.to_string())
    .join("\n")
        + "\n")
}

/// Generate the header file for the kernel configuration.
///
/// The result will look like this:
///
/// ```c++
/// #pragma once
/// #include "thread.hpp"
/// extern thread * const threads[1];
/// ```
pub fn generate_hpp(system: &runtypes::Configuration) -> Result<String, Error> {
    Ok([
        Statement::PragmaOnce,
        Statement::Include {
            header: "thread.hpp".to_string(),
        },
        Statement::ArrayFwdDeclaration {
            r#type: "thread * const".to_string(),
            name: "threads".to_string(),
            count: system.processes.len(),
        },
    ]
    .iter()
    .map(|s| s.to_string())
    .join("\n")
        + "\n")
}
