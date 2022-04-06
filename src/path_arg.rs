use std::ffi::OsStr;
use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

/// `PathArg` stores a path argument received from the user through CLI.
///
/// For common use cases, paths provided will either be directories or files,
/// possibly accessed through a symlink. However, it is possible to
/// receive other paths; for example, on Windows, one might receive a
/// Universal Naming Convention (UNC) path. `PathArg` is an enum to keep track
/// of such variety.
pub enum PathArg {
    File(PathBuf),
    Directory(PathBuf),
    NonExistent(PathBuf),
    Unknown(PathBuf),
}

impl PathArg {
    pub fn inner(&self) -> &Path {
        match self {
            PathArg::File(path) => path,
            PathArg::Directory(path) => path,
            PathArg::NonExistent(path) => path,
            PathArg::Unknown(path) => path,
        }
    }

    pub fn exists(&self) -> bool {
        self.inner().exists()
    }

    pub fn is_file(&self) -> bool {
        matches!(self, PathArg::File(_))
    }

    pub fn type_as_string(&self) -> String {
        match self {
            PathArg::File(_) => "file".into(),
            PathArg::Directory(_) => "directory".into(),
            PathArg::NonExistent(_) => "does not exist".into(),
            PathArg::Unknown(_) => "unknown type".into(),
        }
    }
}

impl From<&OsStr> for PathArg {
    fn from(raw_path: &OsStr) -> Self {
        let path: PathBuf = PathBuf::from(raw_path);
        if path.exists() {
            if path.is_file() {
                PathArg::File(path)
            } else if path.is_dir() {
                PathArg::Directory(path)
            } else {
                PathArg::Unknown(path)
            }
        } else {
            PathArg::NonExistent(path)
        }
    }
}

impl fmt::Display for PathArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner().display())
    }
}
