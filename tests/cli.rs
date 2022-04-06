// General structure taken from Rust CLI book: https://rust-cli.github.io/book/tutorial/testing.html
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::env::current_dir;
use std::fs::{create_dir_all, remove_dir, remove_file, File};
use std::process::Command;

#[test]
fn input_is_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("svg-to-pico")?;

    // might want to consider using a crate like `tempfile` in the future?
    let test_dir = current_dir()?.join("input_file_test");
    create_dir_all(&test_dir)?;
    println!(
        "successfully created test dir {}: {}",
        test_dir.exists(),
        test_dir.display()
    );
    let test_file_path = test_dir.join("test_file.tf");
    // create the file; semicolon drops it, closing it
    File::create(&test_file_path)?;
    println!("created temp file at: {}", test_file_path.display());

    cmd.arg("-i").arg(test_file_path.as_os_str());
    cmd.arg("-o").arg("test/file/some/output");
    cmd.assert().stdout(predicate::str::contains("input file"));
    println!("correctly detected input test file as file");

    remove_file(&test_file_path)?;
    println!("removed test file: {}", !test_file_path.exists());
    remove_dir(&test_dir)?;
    println!("removed test dir: {}", !test_dir.exists());
    Ok(())
}

#[test]
fn input_is_dir() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("svg-to-pico")?;

    // might want to consider using a crate like `tempfile` in the future?
    let test_dir = current_dir()?.join("input_dir_test");
    create_dir_all(&test_dir)?;
    println!(
        "successfully created test dir {}: {}",
        test_dir.exists(),
        test_dir.display()
    );

    cmd.arg("-i").arg(test_dir.as_os_str());
    cmd.arg("-o").arg("test/file/some/output");
    cmd.assert()
        .stdout(predicate::str::contains("input directory"));
    println!("correctly detected input test file as directory");

    remove_dir(&test_dir)?;
    println!("removed test dir: {}", !test_dir.exists());
    Ok(())
}
