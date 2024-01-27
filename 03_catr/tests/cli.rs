use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let file = "blargh";
    let expected = format!{"{}: .* [(]os error 2[)]", file};
    Command::cargo_bin(PRG)?
        .arg(file)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n() -> TestResult {
    for flag in &["-n", "--number"] {
		run(&[EMPTY, flag], "tests/expected/empty.txt.n.out")?
    }
    Ok(())
}

#[test]
fn empty_b() -> TestResult {
    for flag in &["-b", "--number-nonblank"] {
        run(&[EMPTY, flag], "tests/expected/empty.txt.b.out")?
    }
    Ok(())
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> TestResult {
    for flag in &["-n", "--number"] {
		run(&[FOX, flag], "tests/expected/fox.txt.n.out")?
    }
    Ok(())
}

#[test]
fn fox_b() -> TestResult {
    for flag in &["-b", "--number-nonblank"] {
        run(&[FOX, flag], "tests/expected/fox.txt.b.out")?
    }
    Ok(())
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    for flag in &["-n", "--number"] {
		run(&[SPIDERS, flag], "tests/expected/spiders.txt.n.out")?
    }
    Ok(())
}

#[test]
fn spiders_b() -> TestResult {
    for flag in &["-b", "--number-nonblank"] {
        run(&[SPIDERS, flag], "tests/expected/spiders.txt.b.out")?
    }
    Ok(())
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> TestResult {
    for flag in &["-n", "--number"] {
		run(&[BUSTLE, flag], "tests/expected/the-bustle.txt.n.out")?
    }
    Ok(())
}

#[test]
fn bustle_b() -> TestResult {
    for flag in &["-b", "--number-nonblank"] {
        run(&[BUSTLE, flag], "tests/expected/the-bustle.txt.b.out")?
    }
    Ok(())
}

fn run_stdin(
    input_file: &str,
    args: &[&str],
    expected_file: &str
) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE,
        &["-"],
        "tests/expected/the-bustle.txt.stdin.out"
    )
}

#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out"
    )
}

#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(BUSTLE,
        &["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out"
    )
}

#[test]
fn all() -> TestResult {
    run(
        &[FOX, SPIDERS, BUSTLE],
        "tests/expected/all.out"
    )
}

#[test]
fn all_n() -> TestResult {
    run(
        &[FOX, SPIDERS, BUSTLE, "-n"],
        "tests/expected/all.n.out"
    )
}