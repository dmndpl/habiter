use crate::state::habit_metadata;
use crate::Habit;

pub fn run() -> Result<Vec<Habit>, Box<dyn std::error::Error>> {
    let data = habit_metadata::get_or_create()?;

    Ok(data.values().cloned().collect())
}
