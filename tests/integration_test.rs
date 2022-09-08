use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
/// Test for convert subcommand
/// -g (gfa)
/// default is node
fn test1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfa_annotate")?;
    cmd.arg("-g")
        .arg("data/example_data/testGraph.gfa")
        .arg("-b")
        .arg("data/example_data/test.bed")
        .arg("-o")
        .arg("data/output/test.overlap.txt")
        .arg("-f");
    cmd.assert().success();

    Ok(())
}
