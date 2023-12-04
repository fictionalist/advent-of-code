use crate::parse_file::read_file;

fn get_scratch_values(data: &Vec<String>) -> i32 {
    let mut scratch = 0;

    for (_, line) in data.iter().enumerate() {
        let mut winning_values: Vec<i32> = vec!();

        let winning_values_str = line[(line.find(":").unwrap()) .. line.find("|").unwrap()].to_string();

        let mut split = winning_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                winning_values.push(i);
            }
        }

        let mut card_values: Vec<i32> = vec!();

        let card_values_str = line[line.find("|").unwrap() ..].to_string();
        
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

    let mut scratchcards: Vec<i32> = vec!();

    for (_, _) in data.iter().enumerate() {
        scratchcards.push(1);
    }

    for (id, line) in data.iter().enumerate() {
        let mut winning_values: Vec<i32> = vec!();

        let winning_values_str = line[line.find(":").unwrap() .. line.find("|").unwrap()].to_string();

        let mut split = winning_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                winning_values.push(i);
            }
        }

        let mut card_values: Vec<i32> = vec!();

        let card_values_str = line[line.find("| ").unwrap() ..].to_string();
        
        split = card_values_str.split_whitespace();

        while let Some(idx) = split.next() {
            if let Ok(i) = idx.to_string().parse::<i32>() {
                card_values.push(i);
            }
        }

        let mut i = id + 1;
        for winning_value in &winning_values {
            for card_value in &card_values {
                if *card_value == *winning_value {
                    scratchcards[i as usize] += scratchcards[id];
                    i += 1;
                    break;
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

    let start = std::time::Instant::now();
    let sum = get_scratch_values(&data);

    println!("\tPart 1 - Scratchcards values: {} ({} ms)", sum, start.elapsed().as_millis());

    let start = std::time::Instant::now();
    let scratch_cards = get_scratch_cards(&data);

    println!("\tPart 2 - Sum of scratchcards: {} ({} ms)", scratch_cards, start.elapsed().as_millis());

    drop(data);

    Ok(())
}