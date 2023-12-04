use std::collections::HashMap;

use crate::parse_file::read_file;

fn is_digit(input: u8) -> bool {
    if input <= 0x29 || input >= 0x3A {
        return false;
    }

    true
}

fn is_spelled_digit(input: &str) -> Option<u8> {
    let num_strings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    for item in num_strings {
        if input.find(item.0).is_some() {
            return Some(item.1);
        }
    }

    None
}

fn get_calibration_values(data: &Vec<String>) -> u64 {
    let mut sum : u64 = 0;

    for s in data {
        let mut dec: u8;
        for c in s.bytes() {
            if is_digit(c) {
                dec = c - 0x30;
                sum = sum + (dec * 10) as u64;
                break;
            }
        }
        for c in s.bytes().rev() {
            if is_digit(c) {
                dec = c - 0x30;
                sum = sum + dec as u64;
                break;
            }
        }
    }

    sum
}

fn get_calibration_values_expressed(data: &Vec<String>) -> u64 {
    let mut sum : u64 = 0;

    for s in data {
        let mut dec: u8;
        let mut base_pos: usize = 0;
        let mut curr_pos: usize = 0;
        for c in s.bytes() {
            if is_digit(c) {
                dec = c - 0x30;
                sum = sum + (dec * 10) as u64;
                break;
            }
            curr_pos = curr_pos + 1;
            if curr_pos > 5 {
                base_pos = base_pos + 1;
            }
            if let Some(t) = is_spelled_digit(&s[base_pos .. curr_pos]) {
                sum = sum + (t * 10) as u64;
                break;
            }
        }

        base_pos = s.len();
        curr_pos = s.len();
        for c in s.bytes().rev() {
            if is_digit(c) {
                dec = c - 0x30;
                sum = sum + dec as u64;
                break;
            }

            curr_pos = curr_pos - 1;
            if (s.len() - curr_pos) > 5 {
                base_pos = base_pos - 1;
            }
            if let Some(t) = is_spelled_digit(&s[curr_pos .. base_pos]) {
                sum = sum + t as u64;
                break;
            }
            if curr_pos == 0 {
                break;
            }
        }
    }

    sum
}

pub fn main() -> Result<(), std::io::Error> {
    let data = read_file("input01.txt")?;

    /* let data = vec!{
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string()
    }; */

    println!("Day 01:");
    let sum = get_calibration_values(&data);
    println!("\tPart 1 - total: {}", sum);
    let sum = get_calibration_values_expressed(&data);
    println!("\tPart 2 - total: {}", sum);

    drop(data);

    Ok(())
}