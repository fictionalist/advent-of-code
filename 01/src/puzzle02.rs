use std::{fs::File, io::Read};
use std::collections::HashMap;

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

fn get_callibration_values(data: Vec<String>) -> u64 {
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

fn separate_by_line(mut data: String) -> Vec<String> {
    let mut out : Vec<String> = Default::default();

    while let Some(idx) = data.find('\n') {
        let line = data[0..idx].to_string();
        out.push(line);
        data = data[(idx + 1)..].to_string();
    }

    out
}

pub fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input01.txt")?;
    let mut buffer : String = Default::default();
    
    if file.read_to_string(&mut buffer)? == 0 {
        println!("Empty file.");
        ()
    }
    

    let data = separate_by_line(buffer);

    /*let data : Vec<String> = vec!(
        "two1nine".to_string(),
        "eighttwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string()
    );*/

    let sum = get_callibration_values(data);

    println!("Puzzle #02 total: {}", sum);

    Ok(())
}