mod path_arg;

use crate::path_arg::PathArgs;
use clap::{arg, command, ArgMatches};
use log::error;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

pub fn parse_paths(matches: &ArgMatches) -> Result<PathArgs, String> {
    let input = PathBuf::from(
        matches
            .value_of_os("INPUT")
            .ok_or_else(|| "Input path must be provided.".to_string())?,
    );

    let output = PathBuf::from(
        matches
            .value_of_os("OUTPUT")
            .ok_or_else(|| "Output path must be provided.".to_string())?,
    );

    if input.is_file() {
        Ok(PathArgs::File { input, output })
    } else if input.is_dir() {
        Ok(PathArgs::Directory { input, output })
    } else {
        Err("Input path be an existing file or directory.".into())
    }
}

fn stringify_and_log(explanation: &str, error: std::io::Error) -> String {
    let error = error.to_string();
    error!("{explanation}: {error}");
    error
}

pub fn convert_svg_to_pico(inp: &Path, _out: &Path) -> Result<(), String> {
    let _inp = File::open(inp)
        .map_err(|e| stringify_and_log("Could not open file", e))?;
    Ok(())
}

pub fn convert_to_pico(path_args: &PathArgs) -> Result<(), String> {
    match path_args {
        PathArgs::File { input, output } => convert_svg_to_pico(input, output),
        PathArgs::Directory { input, .. } => {
            let dir_entries = read_dir(input).map_err(|e| {
                stringify_and_log("Could not open directory", e)
            })?;
            dir_entries.into_iter().try_for_each(|entry| {
                let _ = entry
                    .map_err(|e| {
                        stringify_and_log("Could not open directory entry", e)
                    })?
                    .path();
                // TODO: flesh this out later, for now confirm design
                Ok(())
            })
        }
    }
}

fn main() {
    env_logger::init();

    // version and about are picked up from Cargo.toml by the command! macro
    // angle brackets (e.g. <INPUT>) specifies that it is a required argument
    let matches = command!()
        .arg_required_else_help(true)
        .arg(
            arg!(
                <INPUT> "Input file or directory"
            )
            .allow_invalid_utf8(true),
        )
        .arg_required_else_help(true)
        .arg(
            arg!(
                <OUTPUT> "Output file or directory"
            )
            .allow_invalid_utf8(true),
        )
        .get_matches();

    let path_args = parse_paths(&matches).unwrap();

    println!(
        "input {}: {}",
        path_args.type_as_string(),
        path_args.input().display(),
    );

    convert_to_pico(&path_args).unwrap();
}
