use std::env::args;
use std::fs;
use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
/// Test for convert subcommand
/// -g (gfa)
/// default is node
fn test_default() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfa_annotate")?;
    cmd.arg("-g")
        .arg("data/example_data/testGraph.gfa")
        .arg("-b")
        .arg("data/example_data/test.bed")
        .arg("-o")
        .arg("data/output/test.overlap.default.txt");
    cmd.assert().success();
    let path = "data/output/test.overlap.default.txt";
    fs::remove_file(path).unwrap();

    Ok(())
}

#[test]
/// Test for convert subcommand
/// -g (gfa)
/// default is node
fn test_fraction() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfa_annotate")?;
    cmd.arg("-g")
        .arg("data/example_data/testGraph.gfa")
        .arg("-b")
        .arg("data/example_data/test.bed")
        .arg("-o")
        .arg("data/output/test.overlap.fraction.txt")
        .arg("-f");
    cmd.assert().success();
    let path = "data/output/test.overlap.fraction.txt";
    fs::remove_file(path).unwrap();

    Ok(())
}

#[test]
/// Test for convert subcommand
/// -g (gfa)
/// default is node
fn test_length() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfa_annotate")?;
    cmd.arg("-g")
        .arg("data/example_data/testGraph.gfa")
        .arg("-b")
        .arg("data/example_data/test.bed")
        .arg("-o")
        .arg("data/output/test.overlap.length.txt")
        .arg("-l");
    cmd.assert().success();
    let path = "data/output/test.overlap.length.txt";
    fs::remove_file(path).unwrap();

    Ok(())
}


#[test]
/// Test for convert subcommand
/// -g (gfa)
/// default is node
fn test_fraction_length() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfa_annotate")?;
    cmd.arg("-g")
        .arg("data/example_data/testGraph.gfa")
        .arg("-b")
        .arg("data/example_data/test.bed")
        .arg("-o")
        .arg("data/output/test.overlap.fraction.length.txt")
        .arg("-f")
        .arg("-l");
    cmd.assert().success();
    let path = "data/output/test.overlap.fraction.length.txt";
    fs::remove_file(path).unwrap();

    Ok(())
}

