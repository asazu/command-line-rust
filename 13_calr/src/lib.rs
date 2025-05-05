use chrono::{Datelike, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[arg(value_parser = clap::value_parser!(i32).range(1..=9999))]
    year: Option<i32>,

    #[arg(short, value_parser = month_parser)]
    month: Option<u32>,

    #[arg(short = 'y', long = "year", conflicts_with_all = ["month", "year"])]
    shows_yearly_cal: bool,
}

fn month_parser(s: &str) -> Result<u32, anyhow::Error> {
    match s.parse::<u32>() {
        Ok(m) if (1..=12).contains(&m) => Ok(m),
        Ok(m) => anyhow::bail!("{} is not in 1..=12", m),
        Err(_) => {
            let s = s.to_lowercase();
            let matched = (1..=12)
                .map(|m| Month::from_u32(m).unwrap())
                .filter(|m| m.name().to_lowercase().starts_with(&s))
                .collect::<Vec<_>>();

            if matched.len() == 1 {
                Ok(matched[0].number_from_month())
            } else {
                anyhow::bail!("{} is not a valid month", s)
            }
        }
    }
}

fn format_week(first_day: NaiveDate, month: u32) -> String {
    first_day
        .iter_days()
        .take(7)
        .map(|d| {
            if d.month() == month {
                format!("{:-2} ", d.day())
            } else {
                String::from("   ")
            }
        })
        .collect::<String>()
        + " "
}

fn header_month(month: u32) -> String {
    assert!((1..=12).contains(&month));
    format!("{:^20}  ", Month::from_u32(month).unwrap().name())
}

fn header_month_year(month: u32, year: i32) -> String {
    assert!((1..=12).contains(&month));
    let month_year = format!("{} {}", Month::from_u32(month).unwrap().name(), year);
    format!("{:^20}  ", month_year)
}

fn cal_month_inner(month: u32, year: i32) -> Result<Vec<String>, anyhow::Error> {
    assert!((1..=12).contains(&month));

    let mut result = Vec::with_capacity(7);
    result.push(String::from("Su Mo Tu We Th Fr Sa  "));

    NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| anyhow::anyhow!("out of range date"))?
        .week(Weekday::Sun)
        .first_day()
        .iter_weeks()
        .take(6)
        .for_each(|first_day_of_week| {
            result.push(format_week(first_day_of_week, month));
        });
    Ok(result)
}

fn cal_month(month: u32, year: i32) -> Result<Vec<String>, anyhow::Error> {
    assert!((1..=12).contains(&month));

    let mut result = Vec::with_capacity(8);
    result.push(header_month_year(month, year));
    result.append(&mut cal_month_inner(month, year)?);

    assert!(result.len() == 8);
    Ok(result)
}

fn cal_year(year: i32) -> Result<Vec<String>, anyhow::Error> {
    let mut calender_of_months = Vec::with_capacity(12);
    for month in 1..=12 {
        let mut cal = Vec::with_capacity(8);
        cal.push(header_month(month));
        cal.append(&mut cal_month_inner(month, year)?);
        calender_of_months.push(cal);
    }

    let mut result = Vec::new();
    result.push(format!("{:-32}", year));
    calender_of_months.chunks(3).for_each(|chunk| {
        for i in 0..chunk[0].len() {
            result.push((0..3).map(|j| chunk[j][i].as_str()).collect());
        }
        result.push(String::from(""));
    });
    result.pop(); // Remove the last empty line

    Ok(result)
}

pub fn run(args: Args) -> Result<(), anyhow::Error> {
    eprintln!("{:#?}\n", args);

    let today = chrono::Local::now();

    let year = match args.year {
        Some(y) => y,
        None => today.year(),
    };

    let month = match args.month {
        Some(m) => m,
        None => today.month(),
    };

    let shows_yearly_cal = args.shows_yearly_cal || (args.month.is_none() && args.year.is_some());

    let cal = if shows_yearly_cal {
        cal_year(year)?
    } else {
        cal_month(month, year)?
    };

    for line in cal {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[test]
    fn format_week() {
        let first_day = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();
        assert_eq!("    1  2  3  4  5  6  ", super::format_week(first_day, 4));
    }

    #[test]
    fn cal_month() {
        let expected = vec![
            String::from("     April 2024       "),
            String::from("Su Mo Tu We Th Fr Sa  "),
            String::from("    1  2  3  4  5  6  "),
            String::from(" 7  8  9 10 11 12 13  "),
            String::from("14 15 16 17 18 19 20  "),
            String::from("21 22 23 24 25 26 27  "),
            String::from("28 29 30              "),
            String::from("                      "),
        ];
        assert_eq!(expected, super::cal_month(4, 2024).unwrap());
    }
}
