use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

const PRG: &str = "uniqr";

#[test]
fn dies_bad_file() -> Result<()> {
    let bad_file = "blargh";
    let expected = format! {"{}: .* [(]os error 2[)]", bad_file};
    Command::cargo_bin(PRG)?
        .arg(bad_file)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

const OUT_SUFFIX: &str = ".out";
const STDIN_SUFFIX: &str = ".stdin";
const COUNT_SUFFIX: &str = ".c";

struct Test {
    input: &'static str,
    expected: &'static str,
}

fn run(test: &Test) -> Result<()> {
    let expected = format!("{}{}", test.expected, OUT_SUFFIX);
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .arg(test.input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_count(test: &Test) -> Result<()> {
    let expected = format!("{}{}{}", test.expected, COUNT_SUFFIX, OUT_SUFFIX);
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .args(&["-c", test.input])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_stdin(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = format!("{}{}{}", test.expected, STDIN_SUFFIX, OUT_SUFFIX);
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_stdin_count(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = format!(
        "{}{}{}{}",
        test.expected, STDIN_SUFFIX, COUNT_SUFFIX, OUT_SUFFIX
    );
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .arg("-c")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_outfile(test: &Test) -> Result<()> {
    let output = NamedTempFile::new()?;
    let outpath = output.path().to_str().unwrap();
    let expected = format!("{}{}", test.expected, OUT_SUFFIX);
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .args(&[test.input, outpath])
        .assert()
        .success()
        .stdout("");

    assert_eq!(expected, fs::read_to_string(outpath)?);
    Ok(())
}

fn run_outfile_count(test: &Test) -> Result<()> {
    let output = NamedTempFile::new()?;
    let output = output.path().to_str().unwrap();
    let expected = format!("{}{}{}", test.expected, COUNT_SUFFIX, OUT_SUFFIX);
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .args(&["-c", test.input, output])
        .assert()
        .success()
        .stdout("");

    assert_eq!(expected, fs::read_to_string(output)?);
    Ok(())
}

fn run_stdin_outfile_count(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let output = NamedTempFile::new()?;
    let output = output.path().to_str().unwrap();
    let expected = format!(
        "{}{}{}{}",
        test.expected, STDIN_SUFFIX, COUNT_SUFFIX, OUT_SUFFIX
    );
    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .args(&["-c", "-", output])
        .write_stdin(input)
        .assert()
        .success()
        .stdout("");

    assert_eq!(expected, fs::read_to_string(output)?);
    Ok(())
}

const EMPTY: Test = Test {
    input: "tests/inputs/empty.txt",
    expected: "tests/expected/empty.txt",
};

#[test]
fn empty() -> Result<()> {
    run(&EMPTY)
}

#[test]
fn empty_count() -> Result<()> {
    run_count(&EMPTY)
}

#[test]
fn empty_stdin() -> Result<()> {
    run_stdin(&EMPTY)
}

#[test]
fn empty_stdout_count() -> Result<()> {
    run_stdin_count(&EMPTY)
}

#[test]
fn empty_outfile() -> Result<()> {
    run_outfile(&EMPTY)
}

#[test]
fn empty_outfile_count() -> Result<()> {
    run_outfile_count(&EMPTY)
}

#[test]
fn empty_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&EMPTY)
}

const ONE: Test = Test {
    input: "tests/inputs/one.txt",
    expected: "tests/expected/one.txt",
};

#[test]
fn one() -> Result<()> {
    run(&ONE)
}

#[test]
fn one_count() -> Result<()> {
    run_count(&ONE)
}

#[test]
fn one_stdin() -> Result<()> {
    run_stdin(&ONE)
}

#[test]
fn one_stdout_count() -> Result<()> {
    run_stdin_count(&ONE)
}

#[test]
fn one_outfile() -> Result<()> {
    run_outfile(&ONE)
}

#[test]
fn one_outfile_count() -> Result<()> {
    run_outfile_count(&ONE)
}

#[test]
fn one_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&ONE)
}

const TWO: Test = Test {
    input: "tests/inputs/two.txt",
    expected: "tests/expected/two.txt",
};

#[test]
fn two() -> Result<()> {
    run(&TWO)
}

#[test]
fn two_count() -> Result<()> {
    run_count(&TWO)
}

#[test]
fn two_stdin() -> Result<()> {
    run_stdin(&TWO)
}

#[test]
fn two_stdout_count() -> Result<()> {
    run_stdin_count(&TWO)
}

const THREE: Test = Test {
    input: "tests/inputs/three.txt",
    expected: "tests/expected/three.txt",
};

#[test]
fn three() -> Result<()> {
    run(&THREE)
}

#[test]
fn three_count() -> Result<()> {
    run_count(&THREE)
}

#[test]
fn three_stdin() -> Result<()> {
    run_stdin(&THREE)
}

#[test]
fn three_stdout_count() -> Result<()> {
    run_stdin_count(&THREE)
}

const T1: Test = Test {
    input: "tests/inputs/t1.txt",
    expected: "tests/expected/t1.txt",
};

#[test]
fn t1() -> Result<()> {
    run(&T1)
}

#[test]
fn t1_count() -> Result<()> {
    run_count(&T1)
}

#[test]
fn t1_stdin() -> Result<()> {
    run_stdin(&T1)
}

#[test]
fn t1_stdout_count() -> Result<()> {
    run_stdin_count(&T1)
}

const T2: Test = Test {
    input: "tests/inputs/t2.txt",
    expected: "tests/expected/t2.txt",
};

#[test]
fn t2() -> Result<()> {
    run(&T2)
}

#[test]
fn t2_count() -> Result<()> {
    run_count(&T2)
}

#[test]
fn t2_stdin() -> Result<()> {
    run_stdin(&T2)
}

#[test]
fn t2_stdout_count() -> Result<()> {
    run_stdin_count(&T2)
}

const T3: Test = Test {
    input: "tests/inputs/t3.txt",
    expected: "tests/expected/t3.txt",
};

#[test]
fn t3() -> Result<()> {
    run(&T3)
}

#[test]
fn t3_count() -> Result<()> {
    run_count(&T3)
}

#[test]
fn t3_stdin() -> Result<()> {
    run_stdin(&T3)
}

#[test]
fn t3_stdout_count() -> Result<()> {
    run_stdin_count(&T3)
}

const T4: Test = Test {
    input: "tests/inputs/t4.txt",
    expected: "tests/expected/t4.txt",
};

#[test]
fn t4() -> Result<()> {
    run(&T4)
}

#[test]
fn t4_count() -> Result<()> {
    run_count(&T4)
}

#[test]
fn t4_stdin() -> Result<()> {
    run_stdin(&T4)
}

#[test]
fn t4_stdout_count() -> Result<()> {
    run_stdin_count(&T4)
}

const T5: Test = Test {
    input: "tests/inputs/t5.txt",
    expected: "tests/expected/t5.txt",
};

#[test]
fn t5() -> Result<()> {
    run(&T5)
}

#[test]
fn t5_count() -> Result<()> {
    run_count(&T5)
}

#[test]
fn t5_stdin() -> Result<()> {
    run_stdin(&T5)
}

#[test]
fn t5_stdout_count() -> Result<()> {
    run_stdin_count(&T5)
}

const T6: Test = Test {
    input: "tests/inputs/t6.txt",
    expected: "tests/expected/t6.txt",
};

#[test]
fn t6() -> Result<()> {
    run(&T6)
}

#[test]
fn t6_count() -> Result<()> {
    run_count(&T6)
}

#[test]
fn t6_stdin() -> Result<()> {
    run_stdin(&T6)
}

#[test]
fn t6_stdout_count() -> Result<()> {
    run_stdin_count(&T6)
}
