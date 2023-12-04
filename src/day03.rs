use crate::parse_file::read_file;

struct Vec2 {
    x: i32,
    y: i32,
}

struct Node {
    position: Vec2,
    part: char
}

struct PartNumber {
    position: Vec2,
    number: i32,
    size: i8,
}

fn get_parts(data: &Vec<String>) -> Vec<Node> {
    let mut part_list : Vec<Node> = vec!();

    let mut x: i32;
    let mut y: i32 = 0;

    for line in data {
        x = 0;
        let undotted_line = line.replace(".", " ").replace(char::is_numeric, " ");
        for split in undotted_line.split(' ') {
            if split != "" {
                for c in split.chars() {
                    part_list.push(Node {part: c, position: Vec2 {x, y}});
                    x += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    /*for part in &part_list {
        println!("Part {} ({}, {})", part.part, part.position.0, part.position.1);
    }*/

    part_list
}

fn get_numbers(data: &Vec<String>) -> Vec<PartNumber> {
    let parts: Vec<char> = vec!('!', '@', '#', '$', '%', '&', '*', '-', '+', '=', '/');
    let mut number_list: Vec<PartNumber> = vec!();

    let mut x: i32;
    let mut y: i32 = 0;

    for line in data {
        x = 0;
        let mut undotted_line = line.replace(".", " ");
        for part in &parts {
            undotted_line = undotted_line.replace(*part, " ");
        }
        for split in undotted_line.split(' ') {
            let num = split.parse::<i32>();
            if let Ok(i) = num {
                number_list.push(PartNumber { position: Vec2{x, y}, number: i, size: split.len() as i8});
                x += split.len() as i32;
            }
            x += 1;
        }
        y += 1;
    }

    /*for num in &number_list {
        println!("Number `{}` ({}, {})", num.number, num.position.0, num.position.1);
    }*/

    number_list
}

fn get_part_num_sum(numbers: &Vec<PartNumber>, parts: &Vec<Node>) -> i32 {
    let mut sum = 0;

    let positions: Vec<Vec2> = vec!(
        Vec2 {x: -1, y: -1}, Vec2 {x:  0, y: -1}, Vec2 {x:  1, y: -1},
        Vec2 {x: -1, y:  0},                      Vec2 {x:  1, y:  0},
        Vec2 {x: -1, y:  1}, Vec2 {x:  0, y:  1}, Vec2 {x:  1, y:  1},
    );
    
    for part in parts {
        //println!("Part `{}` ({}, {})", part.part, part.position.x, part.position.y);
        for number in numbers {
            //println!("- Number `{}` ({}, {})", number.number, number.position.x, number.position.y);
            let mut found = false;
            for kernel_pos in &positions {
                let try_pos = Vec2 {x: part.position.x + kernel_pos.x, y: part.position.y + kernel_pos.y};
                //print!(" > ({}, {}) ", try_pos.x, try_pos.y);
                for num_pos_x in 0 .. number.size {
                    if (try_pos.x == number.position.x + num_pos_x as i32) &&
                        (try_pos.y == number.position.y) {
                        found = true;
                        //print!("#");
                        break;
                    }
                }
                //println!("");
                if found {
                    sum += number.number;
                    break;
                }
            }
        }
    }

    sum
}

fn get_gear_ratios(numbers: &Vec<PartNumber>, parts: &Vec<Node>) -> i32 {
    let mut sum = 0;

    let positions: Vec<Vec2> = vec!(
        Vec2 {x: -1, y: -1}, Vec2 {x:  0, y: -1}, Vec2 {x:  1, y: -1},
        Vec2 {x: -1, y:  0},                      Vec2 {x:  1, y:  0},
        Vec2 {x: -1, y:  1}, Vec2 {x:  0, y:  1}, Vec2 {x:  1, y:  1},
    );

    for part in parts {
        if part.part != '*' {
            continue;
        }
        //println!("Gear ({}, {})", part.position.x, part.position.y);
        let mut gear_nums: Vec<i32> = vec!();
        for number in numbers {
            //println!("- Number `{}` ({}, {})", number.number, number.position.x, number.position.y);
            let mut found = false;
            for kernel_pos in &positions {
                let try_pos = Vec2 {x: part.position.x + kernel_pos.x, y: part.position.y + kernel_pos.y};
                //print!(" > ({}, {}) ", try_pos.x, try_pos.y);
                for num_pos_x in 0 .. number.size {
                    if (try_pos.x == number.position.x + num_pos_x as i32) &&
                        (try_pos.y == number.position.y) {
                        found = true;
                        //print!("#");
                        break;
                    }
                }
                //println!("");
                if found {
                    gear_nums.push(number.number);
                    break;
                }
            }
        }
        /*for n in &gear_nums {
            print!("{} ", *n);
        }*/
        if gear_nums.len() != 2 {
            //println!("");
            continue;
        }
        let mul = gear_nums[0] * gear_nums[1];
        //println!(" = {}", mul);
        sum += mul;
    }
    sum
}

pub fn main() -> Result<(), std::io::Error> {
    let data = read_file("input03.txt")?;

    /*let data = vec!{
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    };*/

    println!("Day 03:");

    let part_list = get_parts(&data);
    let part_numbers = get_numbers(&data);

    let part_num_sum = get_part_num_sum(&part_numbers, &part_list);

    println!("\tPart 1 - part num sum: {}", part_num_sum);

    let gear_ratio_sum = get_gear_ratios(&part_numbers, &part_list);

    println!("\tPart 2 - gear ratios sum: {}", gear_ratio_sum);

    Ok(())
}