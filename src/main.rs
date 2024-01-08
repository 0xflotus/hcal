extern crate clap;

use bdays::easter;
use chrono::{Datelike, NaiveDate, Utc};
use clap::{Arg, Command};
use regex::Regex;
use std::process;
use std::vec::Vec;

use cbb::util::cbb::{
    int_to_bal_ternary, int_to_unbal_ternary,
};

mod helpers;
use helpers::{cal, fmt, hex};

fn main() {
    let matches = Command::new("hcal")
        .version("0.3.5")
        .about("A hexadecimal calendar for terminal")
        .arg(
            Arg::new("easter")
                .short('e')
                .long("easter")
                .takes_value(true)
                .value_name("year")
                .help("Prints the Hex Date of easter."),
        )
        .arg(
            Arg::new("transform")
                .short('T')
                .long("transform")
                .takes_value(true)
                .value_name("date")
                .help("Prints the Hex Date of <date>. Needs format of dd-mm-yyyy."),
        )
        .arg(
            Arg::new("balanced-ternary")
                .short('b')
                .long("balanced-ternary")
                .help("Use balanced ternary representation"),
        )
        .arg(Arg::new("unbalanced-ternary")
                .short('3')
                .long("unbalanced-ternary")
                .help("Use ternary representation"),
        )
        .arg(
            Arg::new("disable")
                .short('d')
                .long("disable")
                .help("Disable day marker"),
        )
        .arg(
            Arg::new("no-weekend")
                .short('W')
                .long("no-weekend")
                .help("Disable weekend marker"),
        )
        .arg(
            Arg::new("effect")
                .short('E')
                .long("effect")
                .help("Enable title font effects"),
        )
        .arg(
            Arg::new("disable-all")
                .short('A')
                .long("disbale-all")
                .help("Disable all font effects"),
        )
        .arg(Arg::new("disable-year-month")
                .short('Y')
                .long("disable-year-month")
                .help("Don't print year and month"),
        )
        .arg(
            Arg::new("year")
                .help("Sets the year")
                .required(false)
                .index(1_usize),
        )
        .arg(
            Arg::new("month")
                .help("Sets the month")
                .required(false)
                .index(2_usize),
        )
        .arg(
            Arg::new("day")
                .help("Sets the day")
                .required(false)
                .index(3_usize),
        )
        .get_matches();

    if let Some(easter) = matches.value_of("easter") {
        let easter_date = easter::easter_naive_date(
            easter.parse::<i32>().unwrap_or_else(|_| {
                println!("Error while parsing year");
                process::exit(1_i32);
            }) as i32,
        )
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(1_i32);
        });
        println!(
            "{}",
            fmt::format_date(
                (
                    easter_date.year(),
                    easter_date.month(),
                    easter_date.day()
                ),
                matches.is_present("balanced-ternary"),
                matches.is_present("unbalanced-ternary"),
            )
        );
        process::exit(0_i32);
    }

    if let Some(date) = matches.value_of("transform") {
        let re = Regex::new(r"^\d{2}-\d{2}-\d{4}$")
            .unwrap_or_else(|_| {
                println!("Error while parsing date");
                process::exit(1_i32);
            });
        if re.is_match(date) {
            let splitted_date: Vec<&str> =
                date.split('-').collect();
            let year = splitted_date[2]
                .parse::<i32>()
                .unwrap_or_else(|_| {
                    println!("Error while parsing year");
                    process::exit(1_i32);
                });
            let month = splitted_date[1]
                .parse::<u32>()
                .unwrap_or_else(|_| {
                    println!("Error while parsing month");
                    process::exit(1_i32);
                });
            let day = splitted_date[0]
                .parse::<u32>()
                .unwrap_or_else(|_| {
                    println!("Error while parsing day");
                    process::exit(1_i32);
                });
            println!(
                "{}",
                fmt::format_date(
                    (year, month, day),
                    matches.is_present("balanced-ternary"),
                    matches
                        .is_present("unbalanced-ternary"),
                )
            );
            process::exit(0_i32);
        } else {
            println!("Wrong format. Please refer to the right format: dd-mm-yyyy");
            process::exit(1_i32);
        }
    }

    let show_day_marker = !(matches
        .is_present("disable-all")
        || matches.is_present("disable"));
    let show_weekend_marker = !(matches
        .is_present("disable-all")
        || matches.is_present("no-weekend"));
    let show_title_font_effect = !matches
        .is_present("disable-all")
        && matches.is_present("effect");
    let show_year_month =
        !matches.is_present("disable-year-month");
    let balanced_ternary_flag =
        matches.is_present("balanced-ternary");
    let mut unbalanced_ternary_flag =
        matches.is_present("unbalanced-ternary");

    if balanced_ternary_flag {
        // balanced ternary has precedence.
        unbalanced_ternary_flag = false;
    }

    if let Some(year) = matches.value_of("year") {
        if let Some(month) = matches.value_of("month") {
            if let Some(day) = matches.value_of("day") {
                if [day, month, year]
                    .iter()
                    .all(|&elem| elem.starts_with("0x"))
                {
                    hcal(
            hex::trim_and_parse_hex(year).unwrap_or_else(
              |_| {
                println!("Error while parsing year");
                process::exit(1_i32);
              },
            ) as i32,
            hex::trim_and_parse_hex(month).unwrap_or_else(
              |_| {
                println!("Error while parsing month");
                process::exit(1_i32);
              },
            ) as u32,
            hex::trim_and_parse_hex(day).unwrap_or_else(
              |_| {
                println!("Error while parsing day");
                process::exit(1_i32);
              },
            ) as u32,
            show_day_marker,
            show_weekend_marker,
            show_title_font_effect,
            balanced_ternary_flag,
            unbalanced_ternary_flag,
            show_year_month,
          );
                } else {
                    let i32_year = year
                        .parse::<i32>()
                        .unwrap_or_else(|_| {
                            println!(
                                "Error while parsing year"
                            );
                            process::exit(1_i32);
                        })
                        as i32;
                    let u32_month = month
                        .parse::<u32>()
                        .unwrap_or_else(|_| {
                            println!(
                                "Error while parsing month"
                            );
                            process::exit(1_i32);
                        })
                        as u32;
                    let u32_day = day
                        .parse::<u32>()
                        .unwrap_or_else(|_| {
                            println!(
                                "Error while parsing day"
                            );
                            process::exit(1_i32);
                        })
                        as u32;
                    hcal(
                        i32_year,
                        u32_month,
                        u32_day,
                        show_day_marker,
                        show_weekend_marker,
                        show_title_font_effect,
                        balanced_ternary_flag,
                        unbalanced_ternary_flag,
                        show_year_month,
                    );
                }
            } else if [month, year]
                .iter()
                .all(|&elem| elem.starts_with("0x"))
            {
                hcal(
                    hex::trim_and_parse_hex(year)
                        .unwrap_or_else(|_| {
                            println!(
                                "Error while parsing year"
                            );
                            process::exit(1_i32);
                        }) as i32,
                    hex::trim_and_parse_hex(month)
                        .unwrap_or_else(|_| {
                            println!(
                                "Error while parsing month"
                            );
                            process::exit(1_i32);
                        }) as u32,
                    1_u32,
                    false,
                    show_weekend_marker,
                    show_title_font_effect,
                    balanced_ternary_flag,
                    unbalanced_ternary_flag,
                    show_year_month,
                );
            } else {
                let i32_year = year
                    .parse::<i32>()
                    .unwrap_or_else(|_| {
                        println!(
                            "Error while parsing year"
                        );
                        process::exit(1_i32);
                    })
                    as i32;
                let u32_month = month
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        println!(
                            "Error while parsing month"
                        );
                        process::exit(1_i32);
                    })
                    as u32;
                hcal(
                    i32_year,
                    u32_month,
                    1_u32,
                    false,
                    show_weekend_marker,
                    show_title_font_effect,
                    balanced_ternary_flag,
                    unbalanced_ternary_flag,
                    show_year_month,
                );
            }
        } else {
            println!("Please set a month.");
        }
    } else {
        let now = Utc::now();
        hcal(
            now.year(),
            now.month(),
            now.day(),
            show_day_marker,
            show_weekend_marker,
            show_title_font_effect,
            balanced_ternary_flag,
            unbalanced_ternary_flag,
            show_year_month,
        );
    }
}

