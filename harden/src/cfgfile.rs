use std::path::{Path, PathBuf};
use failure::Error;

pub enum Type {
    System
}

pub fn find(t: Type, root: &Path, name: &str) -> PathBuf
{
    let mut p : PathBuf =  match t {
        System => [root, Path::new("systems"), Path::new(name)].iter().collect()
    };

    p.set_extension("dhall");
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(find(Type::System, Path::new("root"), "foo").as_path(),
                   Path::new("root/systems/foo.dhall"))
    }
}
