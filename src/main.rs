use std::path::Path;
use clap::{arg, command};

fn main() {
    // version and about are picked up from Cargo.toml by the command! macro
    let matches = command!()
        .arg_required_else_help(true)
        .arg(
            arg!(
                -i --input <INPUT> "Input file or directory"
            )
                .allow_invalid_utf8(true),
        )
        .arg_required_else_help(true)
        .arg(
            arg!(
                -o --output <OUTPUT> "Output directory"
            )
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let input = if let Some(inp) = matches.value_of_os("input") {
        Path::new(inp)
    } else {
        unreachable!("Input required, otherwise help should be printed.")
    };

    let output = if let Some(out) = matches.value_of_os("output") {
        Path::new(out)
    } else {
        unreachable!("Output required, otherwise help should be printed.")
    };

    println!("input: {}", input.display());
    println!("output: {}", output.display());
}
