use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "wcr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

#[test]
fn dies_chars_and_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["-c", "-m"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--bytes' cannot be used with '--chars'",
        ));
    Ok(())
}

#[test]
fn skip_bad_files() -> TestResult {
    let bad_file = "blargh";
    let expected = format! {"{}: .* [(]os error 2[)]", bad_file};
    Command::cargo_bin(PRG)?
        .arg(bad_file)
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
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_bytes() -> TestResult {
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

#[test]
fn fox_chars() -> TestResult {
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

#[test]
fn fox_words() -> TestResult {
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

#[test]
fn fox_lines() -> TestResult {
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

#[test]
fn fox_words_bytes() -> TestResult {
    run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
}

#[test]
fn fox_words_lines() -> TestResult {
    run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
}

#[test]
fn fox_bytes_line() -> TestResult {
    run(&["-c", "-l", FOX], "tests/expected/fox.txt.cl.out")
}

#[test]
fn atlamal() -> TestResult {
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

#[test]
fn atlamal_bytes() -> TestResult {
    run(&["--bytes", ATLAMAL], "tests/expected/atlamal.txt.c.out")
}

#[test]
fn atlamal_chars() -> TestResult {
    run(&["--chars", ATLAMAL], "tests/expected/atlamal.txt.m.out")
}

#[test]
fn atlamal_words() -> TestResult {
    run(&["--words", ATLAMAL], "tests/expected/atlamal.txt.w.out")
}

#[test]
fn atlamal_lines() -> TestResult {
    run(&["--lines", ATLAMAL], "tests/expected/atlamal.txt.l.out")
}

#[test]
fn atlamal_words_bytes() -> TestResult {
    run(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
}

#[test]
fn atlamal_words_lines() -> TestResult {
    run(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
}

#[test]
fn atlamal_bytes_lines() -> TestResult {
    run(&["-c", "-l", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
}

#[test]
fn all() -> TestResult {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

#[test]
fn all_bytes() -> TestResult {
    run(
        &["--bytes", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.c.out",
    )
}

#[test]
fn all_chars() -> TestResult {
    run(
        &["--chars", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.m.out",
    )
}

#[test]
fn all_words() -> TestResult {
    run(
        &["--words", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.w.out",
    )
}

#[test]
fn all_lines() -> TestResult {
    run(
        &["--lines", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.l.out",
    )
}

#[test]
fn all_words_bytes() -> TestResult {
    run(
        &["-w", "-c", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.wc.out",
    )
}

#[test]
fn all_words_lines() -> TestResult {
    run(
        &["-w", "-l", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.wl.out",
    )
}

#[test]
fn all_bytes_lines() -> TestResult {
    run(
        &["-c", "-l", EMPTY, FOX, ATLAMAL],
        "tests/expected/all.cl.out",
    )
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
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
fn atlamal_stdin() -> TestResult {
    run_stdin(ATLAMAL, &[], "tests/expected/atlamal.txt.stdin.out")
}
