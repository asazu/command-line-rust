use assert_cmd::Command;
use std::fs;
use std::{error::Error, fs::File, io::Read};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TEN: &str = "./tests/inputs/ten.txt";

#[test]
fn dies_bad_bytes() -> TestResult {
    let illegal_number = "bad_bytes";
    let expected = format!("illegal byte count -- {}", illegal_number);
    Command::cargo_bin(PRG)?
        .args(&["-c", illegal_number, "empty"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));
    Ok(())
}

#[test]
fn dies_bad_lines() -> TestResult {
    let illegal_number = "bad_lines";
    let expected = format!("illegal line count -- {}", illegal_number);
    Command::cargo_bin(PRG)?
        .args(&["-n", illegal_number, "empty"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));
    Ok(())
}

#[test]
fn dies_both_bytes_and_number() -> TestResult {
    let msg = "The argument '--lines <LINES>' cannot be used with \
                    '--bytes <BYTES>'";
    Command::cargo_bin(PRG)?
        .args(&["-n", "1", "-c", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(msg));
    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let expected = "blargh: .* [(]os error 2[)]";
    Command::cargo_bin(PRG)?
        .args(&[EMPTY, "blargh", ONE])
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(String::from_utf8(buf)?);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n2() -> TestResult {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

#[test]
fn empty_n4() -> TestResult {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_c2() -> TestResult {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

#[test]
fn empty_c4() -> TestResult {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c2() -> TestResult {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> TestResult {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> TestResult {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> TestResult {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> TestResult {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> TestResult {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn three() -> TestResult {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> TestResult {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> TestResult {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> TestResult {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> TestResult {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn ten() -> TestResult {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2() -> TestResult {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> TestResult {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2() -> TestResult {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> TestResult {
    run(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;

    let mut file = File::open(expected_file)?;
    let mut expected = Vec::new();
    file.read_to_end(&mut expected)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn empty_stdin() -> TestResult {
    run_stdin(EMPTY, &["-"], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n2_stdin() -> TestResult {
    run_stdin(EMPTY, &["-n", "2"], "tests/expected/empty.txt.n2.out")
}

#[test]
fn empty_n4_stdin() -> TestResult {
    run_stdin(EMPTY, &["-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_c2_stdin() -> TestResult {
    run_stdin(EMPTY, &["-c", "2"], "tests/expected/empty.txt.c2.out")
}

#[test]
fn empty_c4_stdin() -> TestResult {
    run_stdin(EMPTY, &["-c", "4"], "tests/expected/empty.txt.c4.out")
}

#[test]
fn one_stdin() -> TestResult {
    run_stdin(ONE, &[], "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> TestResult {
    run_stdin(ONE, &["-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() -> TestResult {
    run_stdin(ONE, &["-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c2_stdin() -> TestResult {
    run_stdin(ONE, &["-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() -> TestResult {
    run_stdin(ONE, &["-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn two_stdin() -> TestResult {
    run_stdin(TWO, &[], "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> TestResult {
    run_stdin(TWO, &["-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() -> TestResult {
    run_stdin(TWO, &["-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() -> TestResult {
    run_stdin(TWO, &["-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() -> TestResult {
    run_stdin(TWO, &["-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn three_stdin() -> TestResult {
    run_stdin(THREE, &[], "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() -> TestResult {
    run_stdin(THREE, &["-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() -> TestResult {
    run_stdin(THREE, &["-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() -> TestResult {
    run_stdin(THREE, &["-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() -> TestResult {
    run_stdin(THREE, &["-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn ten_stdin() -> TestResult {
    run_stdin(TEN, &[], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2_stdin() -> TestResult {
    run_stdin(TEN, &["-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4_stdin() -> TestResult {
    run_stdin(TEN, &["-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2_stdin() -> TestResult {
    run_stdin(TEN, &["-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4_stdin() -> TestResult {
    run_stdin(TEN, &["-c", "4"], "tests/expected/ten.txt.c4.out")
}

#[test]
fn multiple_files() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN], "tests/expected/all.out")?;
    Ok(())
}

#[test]
fn multiple_files_n2() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-n", "2"],
        "tests/expected/all.n2.out",
    )?;
    Ok(())
}

#[test]
fn multiple_files_n4() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-n", "4"],
        "tests/expected/all.n4.out",
    )?;
    Ok(())
}

#[test]
fn multiple_files_c2() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "2"],
        "tests/expected/all.c2.out",
    )?;
    Ok(())
}

#[test]
fn multiple_files_c4() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "4"],
        "tests/expected/all.c4.out",
    )?;
    Ok(())
}
