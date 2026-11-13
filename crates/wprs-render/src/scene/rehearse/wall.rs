const DAY: f64 = 86_400.0;

pub fn local_wall() -> f64 {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let seconds = libc::time_t::try_from(now.as_secs()).unwrap_or_default();
    let mut local = unsafe { std::mem::zeroed::<libc::tm>() };
    let found = !unsafe { libc::localtime_r(&seconds, &mut local) }.is_null();
    now.as_secs_f64() + if found { local.tm_gmtoff as f64 } else { 0.0 }
}

pub fn on_today(seconds: f64) -> f64 {
    (local_wall() / DAY).floor() * DAY + seconds.clamp(0.0, DAY - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_time_of_day_lands_on_today_and_never_leaves_it() {
        let day = (local_wall() / DAY).floor() * DAY;
        assert_eq!(on_today(15.0 * 3600.0 + 42.0 * 60.0), day + 56_520.0);
        assert_eq!(on_today(-5.0), day);
        assert_eq!(on_today(1e9), day + DAY - 1.0);
    }
}