fn hcal(
    year: i32,
    month: u32,
    day: u32,
    show_day: bool,
    show_weekend: bool,
    effect: bool,
    balanced_ternary: bool,
    unbalanced_ternary: bool,
    show_year_month: bool,
) {
    let now =
        NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let days_from_monday =
        NaiveDate::from_ymd_opt(year as i32, month, 1_u32)
            .unwrap()
            .weekday()
            .num_days_from_monday();

    if show_year_month {
        if balanced_ternary {
            println!(
                "\t\t\t{}\n\t\t\t  {}\n",
                int_to_bal_ternary(year as i128),
                int_to_bal_ternary(month as i128)
            );
        } else if unbalanced_ternary {
            println!(
                "\t\t\t{}\n\t\t\t  {}\n",
                int_to_unbal_ternary(year as i128),
                int_to_unbal_ternary(month as i128)
            );
        } else {
            println!(
                "\t\t\t{}\n\t\t\t  {}\n",
                format!("{:#06x}", year),
                format!("{:02x}", month)
            );
        }
    }

    println!("{}", headline(effect));
    let mut end = 7_u32 - days_from_monday;
    let mut vec = Vec::new();
    for x in 1_u32..=end {
        vec.push(mark(
            (year as i32, month, x),
            day,
            show_day,
            show_weekend,
            balanced_ternary,
            unbalanced_ternary,
        ));
    }
    println!(
        "{}{}",
        "\t".repeat(days_from_monday as usize),
        vec.join("\t")
    );
    end += 1_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        vec.push(mark(
            (year as i32, month, x),
            day,
            show_day,
            show_weekend,
            balanced_ternary,
            unbalanced_ternary,
        ));
    }
    println!("{}", vec.join("\t"));
    end += 7_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        vec.push(mark(
            (year as i32, month, x),
            day,
            show_day,
            show_weekend,
            balanced_ternary,
            unbalanced_ternary,
        ));
    }
    println!("{}", vec.join("\t"));
    end += 7_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        vec.push(mark(
            (year as i32, month, x),
            day,
            show_day,
            show_weekend,
            balanced_ternary,
            unbalanced_ternary,
        ));
    }
    println!("{}", vec.join("\t"));
    end += 7_u32;
    vec = Vec::new();
    if cal::get_days_from_month(year as i32, month as u32)
        as u32
        - end
        < 7_u32
    {
        for x in end..=cal::get_days_from_month(
            year as i32,
            month as u32,
        ) as u32
        {
            vec.push(mark(
                (year as i32, month, x),
                day,
                show_day,
                show_weekend,
                balanced_ternary,
                unbalanced_ternary,
            ));
        }
        println!("{}", vec.join("\t"));
    } else {
        for x in end..=end + 6_u32 {
            vec.push(mark(
                (year as i32, month, x),
                day,
                show_day,
                show_weekend,
                balanced_ternary,
                unbalanced_ternary,
            ));
        }
        println!("{}", vec.join("\t"));
        vec = Vec::new();
        for x in end + 7_u32
            ..=cal::get_days_from_month(
                year as i32,
                month as u32,
            ) as u32
        {
            vec.push(mark(
                (year as i32, month, x),
                day,
                show_day,
                show_weekend,
                balanced_ternary,
                unbalanced_ternary,
            ));
        }
        println!("{}", vec.join("\t"));
    }
}

