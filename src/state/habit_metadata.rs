use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use crate::Habit;

pub fn get_or_create() -> Result<HashMap<u32, Habit>, Box<dyn std::error::Error>> {
    match File::open("metadata.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let data: HashMap<u32, Habit> = serde_json::from_str(&contents)?;
            Ok(data)
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("No file found!");
                    let empty_map : HashMap<u32, Habit> = HashMap::new();
                    Ok(empty_map)
                }
                _ => {
                    Err(Box::new(error))
                }
            }
        }
    }
}

pub fn write_metadata(data: HashMap<u32, Habit>) -> Result<(), Box<dyn std::error::Error>> {
    let string_data = serde_json::to_string(&data)?;
    fs::write("metadata.txt", string_data.as_bytes())?;
 
    Ok(())
}

