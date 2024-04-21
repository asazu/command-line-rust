use super::Config;
use super::Count;
use super::MyResult;

fn num_of_digits(u: usize) -> usize {
    u.checked_ilog10().unwrap_or_default() as usize + 1
}

fn first_value(count: &Count, config: &Config) -> usize {
    if config.opt_lines {
        return count.lines;
    };
    if config.opt_words {
        return count.words;
    };
    if config.opt_chars {
        return count.chars;
    };
    if config.opt_bytes {
        return count.bytes;
    };
    panic!("no option enabled");
}

fn max_digits(counts: &[MyResult<Count>], config: &Config) -> usize {
    let valid_counts = counts
        .iter()
        .filter_map(|count| count.as_ref().ok())
        .collect::<Vec<_>>();

    if config.single_opt() && valid_counts.len() == 1 {
        return num_of_digits(first_value(valid_counts[0], config));
    }

    let max_value = valid_counts
        .iter()
        .map(|count| count.bytes)
        .max()
        .unwrap_or_default();

    num_of_digits(max_value)
}

fn format(label: Option<&str>, count: &Count, max_digits: usize, config: &Config) -> String {
    let mut columns = Vec::new();

    if config.opt_lines {
        let str = format!("{:max_digits$}", count.lines);
        columns.push(str);
    };
    if config.opt_words {
        let str = format!("{:max_digits$}", count.words);
        columns.push(str);
    };
    if config.opt_chars {
        let str = format!("{:max_digits$}", count.chars);
        columns.push(str);
    };
    if config.opt_bytes {
        let str = format!("{:max_digits$}", count.bytes);
        columns.push(str);
    };
    if let Some(label) = label {
        columns.push(label.to_owned());
    };
    columns.join(" ")
}

pub fn print_counts(counts: &[MyResult<Count>], total: &Count, config: &Config) {
    let max_digits = max_digits(counts, config);

    counts
        .iter()
        .zip(&config.files)
        .for_each(|(count, file_name)| match count {
            Ok(count) => {
                let line = format(file_name.name(), count, max_digits, config);
                println!("{line}");
            }
            Err(e) => {
                if let Some(path) = file_name.name() {
                    eprintln!("{path}: {e}");
                } else {
                    eprintln!("{e}");
                }
            }
        });

    if counts.len() > 1 {
        let label = Some("total");
        let total = format(label, total, max_digits, config);
        println!("{total}");
    }
}
