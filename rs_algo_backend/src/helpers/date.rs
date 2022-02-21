pub use chrono::offset::{Local, TimeZone};
pub use chrono::{Date, DateTime, Duration, NaiveDateTime, NaiveTime, Utc};

pub fn parse_time(date: i64) -> DateTime<Local> {
    let ts = chrono::NaiveDateTime::from_timestamp(date, 0);
    Local.from_utc_datetime(&ts)
}
