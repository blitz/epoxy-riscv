use failure::Error;
use itertools::Itertools;

use crate::runtypes;

// cat src/state.cpp
// #include "state.hpp"
// #include "kobject_all.hpp"
// namespace {
// extern exit_kobject kobject_0;
// extern klog_kobject kobject_1;
// extern process kobject_2;
// extern thread kobject_3;
// kobject * const p0_capability_set[2] {&(kobject_0),&(kobject_1)};
// exit_kobject kobject_0 {};
// klog_kobject kobject_1 {"hello"};
// process kobject_2 {0,p0_capability_set};
// thread kobject_3 {&(kobject_2),65716,536887288};
// }
// thread * const threads[1] {&(kobject_3)};
// cat include/state.hpp
// #pragma once
// #include "thread.hpp"
// extern thread * const threads[1];

type Type = String;

enum Statement {
    PragmaOnce,
    Include {
        header: String,
    },
    FwdDeclaration {
        r#type: Type,
        name: String,
    },
    ArrayFwdDeclaration {
        r#type: Type,
        name: String,
        count: usize,
    },
    Namespace {
        name: String,
        statements: Vec<Statement>,
    },
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Statement::PragmaOnce => write!(f, "#pragma once"),
            Statement::Include { header } => write!(f, "#include \"{}\"", header),
            Statement::FwdDeclaration { r#type, name } => write!(f, "extern {} {};", r#type, name),
            Statement::ArrayFwdDeclaration {
                r#type,
                name,
                count,
            } => write!(f, "extern {} {}[{}];", r#type, name, count),
            Statement::Namespace { name, statements } => write!(
                f,
                "namespace {} {{\n{}\n}}",
                name,
                statements.iter().map(|s| s.to_string()).join("\n")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_display() {
        assert_eq!(
            Statement::Include {
                header: "state.hpp".to_string()
            }
            .to_string(),
            "#include \"state.hpp\""
        );
        assert_eq!(
            Statement::FwdDeclaration {
                r#type: "char".to_string(),
                name: "foo".to_string()
            }
            .to_string(),
            "extern char foo;"
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
            Statement::Namespace {
                name: "bar".to_string(),
                statements: vec![
                    Statement::FwdDeclaration {
                        r#type: "char".to_string(),
                        name: "foo".to_string()
                    },
                    Statement::FwdDeclaration {
                        r#type: "int".to_string(),
                        name: "bar".to_string()
                    },
                ]
            }
            .to_string(),
            "namespace bar {
extern char foo;
extern int bar;
}"
        )
    }
}

/// Generate the C++ code for the kernel configuration.
pub fn generate_cpp(_system: &runtypes::Configuration) -> Result<String, Error> {
    todo!()
}

/// Generate the header file for the kernel configuration.
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
