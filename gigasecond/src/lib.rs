extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let billion_seconds = 1_000_000_000;
    start + Duration::seconds(billion_seconds)
}
