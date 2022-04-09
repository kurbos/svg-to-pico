use clap::ArgMatches;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

/// `PathArg` stores a path argument received from the user through CLI.
///
/// For common use cases, paths provided will either be directories or files,
/// possibly accessed through a symlink; other paths will result in an error
/// during parsing.
///
/// Input files/directories must exist, but it is possible that output files
/// and directories might not exist.
pub enum PathArgs {
    File { input: PathBuf, output: PathBuf },
    Directory { input: PathBuf, output: PathBuf },
}

impl PathArgs {
    pub fn type_as_string(&self) -> String {
        match self {
            PathArgs::File { .. } => "file".into(),
            PathArgs::Directory { .. } => "directory".into(),
        }
    }

    pub fn input(&self) -> &PathBuf {
        match self {
            PathArgs::File { input, .. } => input,
            PathArgs::Directory { input, .. } => input,
        }
    }
}
