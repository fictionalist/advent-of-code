use crate::parse_file::read_file;

fn is_digit(input: u8) -> bool {
    if input <= 0x29 || input >= 0x3A {
        return false;
    }

    true
}

fn get_calibration_values(data: Vec<String>) -> u64 {
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

pub fn main() -> Result<(), std::io::Error> {
    let data = read_file("input01.txt")?;

    /* let data = vec!{
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string()
    }; */

    let sum = get_calibration_values(data);

    println!("Puzzle #01 total: {}", sum);

    Ok(())
}