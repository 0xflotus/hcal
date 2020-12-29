use bdays::easter;
use chrono::{Datelike, NaiveDate, Utc};
use clap::{App, Arg};
use regex::Regex;
use std::process;
use std::vec::Vec;

mod helpers;
use helpers::{cal, hex};

fn main() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let matches = App::new("hcal")
        .version(VERSION.unwrap_or("n/a"))
        .about("A hexadecimal calendar")
        .arg(
            Arg::new("easter")
                .short('e')
                .long("easter")
                .takes_value(true)
                .value_name("year")
                .about("Prints the Hex Date of easter."),
        )
        .arg(
            Arg::new("transform")
                .short('T')
                .long("transform")
                .takes_value(true)
                .value_name("date")
                .about("Prints the Hex Date of <date>. Needs format of dd-mm-yyyy."),
        )
        .arg(
            Arg::new("disable")
                .short('d')
                .long("disable")
                .about("Disable day marker"),
        )
        .arg(
            Arg::new("no-weekend")
                .short('W')
                .long("no-weekend")
                .about("Disable weekend marker"),
        )
        .arg(
            Arg::new("effect")
                .short('E')
                .long("effect")
                .about("Enable title font effects"),
        )
        .arg(
            Arg::new("disable-all")
                .short('A')
                .long("disbale-all")
                .about("Disable all font effects"),
        )
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

    if let Some(easter) = matches.value_of("easter") {
        let easter_date = easter::easter_naive_date(easter.parse::<i32>().unwrap_or_else(|_| {
            println!("Error while parsing year");
            process::exit(1_i32);
        }) as i32)
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(1_i32);
        });
        println!(
            "{}",
            format!(
                "{:#06x}-{:#04x}-{:#04x}",
                easter_date.year(),
                easter_date.month(),
                easter_date.day()
            )
        );
        process::exit(0_i32);
    }

    if let Some(date) = matches.value_of("transform") {
        let re = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap_or_else(|_| {
            println!("Error while parsing date");
            process::exit(1_i32);
        });
        if re.is_match(date) {
            let splitted_date: Vec<&str> = date.split('-').collect();
            println!(
                "{}",
                format!(
                    "{:#06x}-{:#04x}-{:#04x}",
                    splitted_date[2].parse::<u32>().unwrap_or_else(|_| {
                        println!("Error while parsing year");
                        process::exit(1_i32);
                    }),
                    splitted_date[1].parse::<u32>().unwrap_or_else(|_| {
                        println!("Error while parsing month");
                        process::exit(1_i32);
                    }),
                    splitted_date[0].parse::<i32>().unwrap_or_else(|_| {
                        println!("Error while parsing day");
                        process::exit(1_i32);
                    }),
                )
            );
            process::exit(0_i32);
        } else {
            println!("Wrong format. Please refer to the right format: dd-mm-yyyy");
            process::exit(1_i32);
        }
    }

    let show_day_marker = !(matches.is_present("disable-all") || matches.is_present("disable"));
    let show_weekend_marker =
        !(matches.is_present("disable-all") || matches.is_present("no-weekend"));
    let show_title_font_effect = !matches.is_present("disable-all") && matches.is_present("effect");

    if let Some(year) = matches.value_of("year") {
        if let Some(month) = matches.value_of("month") {
            if let Some(day) = matches.value_of("day") {
                if [day, month, year]
                    .iter()
                    .all(|&elem| elem.starts_with("0x"))
                {
                    hcal(
                        hex::trim_and_parse_hex(year).unwrap_or_else(|_| {
                            println!("Error while parsing year");
                            process::exit(1_i32);
                        }) as i32,
                        hex::trim_and_parse_hex(month).unwrap_or_else(|_| {
                            println!("Error while parsing month");
                            process::exit(1_i32);
                        }) as u32,
                        hex::trim_and_parse_hex(day).unwrap_or_else(|_| {
                            println!("Error while parsing day");
                            process::exit(1_i32);
                        }) as u32,
                        show_day_marker,
                        show_weekend_marker,
                        show_title_font_effect,
                    );
                } else {
                    let i32_year = year.parse::<i32>().unwrap_or_else(|_| {
                        println!("Error while parsing year");
                        process::exit(1_i32);
                    }) as i32;
                    let u32_month = month.parse::<u32>().unwrap_or_else(|_| {
                        println!("Error while parsing month");
                        process::exit(1_i32);
                    }) as u32;
                    let u32_day = day.parse::<u32>().unwrap_or_else(|_| {
                        println!("Error while parsing day");
                        process::exit(1_i32);
                    }) as u32;
                    hcal(
                        i32_year,
                        u32_month,
                        u32_day,
                        show_day_marker,
                        show_weekend_marker,
                        show_title_font_effect,
                    );
                }
            } else if [month, year].iter().all(|&elem| elem.starts_with("0x")) {
                hcal(
                    hex::trim_and_parse_hex(year).unwrap_or_else(|_| {
                        println!("Error while parsing year");
                        process::exit(1_i32);
                    }) as i32,
                    hex::trim_and_parse_hex(month).unwrap_or_else(|_| {
                        println!("Error while parsing month");
                        process::exit(1_i32);
                    }) as u32,
                    1_u32,
                    false,
                    show_weekend_marker,
                    show_title_font_effect,
                );
            } else {
                let i32_year = year.parse::<i32>().unwrap_or_else(|_| {
                    println!("Error while parsing year");
                    process::exit(1_i32);
                }) as i32;
                let u32_month = month.parse::<u32>().unwrap_or_else(|_| {
                    println!("Error while parsing month");
                    process::exit(1_i32);
                }) as u32;
                hcal(
                    i32_year,
                    u32_month,
                    1_u32,
                    false,
                    show_weekend_marker,
                    show_title_font_effect,
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
        );
    }
}

fn hcal(year: i32, month: u32, day: u32, show_day: bool, show_weekend: bool, effect: bool) {
    let now = NaiveDate::from_ymd(year, month, day);
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let days_from_monday = NaiveDate::from_ymd(year as i32, month, 1_u32)
        .weekday()
        .num_days_from_monday();
    println!(
        "\t\t\t{}\n\t\t\t  {}\n",
        format!("{:#06x}", year),
        format!("{:02x}", month)
    );

    println!("{}", headline(effect));
    let mut end = 7_u32 - days_from_monday;
    let mut vec = Vec::new();
    for x in 1_u32..=end {
        vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
    }
    println!(
        "{}{}",
        "\t".repeat(days_from_monday as usize),
        vec.join("\t")
    );
    end += 1_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
    }
    println!("{}", vec.join("\t"));
    end += 7_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
    }
    println!("{}", vec.join("\t"));
    end += 7_u32;
    vec = Vec::new();
    for x in end..end + 7_u32 {
        vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
    }
    println!("{}", vec.join("\t"));
    end += 7_u32;
    vec = Vec::new();
    if cal::get_days_from_month(year as i32, month as u32) as u32 - end < 7_u32 {
        for x in end..=cal::get_days_from_month(year as i32, month as u32) as u32 {
            vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
        }
        println!("{}", vec.join("\t"));
    } else {
        for x in end..=end + 6_u32 {
            vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
        }
        println!("{}", vec.join("\t"));
        vec = Vec::new();
        for x in end + 7_u32..=cal::get_days_from_month(year as i32, month as u32) as u32 {
            vec.push(mark((year as i32, month, x), day, show_day, show_weekend));
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
) -> std::string::String {
    use chrono::Weekday;
    let the_day = NaiveDate::from_ymd(date_tuple.0, date_tuple.1, date_tuple.2);
    let weekday = the_day.weekday();
    if date_tuple.2 == day && show_day {
        return format!("\u{001b}[41m{:#04x}\u{001b}[0m", date_tuple.2);
    } else if [Weekday::Sat, Weekday::Sun].contains(&weekday) && show_weekend {
        return format!("\u{001b}[7m{:#04x}\u{001b}[0m", date_tuple.2);
    } else {
        return format!("{:#04x}", date_tuple.2);
    }
}
