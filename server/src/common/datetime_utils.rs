use chrono::{Duration, NaiveDateTime, DateTime, Utc, Local};

pub fn convert_timestamp_to_local_datetime(duration_secs: i64) -> DateTime<Local> {
    let duration = Duration::seconds(duration_secs).num_microseconds().unwrap();
    let naive_date = NaiveDateTime::from_timestamp_micros(duration).unwrap();
    let date: DateTime<Utc> = DateTime::from_utc(naive_date, Utc);
    date.with_timezone(&Local)
}