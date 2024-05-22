use chrono::{DateTime, Utc};
use serde_json;
use std::fs::{self, File};
use std::io::{self, Read};

pub fn get_or_create(id: u32) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let filename = format!("{}-history.txt", id);
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let data: Vec<i64> = serde_json::from_str(&contents)?;
            Ok(data)
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                let empty_list: Vec<i64> = Vec::new();
                Ok(empty_list)
            }
            _ => Err(Box::new(error)),
        },
    }
}

pub fn update(id: u32, data: Vec<DateTime<Utc>>) -> Result<(), Box<dyn std::error::Error>> {
    let now: i64 = Utc::now().timestamp();
    let filename = format!("{}-history.txt", id);
    let mut dates_as_timestamp: Vec<i64> = data.iter().map(|dt| dt.timestamp()).collect();
    dates_as_timestamp.push(now);
    let data_as_string = serde_json::to_string(&dates_as_timestamp)?;
    fs::write(filename, data_as_string.as_bytes())?;

    Ok(())
}
