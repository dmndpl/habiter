use crate::{Mode, Frequency, Habit};
use crate::state::habit_metadata;

pub fn run(name: String, learn: Mode, threshold: u32, frequency: Frequency) -> Result<(), Box<dyn std::error::Error>> {
    let mut habits = habit_metadata::get_or_create()?;

    habits.insert(1, Habit {
        id: 1,
        name: name,
        learn: learn,
        threshold: threshold,
        frequency: frequency
    });

    let _ = habit_metadata::write_metadata(habits);

    Ok(())
}