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
        .get_matches();

    if let Some(year) = matches.value_of("year") {
        println!("Provided year: {}", year);
    } else {
        let now = Utc::now();
        let (_is_common_era, year) = now.year_ce();
        println!("Provided year: {}", year)
    }
}
