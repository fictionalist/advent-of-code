/*
fn get_test_data_part_one() -> String {
    "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)".to_string()
}

fn get_test_data_part_two() -> String {
    "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)".to_string()
}
*/

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn get_node(line: &str) -> (String, (String, String)) {
    let split: Vec<&str> = line.trim().split_whitespace().collect();

    let value = split[0].to_string();
    let left = split[2].strip_prefix("(").unwrap().strip_suffix(",").unwrap().to_string();
    let right = split[3].strip_suffix(")").unwrap().to_string();

    (value, (left, right))    
}

fn loop_through(pattern: &Vec<bool>, map: &std::collections::HashMap<String, (String, String)>, targets: &Vec<String>, start: &str) -> i64 {
    let mut pattern_position = 0;
    let mut cursor = start;
    let mut step_count = 0;
    let mut exit_loop = false;

    loop {
        match map.get(cursor) {
            Some(item) => {
                let direction_right = pattern[pattern_position];
                match direction_right {
                    true => {
                        cursor = &item.1;
                    },
                    false => {
                        cursor = &item.0;
                    }
                }
                pattern_position += 1;
                if pattern_position >= pattern.len() {
                    pattern_position = 0;
                }
                step_count += 1;
                //println!("Step {}: {}", step_count, cursor);
                for target in targets {
                    if cursor == *target {
                        exit_loop = true;
                        break;
                    }
                }
            },
            None => break
        }
        if exit_loop {
            break;
        }
    }

    step_count
}

fn part_one(lines: Vec<&str>) -> i64 {
    let mut iterator = lines.iter();

    let pattern_str = iterator.next().unwrap().to_string();
    let mut pattern: Vec<bool> = vec!();

    for c in pattern_str.chars().into_iter() {
        match c {
            'R' => pattern.push(true),
            'L' => pattern.push(false),
            _ => continue
        }
    }

    let mut map = std::collections::HashMap::<String, (String, String)>::new();

    for line in lines.into_iter().skip(2) {
        let node = get_node(line);
        map.insert(node.0, (node.1.0, node.1.1));
    }

    loop_through(&pattern, &map, &vec!("ZZZ".to_string()), "AAA")
}

fn part_two(lines: Vec<&str>) -> i64 {
    let mut iterator = lines.iter();

    let pattern_str = iterator.next().unwrap().to_string();
    let mut pattern: Vec<bool> = vec!();

    for c in pattern_str.chars().into_iter() {
        match c {
            'R' => pattern.push(true),
            'L' => pattern.push(false),
            _ => continue
        }
    }

    let mut map = std::collections::HashMap::<String, (String, String)>::new();

    let mut starting_points: Vec<String> = vec!();
    let mut ending_points: Vec<String> = vec!();

    for line in lines.into_iter().skip(2) {
        let node = get_node(line);
        //println!("({}, l: {}, r: {})", node.0, node.1.0, node.1.1);
        if node.0.ends_with("A") {
            starting_points.push(node.0.clone());
        }
        if node.0.ends_with("Z") {
            ending_points.push(node.0.clone());
        }
        map.insert(node.0, (node.1.0, node.1.1));
    }

    let mut step_cycles: Vec<i64> = Vec::new();

    for point in starting_points {
        step_cycles.push(loop_through(&pattern, &map, &ending_points, point.as_str()));
    }

    let mut min_multiple = 1;

    for cycle in step_cycles {
        min_multiple = lcm(min_multiple, cycle as usize);
    }

    min_multiple as i64
}

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input08.txt")?;
    //let data = get_test_data_part_one();

    println!("Day 08:");

    let lines: Vec<&str> = data.trim().lines().collect();

    let start = std::time::Instant::now();
    let steps = part_one(lines);

    println!("\tPart 1 - Number of steps to reach ZZZ: {} ({} ms)", steps, start.elapsed().as_millis());

    //let data = get_test_data_part_two();
    let lines: Vec<&str> = data.trim().lines().collect();
    let start = std::time::Instant::now();
    let steps = part_two(lines);

    println!("\tPart 2 - Number of steps to reach all XXZ: {} ({} ms)", steps, start.elapsed().as_millis());

    Ok(())
}