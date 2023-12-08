struct Race {
    time: i64,
    distance: i64
}

/*fn get_test_data() -> String {
    "Time:      7  15   30
    Distance:  9  40  200".to_string()
}*/

fn calculate_distance(total_time: i64, time_held: i64) -> i64 {
    // speed = distance / time
    // acceleration = speed / time
    //
    // acceleration = 1mm/ms^2
    // 1mm/ms^2 = speed / time
    // 1mm/ms^2 = distance / (time)^2
    // distance = (time ms)^2 * 1mm/ms^2 
    (total_time - time_held) * time_held
}

fn get_win_count_product(time_limit_str: &str, distances_str: &str) -> i32 {
    let time_limit_split: Vec<&str> = time_limit_str.trim().strip_prefix("Time:").unwrap().split_whitespace().collect();
    let distance_split: Vec<&str> = distances_str.trim().strip_prefix("Distance:").unwrap().split_whitespace().collect();

    let mut races: Vec<Race> = vec!();

    for (i, _) in time_limit_split.iter().enumerate() {
        let time = time_limit_split[i].parse::<i64>().unwrap();
        let distance = distance_split[i].parse::<i64>().unwrap();
        races.push(Race { time, distance });
    }

    let mut result = 0;

    for race in races {
        let mut race_wins = 0;
        for time in 0 .. race.time {
            if calculate_distance(race.time, time) > race.distance {
                race_wins += 1;
            }
        }
        if result == 0 {
            result = race_wins;
        } else {
            result *= race_wins;
        }
    }

    result
}

fn get_win_count_product_part_2(time_limit_str: &str, distances_str: &str) -> i32 {
    let time_limit_split: Vec<&str> = time_limit_str.trim().strip_prefix("Time:").unwrap().split_whitespace().collect();
    let distance_split: Vec<&str> = distances_str.trim().strip_prefix("Distance:").unwrap().split_whitespace().collect();

    let time = time_limit_split.concat().parse::<i64>().unwrap();
    let distance = distance_split.concat().parse::<i64>().unwrap();

    let race = Race { time, distance };
    
    let mut race_wins = 0;
    for time in 0 .. race.time {
        if calculate_distance(race.time, time) > race.distance {
            race_wins += 1;
        }
    }

    race_wins
}

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input06.txt")?;

    //let data = get_test_data();

    let lines : Vec<&str> = data.lines().collect();

    println!("Day 06:");

    let start = std::time::Instant::now();
    let win_count_product = get_win_count_product(lines[0], lines[1]);

    println!("\tPart 1 - Product of number of ways to beat each record: {} ({} us)", win_count_product, start.elapsed().as_micros());

    let start = std::time::Instant::now();
    let win_count = get_win_count_product_part_2(lines[0], lines[1]);
    println!("\tPart 2 - Product of number of ways to beat record: {} ({} ms)", win_count, start.elapsed().as_millis());

    Ok(())
}