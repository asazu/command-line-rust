use std::fs;
use std::io;
use std::str::Lines;

fn default_line_counter(_: i32, _: &str) -> i32 { 0 }
fn normal_line_counter(counter: i32, _: &str) -> i32 { counter + 1 }
fn nonblank_line_counter(counter: i32, line: &str) -> i32 {
    if line.is_empty() { counter } else { counter + 1 }
}

fn default_formatter(_: i32, line: &str) -> String {
    format!{"{}", line}
}

fn line_number_formatter(line_nmber: i32, line: &str) -> String {
    format!{"{:6}\t{}", line_nmber, line}
}

fn nonblank_line_number_formatter(line_number: i32, line: &str) -> String {
    if line.is_empty() {
        default_formatter(line_number, line)
    } else {
        line_number_formatter(line_number, line)
    }
}

fn cat(
    lines: Vec<&str>,
    counter: fn(i32, &str) -> i32,
    formatter: fn(i32, &str) -> String,
) {
    let mut line_count = 0;
    for l in lines {
        line_count = counter(line_count, l);
        println!("{}", formatter(line_count, l));
    }
}

fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }

//     let mut counter: fn(i32, &str) -> i32 = default_line_counter;
//     let mut formatter: fn(i32, &str) -> String = default_formatter;

//    if matches.is_present("numbers-nonblank") {
//         counter = nonblank_line_counter;
//         formatter = nonblank_line_number_formatter;
//     }
//    if matches.is_present("numbers") {
//         counter = normal_line_counter;
//         formatter = line_number_formatter;
//     }

//     if file == "-" {
//         // let lines = io::stdin().lines()
//         //     .map(|x| x.unwrap())
//         //     .collect::<Vec<&str>>();
//         // cat(lines, counter, formatter);
//     } else {
        
//         let f = fs::File::open(file).unwrap();
//         // let reader = io::BufReader::new(f).read_line();
//         match fs::read_to_string(file) {
//             Ok(input) => cat(input.lines().collect::<Vec<_>>(), counter, formatter),
//             Err(err) => eprintln!("{}: {}", file, err),
//         }
//     };
}
