use crate::Habit;
use crate::state::habit_metadata;

pub fn run() -> Result<Vec<Habit>, Box<dyn std::error::Error>> {
    let data = habit_metadata::get_or_create()?;

    Ok(data.values().cloned().collect())
}
