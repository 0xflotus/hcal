use chrono::{Datelike, NaiveDate, Utc, Weekday};
use clap::{App, Arg};
use std::vec::Vec;

fn main() {
    let matches = App::new("hcal")
        .version("0.1.12")
        .about("A hexadecimal calendar")
        .arg(
            Arg::new("year")
                .about("Sets the year")
                .required(false)
                .index(1_u64),
        )
        .arg(
            Arg::new("month")
                .about("Sets the month")
                .required(false)
                .index(2_u64),
        )
        .arg(
            Arg::new("day")
                .about("Sets the day")
                .required(false)
                .index(3_u64),
        )
        .get_matches();

    if let Some(year) = matches.value_of("year") {
        if let Some(month) = matches.value_of("month") {
            if let Some(day) = matches.value_of("day") {
                hcal(
                    year.parse::<i32>().unwrap(),
                    month.parse::<u32>().unwrap(),
                    day.parse::<u32>().unwrap(),
                    true,
                );
            } else {
                hcal(
                    year.parse::<i32>().unwrap(),
                    month.parse::<u32>().unwrap(),
                    1_u32,
                    false,
                );
            }
        } else {
            println!("Please set a month.");
        }
    } else {
        let now = Utc::now();
        hcal(now.year(), now.month(), now.day(), true);
    }
}

fn hcal(year: i32, month: u32, day: u32, show_day: bool) {
    let now = NaiveDate::from_ymd(year, month, day);
    let (_is_common_era, year) = now.year_ce();
    let month = now.month();
    let day = now.day();
    let start_date = NaiveDate::from_ymd(year as i32, month, 1_u32);
    let days_from_monday = start_date.weekday().num_days_from_monday();
    println!(
        "\t\t{}\n\t\t  {}\n",
        format!("{:#06x}", year),
        format!("{:02x}", month)
    );

    println!("{}", "Mon\tTue\tWed\tThu\tFri\tSat\tSun");
    let mut end = 7_u32 - days_from_monday;
    let mut vec = Vec::new();
    for x in 1_u32..=end {
        let the_day = NaiveDate::from_ymd(year as i32, month, x);
        let weekday = the_day.weekday();
        if x == day && show_day {
            vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
        } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
            vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
        } else {
            vec.push(format!("{:#04x}", x));
        }
    }
    println!(
        "{}{}",
        " \t".repeat(days_from_monday as usize),
        vec.join("\t")
    );
    end = end + 1_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        let the_day = NaiveDate::from_ymd(year as i32, month, x);
        let weekday = the_day.weekday();
        if x == day && show_day {
            vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
        } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
            vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
        } else {
            vec.push(format!("{:#04x}", x));
        }
    }
    println!("{}", vec.join("\t"));
    end = end + 7_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        let the_day = NaiveDate::from_ymd(year as i32, month, x);
        let weekday = the_day.weekday();
        if x == day && show_day {
            vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
        } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
            vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
        } else {
            vec.push(format!("{:#04x}", x));
        }
    }
    println!("{}", vec.join("\t"));
    end = end + 7_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        let the_day = NaiveDate::from_ymd(year as i32, month, x);
        let weekday = the_day.weekday();
        if x == day && show_day {
            vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
        } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
            vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
        } else {
            vec.push(format!("{:#04x}", x));
        }
    }
    println!("{}", vec.join("\t"));
    end = end + 7_u32;
    vec = Vec::new();
    if get_days_from_month(year as i32, month as u32) as u32 - end < 7_u32 {
        for x in end..=get_days_from_month(year as i32, month as u32) as u32 {
            let the_day = NaiveDate::from_ymd(year as i32, month, x);
            let weekday = the_day.weekday();
            if x == day && show_day {
                vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
            } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
                vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
            } else {
                vec.push(format!("{:#04x}", x));
            }
        }
        println!("{}", vec.join("\t"));
    } else {
        for x in end..=end + 6_u32 {
            let the_day = NaiveDate::from_ymd(year as i32, month, x);
            let weekday = the_day.weekday();
            if x == day && show_day {
                vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
            } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
                vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
            } else {
                vec.push(format!("{:#04x}", x));
            }
        }
        println!("{}", vec.join("\t"));
        vec = Vec::new();
        for x in end + 7_u32..=get_days_from_month(year as i32, month as u32) as u32 {
            let the_day = NaiveDate::from_ymd(year as i32, month, x);
            let weekday = the_day.weekday();
            if x == day && show_day {
                vec.push(format!("\u{001b}[41m{:#04x}\u{001b}[0m", x));
            } else if weekday == Weekday::Sat || weekday == Weekday::Sun {
                vec.push(format!("\u{001b}[7m{:#04x}\u{001b}[0m", x));
            } else {
                vec.push(format!("{:#04x}", x));
            }
        }
        println!("{}", vec.join("\t"));
    }
}

// from https://stackoverflow.com/a/58188385
fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12_u32 => year + 1_i32,
            _ => year,
        },
        match month {
            12_u32 => 1_u32,
            _ => month + 1_u32,
        },
        1_u32,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1_u32))
    .num_days()
}
