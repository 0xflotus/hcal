pub mod cal {
    use chrono::NaiveDate;

    // from https://stackoverflow.com/a/58188385
    pub fn get_days_from_month(year: i32, month: u32) -> i64 {
        NaiveDate::from_ymd(
            match month {
                0xc_u32 => year + 1_i32,
                _ => year,
            },
            match month {
                0xc_u32 => 1_u32,
                _ => month + 1_u32,
            },
            1_u32,
        )
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1_u32))
        .num_days()
    }
}

pub mod hex {
    use std::{i64, num};

    const PREFIX: &str = "0x";

    pub fn trim_and_parse_hex(hex: &str) -> Result<i64, num::ParseIntError> {
        return i64::from_str_radix(hex.trim_start_matches(PREFIX), 0x10_u32);
    }
}

pub mod fmt {
    use cbb::util::cbb::int_to_bal_ternary;
    pub fn format_date(date: (i32, u32, u32), balanced_ternary: bool) -> std::string::String {
        if balanced_ternary {
            return format!(
                "{}:{}:{}",
                int_to_bal_ternary(date.0 as i128),
                int_to_bal_ternary(date.1 as i128),
                int_to_bal_ternary(date.2 as i128)
            );
        } else {
            return format!("{:#06x}-{:#04x}-{:#04x}", date.0, date.1, date.2);
        }
    }
}
