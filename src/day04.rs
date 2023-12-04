use crate::parse_file::read_file;

fn get_card_id(line: &str) -> i32 {
    let colon_position = line.find(':').unwrap();
    let card_id_str = line[0 .. colon_position].to_string();

    let mut splits = card_id_str.split_whitespace();
    splits.next();

    let i_str = splits.next().unwrap();
    
    let i = i_str.parse::<i32>();

    match i {
        Ok(i) => {
            return i;
        },
        Err(e) => {
            println!("Failed to parse card ID: {}", e);
            return -1;
        }
    }
}

fn get_scratch_values(data: &Vec<String>) -> i32 {
    let mut scratch = 0;

    for line in data {
        let mut winning_values: Vec<i32> = vec!();

        let scratch_values_start = line.find(": ").unwrap() + 2;
        let scratch_values_end = line.find(" |").unwrap();

        let winning_values_str = line[scratch_values_start .. scratch_values_end].to_string();

        let mut split = winning_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                winning_values.push(i);
            }
        }

        let mut card_values: Vec<i32> = vec!();

        let card_values_start = line.find("| ").unwrap() + 1;

        let card_values_str = line[card_values_start ..].to_string();
        
        split = card_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                card_values.push(i);
            }
        }

        let mut card_sum = 0;
        for winning_value in winning_values {
            for card_value in &card_values {
                if *card_value == winning_value {
                    if card_sum == 0 {
                        card_sum += 1;
                        break;
                    }
                    card_sum *= 2;
                }
            }
        }
        scratch += card_sum;
    }

    scratch
}

fn get_scratch_cards(data: &Vec<String>) -> i32 {
    let mut sum_scratchcards = 0;

    let mut num_scratchcards = 0;

    for (i, _) in data.iter().enumerate() {
        num_scratchcards = i;
    }

    let mut scratchcards : Vec<i32> = vec!();
    for _ in 0 .. (num_scratchcards + 1) {
        scratchcards.push(1);
    }

    for line in data {
        let id = get_card_id(&(line.as_str()));

        let mut winning_values: Vec<i32> = vec!();

        let scratch_values_start = line.find(": ").unwrap() + 2;
        let scratch_values_end = line.find(" |").unwrap();

        let winning_values_str = line[scratch_values_start .. scratch_values_end].to_string();

        let mut split = winning_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                winning_values.push(i);
            }
        }

        let mut card_values: Vec<i32> = vec!();

        let card_values_start = line.find("| ").unwrap() + 1;

        let card_values_str = line[card_values_start ..].to_string();
        
        split = card_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                card_values.push(i);
            }
        }
        for _ in 0 .. scratchcards[id as usize - 1] {
            let mut i = id;
            for winning_value in &winning_values {
                for card_value in &card_values {
                    if *card_value == *winning_value {
                        scratchcards[i as usize] += 1;
                        i += 1;
                        break;
                    }
                }
            }
        }
    }

    for (_, scratchcard) in scratchcards.iter().enumerate() {
        sum_scratchcards += scratchcard;
    }

    sum_scratchcards
}

pub fn main() -> Result<(), std::io::Error> {
    let data = read_file("input04.txt")?;
    /*let data = vec!(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    );*/

    println!("Day 04:");

    let sum = get_scratch_values(&data);

    println!("\tPart 1 - Scratchcards values: {}", sum);

    let scratch_cards = get_scratch_cards(&data);

    println!("\tPart 2 - Sum of scratchcards: {}", scratch_cards);

    drop(data);

    Ok(())
}