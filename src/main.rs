use chrono::{Datelike, Utc};
use clap::{App, Arg};

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
        println!(
            "{}-{}-{}",
            format!("{:#06x}", year),
            format!("{:02x}", month),
            format!("{:02x}", day)
        );
    }
}
