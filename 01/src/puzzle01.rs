use std::{fs::File, io::Read};

fn is_digit(input: u8) -> bool {
    if input <= 0x29 || input >= 0x3A {
        return false;
    }

    true
}

fn get_callibration_values(data: Vec<String>) -> u64 {
    let mut sum : u64 = 0;

    for s in data {
        print!("{}: ", s);
        let mut dec: u8;
        for c in s.bytes() {
            if is_digit(c) {
                dec = c - 0x30;
                sum = sum + (dec * 10) as u64;
                print!("{}", dec);
                break;
            }
        }
        for c in s.bytes().rev() {
            if is_digit(c) {
                dec = c - 0x30;
                sum = sum + dec as u64;
                print!("{}", dec);
                break;
            }
        }
        println!("");
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

    /* let data = vec!{
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string()
    }; */

    let sum = get_callibration_values(data);

    println!("Puzzle #01 Total: {}", sum);

    Ok(())
}