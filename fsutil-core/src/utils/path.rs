//! ## Path
//!
//! path utilities

use std::path::{Path, PathBuf};

/// Absolutize target path if relative.
pub fn absolutize(wrkdir: &Path, target: &Path) -> PathBuf {
    match target.is_absolute() {
        true => target.to_path_buf(),
        false => {
            let mut p: PathBuf = wrkdir.to_path_buf();
            p.push(target);
            p
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn absolutize_path() {
        assert_eq!(
            absolutize(&Path::new("/home/user0"), &Path::new("test.txt")).as_path(),
            Path::new("/home/user0/test.txt")
        );
        assert_eq!(
            absolutize(&Path::new("/home/user0"), &Path::new("/tmp/test.txt")).as_path(),
            Path::new("/tmp/test.txt")
        );
    }
}
