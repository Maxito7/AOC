use core::panic;
use std::fs;

pub fn load_file(day: u8, suffix: Option<&str>) -> String {
    load_raw_file(day, suffix).trim().replace('\r', "")
}

pub fn load_raw_file(day: u8, suffix: Option<&str>) -> String {
    let file = format!("./inputs/day{:02}{}.in", day, suffix.unwrap_or(""));
    println!("FILE NAME: {}", file);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}
