//! Configuration file lookup functions.

use std::path::{Path, PathBuf};

/// The type of a configuration file.
pub enum Type {
    System,
    Application,
    Machine,
}

/// Find a configuration file in the configuration root directory.
pub fn find(t: Type, root: &Path, name: &str) -> PathBuf {
    let mut p: PathBuf = [
        root,
        Path::new(match t {
            Type::System => "systems",
            Type::Application => "apps",
            Type::Machine => "machines",
        }),
        Path::new(name),
    ]
    .iter()
    .collect();

    p.set_extension("dhall");
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(
            find(Type::System, Path::new("root"), "foo").as_path(),
            Path::new("root/systems/foo.dhall")
        )
    }
}
