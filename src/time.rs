use chrono::{DateTime, FixedOffset, Local};

pub struct Time {}

impl Time {
    // Returns the local time in RFC3339 format: e.g. 2022-10-26 07:06:30.787611 -03:00
    pub fn now() -> String {
        let now = Local::now();
        now.to_rfc3339()
    }

    // Receives a time in RFC3339 format, and returns the elapsed time since that timestamp
    pub fn elapsed_since(date_str: &String) -> String {
        let back_then: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&date_str).unwrap();
        let now = Local::now();
        let duration = now.signed_duration_since(back_then);
        let total_secs = duration.num_seconds();
        let ss = total_secs % 60;
        let mm = (total_secs / 60) % 60;
        let hh = (total_secs / 60) / 60;
        format!("{:02}:{:02}:{:02}", hh, mm, ss)
    }

    // Reads a time in RFC3339 format into a DateTime: e.g. 2022-10-26 07:06:30.787611 -03:00
    // pub fn from_str(date_str: String) -> DateTime<FixedOffset> {
    // let date_time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&date_str).unwrap();
    //     date_time
    // }
}