fn headline(effect: bool) -> &'static str {
    return if effect {
        "\u{001b}[1m\u{001b}[4mMon\tTue\tWed\tThu\tFri\tSat\tSun\u{001b}[0m"
    } else {
        "Mon\tTue\tWed\tThu\tFri\tSat\tSun"
    };
}

fn mark(
    date_tuple: (i32, u32, u32),
    day: u32,
    show_day: bool,
    show_weekend: bool,
    balanced_ternary: bool,
    unbalanced_ternary: bool,
) -> std::string::String {
    use chrono::Weekday;
    let the_day = NaiveDate::from_ymd_opt(
        date_tuple.0,
        date_tuple.1,
        date_tuple.2,
    )
    .unwrap();
    let weekday = the_day.weekday();
    if date_tuple.2 == day && show_day {
        if balanced_ternary {
            return format!(
                "\u{001b}[41m{}\u{001b}[0m",
                int_to_bal_ternary(date_tuple.2 as i128)
            );
        } else if unbalanced_ternary {
            return format!(
                "\u{001b}[41m{}\u{001b}[0m",
                int_to_unbal_ternary(date_tuple.2 as i128)
            );
        } else {
            return format!(
                "\u{001b}[41m{:#04x}\u{001b}[0m",
                date_tuple.2
            );
        }
    } else if [Weekday::Sat, Weekday::Sun]
        .contains(&weekday)
        && show_weekend
    {
        if balanced_ternary {
            return format!(
                "\u{001b}[7m{}\u{001b}[0m",
                int_to_bal_ternary(date_tuple.2 as i128)
            );
        } else if unbalanced_ternary {
            return format!(
                "\u{001b}[7m{}\u{001b}[0m",
                int_to_unbal_ternary(date_tuple.2 as i128)
            );
        } else {
            return format!(
                "\u{001b}[7m{:#04x}\u{001b}[0m",
                date_tuple.2
            );
        }
    } else {
        if balanced_ternary {
            return format!(
                "{}",
                int_to_bal_ternary(date_tuple.2 as i128)
            );
        } else if unbalanced_ternary {
            return format!(
                "{}",
                int_to_unbal_ternary(date_tuple.2 as i128)
            );
        } else {
            return format!("{:#04x}", date_tuple.2);
        }
    }
}
