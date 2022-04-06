mod path_arg;

use crate::path_arg::PathArg;
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

    let input: PathArg = if let Some(inp) = matches.value_of_os("input") {
        inp.into()
    } else {
        unreachable!("Input required, otherwise help should be printed.")
    };

    let output: PathArg = if let Some(out) = matches.value_of_os("output") {
        out.into()
    } else {
        unreachable!("Output required, otherwise help should be printed.")
    };

    println!("input {}: {}", input.type_as_string(), input);
    println!("expected output {}: {}", output.type_as_string(), output);
}
