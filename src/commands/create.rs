use crate::{Mode, Frequency, Habit};
use crate::state::habit_metadata;

pub fn run(name: String, learn: Mode, threshold: u32, frequency: Frequency) -> Result<(), Box<dyn std::error::Error>> {
    let mut habits = habit_metadata::get_or_create()?;
    
    let prev_key = habits.keys().max().unwrap_or_else(|| {&0});
    let new_key = prev_key + 1;

    habits.insert(new_key, Habit {
        id: new_key,
        name: name,
        learn: learn,
        threshold: threshold,
        frequency: frequency
    });

    let _ = habit_metadata::write_metadata(habits);

    Ok(())
}
