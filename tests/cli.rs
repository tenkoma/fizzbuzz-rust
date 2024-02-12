use anyhow::Result;
use std::fs;
use assert_cmd::Command;

const PRG: &str = "fizzbuzz";

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn one() -> Result<()> {
    run(&["1"], "tests/expected/one.txt")
}

#[test]
fn three() -> Result<()> {
    run(&["3"], "tests/expected/three.txt")
}

#[test]
fn fifteen() -> Result<()> {
    run(&["15"], "tests/expected/fifteen.txt")
}

#[test]
fn one_hundred() -> Result<()> {
    run(&["100"], "tests/expected/one_hundred.txt")
}
