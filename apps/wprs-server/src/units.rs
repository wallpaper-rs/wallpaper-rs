use crate::certs::MONTHS;
use crate::lang::{Lang, current, tf, word};

const MB: u64 = 1 << 20;
const GB: u64 = 1 << 30;

pub fn size(bytes: u64) -> String {
    let (number, unit) = if bytes >= GB {
        let tenths = (bytes * 10 + GB / 2) / GB;
        let whole = tenths / 10;
        (if tenths.is_multiple_of(10) { whole.to_string() } else { format!("{whole}.{}", tenths % 10) }, "GB")
    } else {
        (((bytes + MB / 2) / MB).to_string(), "MB")
    };
    let number = if current() == Lang::Es { number.replace('.', ",") } else { number };
    format!("{number}\u{a0}{unit}")
}

pub fn date(ms: i64) -> String {
    let when = chrono::DateTime::from_timestamp_millis(ms).unwrap_or_default();
    let month = MONTHS[chrono::Datelike::month0(&when) as usize];
    tf("{2} {1}", &[&chrono::Datelike::day(&when), &word(month)])
}

pub fn gigabytes(bytes: u64) -> String {
    let tenths = (bytes * 10 + GB / 2) / GB;
    if tenths.is_multiple_of(10) {
        (tenths / 10).to_string()
    } else {
        format!("{}.{}", tenths / 10, tenths % 10)
    }
}

pub fn parse_gigabytes(text: &str) -> Option<i64> {
    let value: f64 = text.trim().replace(',', ".").parse().ok()?;
    (value.is_finite() && (0.0..=1_048_576.0).contains(&value)).then(|| (value * 1024.0).round() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_shown_in_megabytes_and_gigabytes() {
        assert_eq!(size(2 * GB), "2\u{a0}GB");
        assert_eq!(size(640 * MB), "640\u{a0}MB");
        assert_eq!(size(GB + GB / 2), "1.5\u{a0}GB");
        assert_eq!(gigabytes(2 * GB), "2");
        assert_eq!(parse_gigabytes("1,5"), Some(1536));
        assert_eq!(parse_gigabytes("x"), None);
        assert_eq!(parse_gigabytes("-1"), None);
    }
}
