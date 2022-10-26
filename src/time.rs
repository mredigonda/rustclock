use chrono::Local;

pub struct Time {}

impl Time {
    // Returns the local time in RFC3339 format: e.g. 2022-10-26 07:06:30.787611 -03:00
    pub fn now() -> String {
        let now = Local::now();
        now.to_rfc3339()
    }

    // Reads a time in RFC3339 format into a DateTime: e.g. 2022-10-26 07:06:30.787611 -03:00
    // pub fn from_str(date_str: String) -> DateTime<FixedOffset> {
    //     let date_time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&date_str).unwrap();
    //     date_time
    // }
}
