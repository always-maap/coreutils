use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn kill_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn test1() -> TestResult {
    run(&["Never gonna give you up"], "tests/expected/test1.txt")
}

#[test]
fn test2() -> TestResult {
    run(&["hello", "world"], "tests/expected/test2.txt")
}

#[test]
fn test1_no_newline() -> TestResult {
    run(
        &["Never gonna give you up", "-n"],
        "tests/expected/test1.n.txt",
    )
}

#[test]
fn test2_no_newline() -> TestResult {
    run(&["-n", "hello", "world"], "tests/expected/test2.n.txt")
}
