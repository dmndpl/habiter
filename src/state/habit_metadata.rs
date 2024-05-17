use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, Read};
use crate::Habit;

pub fn get_or_create() -> Result<HashMap<u32, Habit>, Box<dyn std::error::Error>> {
    match File::open("metadata.txt") {
        Ok(mut file) => {
            println!("File opened successfully!");
                // Read the contents of the file into a string
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Parse the contents of the file into a MyData struct
            let data: HashMap<u32, Habit> = serde_json::from_str(&contents)?;
            Ok(data)
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
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
    println!("{}", string_data);

    let mut file = File::open("metadata.txt")?;

    // Write some content to the file
    file.write_all(string_data.as_bytes())?;
    Ok(())
}

