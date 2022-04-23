// General structure taken from Rust CLI book: https://rust-cli.github.io/book/tutorial/testing.html
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::env::current_dir;
use std::fs::{create_dir_all, remove_dir, remove_file, File};
use std::process::Command;

#[test]
fn input_is_file() {
    let mut cmd = Command::cargo_bin("svg-to-pico").unwrap();

    // might want to consider using a crate like `tempfile` in the future?
    let test_dir = current_dir().unwrap().join("input_file_test");
    create_dir_all(&test_dir).unwrap();
    println!(
        "successfully created test dir {}: {}",
        test_dir.exists(),
        test_dir.display()
    );
    let test_file_path = test_dir.join("test_file.tf");
    // create the file; semicolon drops it, closing it
    File::create(&test_file_path).unwrap();
    println!("created temp file at: {}", test_file_path.display());

    cmd.arg(test_file_path.as_os_str());
    cmd.arg("test/file/some/output");
    let result = cmd.assert().try_stdout(predicate::str::contains("input file"));

    remove_file(&test_file_path).unwrap();
    println!("removed test file: {}", !test_file_path.exists());
    remove_dir(&test_dir).unwrap();
    println!("removed test dir: {}", !test_dir.exists());
    result.unwrap();
}

#[test]
fn input_is_dir() {
    let mut cmd = Command::cargo_bin("svg-to-pico").unwrap();

    // might want to consider using a crate like `tempfile` in the future?
    let test_dir = current_dir().unwrap().join("input_dir_test");
    create_dir_all(&test_dir).unwrap();
    println!(
        "successfully created test dir {}: {}",
        test_dir.exists(),
        test_dir.display()
    );

    cmd.arg(test_dir.as_os_str());
    cmd.arg("test/file/some/output");
    let result = cmd.assert()
        .try_stdout(predicate::str::contains("input directory"));

    remove_dir(&test_dir).unwrap();
    println!("removed test dir: {}", !test_dir.exists());
    result.unwrap();
}
