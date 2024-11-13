pub use chrono::{Weekday as wd, NaiveDate, Datelike};

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    if is_leap_year {
        None
    } else {
        let middle_day = 183;
        let middle_date = NaiveDate::from_yo(year, middle_day);
        Some(middle_date.weekday())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        println!(
            "{:?}",
            (middle_day(1892), middle_day(1904), middle_day(2012))
        );
        assert!(middle_day(1892).is_none(), "1892 was a leap year!");
        assert!(middle_day(1904).is_none(), "1904 was a leap year!");
        assert!(middle_day(2012).is_none(), "2012 was a leap year!");
    }

    #[test]
    fn weekdays() {
        assert_eq!(wd::Tue, middle_day(2019).unwrap());
        assert_eq!(wd::Wed, middle_day(1997).unwrap());
        assert_eq!(wd::Mon, middle_day(1663).unwrap());
        assert_eq!(wd::Wed, middle_day(1873).unwrap());
        assert_eq!(wd::Thu, middle_day(1953).unwrap());
        assert_eq!(wd::Wed, middle_day(1879).unwrap());
    }
}
