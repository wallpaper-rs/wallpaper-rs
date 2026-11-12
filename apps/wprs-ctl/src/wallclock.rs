use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

const DAY: f64 = 86_400.0;

pub fn parse(text: &str) -> Result<f64, String> {
    parse_on(text, wprs_render::local_wall())
}

fn parse_on(text: &str, now: f64) -> Result<f64, String> {
    let text = text.trim();
    let bad = || format!("{text} is not a time: use HH:MM, HH:MM:SS or YYYY-MM-DD HH:MM");
    for format in ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M:%S", "%Y-%m-%dT%H:%M"] {
        if let Ok(at) = NaiveDateTime::parse_from_str(text, format) {
            return Ok(at.and_utc().timestamp() as f64);
        }
    }
    if let Ok(day) = NaiveDate::parse_from_str(text, "%Y-%m-%d") {
        return Ok(day.and_hms_opt(12, 0, 0).ok_or_else(bad)?.and_utc().timestamp() as f64);
    }
    for format in ["%H:%M:%S", "%H:%M"] {
        if let Ok(at) = NaiveTime::parse_from_str(text, format) {
            let seconds = f64::from(chrono::Timelike::num_seconds_from_midnight(&at));
            return Ok((now / DAY).floor() * DAY + seconds);
        }
    }
    Err(bad())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_time_of_day_is_placed_on_today_and_a_full_date_stands_alone() {
        let today = 19_000.0 * DAY + 5_000.0;
        assert_eq!(parse_on("15:42", today), Ok(19_000.0 * DAY + 15.0 * 3600.0 + 42.0 * 60.0));
        assert_eq!(parse_on(" 03:07:09 ", today), Ok(19_000.0 * DAY + 3.0 * 3600.0 + 7.0 * 60.0 + 9.0));
        assert_eq!(parse_on("2024-02-29 08:30", today), Ok(1_709_195_400.0));
        assert_eq!(parse_on("2024-02-29", today), Ok(1_709_208_000.0));
        assert!(parse_on("25:00", today).is_err());
        assert!(parse_on("noon", today).unwrap_err().contains("HH:MM"));
    }
}
