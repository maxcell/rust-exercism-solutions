use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // Love learning about the add trait!
    start + Duration::seconds(1_000_000_000)
}
