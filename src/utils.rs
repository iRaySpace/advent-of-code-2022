use std::fs;

pub fn get_data(date: u8) -> String {
    fs::read_to_string(format!("data/{date}.txt")).expect("Error reading data")
}
