use chrono::{Datelike, NaiveDate, Utc};
use clap::{App, Arg};
use std::vec::Vec;

fn main() {
    let matches = App::new("hcal")
        .version("0.1.0")
        .author("0xflotus")
        .about("A hexadecimal calendar")
        .arg(
            Arg::new("year")
                .about("Sets the year")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("month")
                .about("Sets the month")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("day")
                .about("Sets the day")
                .required(false)
                .index(3),
        )
        .get_matches();

    if let Some(year) = matches.value_of("year") {
        if let Some(month) = matches.value_of("month") {
            if let Some(day) = matches.value_of("day") {
                println!(
                    "{}-{}-{}",
                    format!("{:#06x}", year.parse::<i32>().unwrap()),
                    format!("{:02x}", month.parse::<i32>().unwrap()),
                    format!("{:02x}", day.parse::<i32>().unwrap())
                );
            } else {
                println!("Please provide a day.");
            }
        } else {
            println!("Please provide a month.");
        }
    } else {
        let now = Utc::now();
        let (_is_common_era, year) = now.year_ce();
        let month = now.month();
        let day = now.day();
        let start_date = NaiveDate::from_ymd(year as i32, month, 1);
        let days_from_monday = start_date.weekday().num_days_from_monday();
        println!(
            "{}-{}-{}",
            format!("{:#06x}", year),
            format!("{:02x}", month),
            format!("{:02x}", day)
        );
        println!(
            "{}-{}-{}",
            format!("{}", year),
            format!("{}", month),
            format!("{}", day)
        );

        println!("weekday: {}", start_date.weekday().num_days_from_monday());

        println!("{}", "Mon\tTue\tWed\tThu\tFri\tSat\tSun");
        let mut end = 7 - days_from_monday;
        let mut vec = Vec::new();
        for x in 1..=end {
            vec.push(format!("{}", x));
        }
        println!(
            "{}{}",
            " \t".repeat(days_from_monday as usize),
            vec.join("\t")
        );
        end = end + 1;
        vec = Vec::new();
        for x in end + 1..=end + 7 {
            vec.push(format!("{}", x));
        }
        println!("{}", vec.join("\t"));
        end = end + 8;
        vec = Vec::new();
        for x in end + 1..=end + 7 {
            vec.push(format!("{}", x));
        }
        println!("{}", vec.join("\t"));
        end = end + 8;
        vec = Vec::new();
        for x in end + 1..=end + 7 {
            vec.push(format!("{}", x));
        }
        println!("{}", vec.join("\t"));
        end = end + 8;
        vec = Vec::new();
        for x in end + 1..=get_days_from_month(year as i32, month as u32) as u32 {
            vec.push(format!("{}", x));
        }
        println!("{}", vec.join("\t"));
    }
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}
