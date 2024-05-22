use crate::state::habit_occurrence;
use chrono::{DateTime, TimeZone, Utc};

pub fn run(id: u32) -> Result<Vec<DateTime<Utc>>, Box<dyn std::error::Error>> {
    let occurrences_timestamp: Vec<i64> = habit_occurrence::get_or_create(id)?;
    let occurrences = occurrences_timestamp
        .iter()
        .map(|timestamp| Utc.timestamp(*timestamp, 0))
        .collect();
    Ok(occurrences)
}
