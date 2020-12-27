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
    use std::i64;

    pub fn trim_and_parse_hex(hex: &str) -> i64 {
        let without_prefix = hex.trim_start_matches("0x");
        match i64::from_str_radix(without_prefix, 0x10_u32) {
            Ok(val) => val,
            Err(_) => 0,
        }
    }
}
