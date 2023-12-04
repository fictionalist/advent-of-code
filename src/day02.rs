// cube conundrum
use crate::parse_file::read_file;

struct Colors {
    red: i32,
    green: i32,
    blue: i32
}

fn get_count_from_slice(slice: &str, color: &str) -> i32 {
    let mut count = 0;
    if let Some(idx) = slice.find(color) {
        count = (&slice[idx - 3 .. idx]).trim().parse::<i32>().unwrap();
    }
    count
}

fn get_max_cube_colors(line: &String) -> Colors {
    let mut input = line.clone();

    let mut colors = Colors { red: 0, green: 0, blue: 0}; // red, green, blue

    input = input[(input.find(": ").unwrap() + 1) ..].to_string();

    let mut sets = input.split(";");

    while let Some(s) = sets.next() {
        let red = get_count_from_slice(s, "red");
        let green = get_count_from_slice(s, "green");
        let blue = get_count_from_slice(s, "blue");
        if red > colors.red {
            colors.red = red;
        }
        if green > colors.green {
            colors.green = green;
        }
        if blue > colors.blue {
            colors.blue = blue;
        }
    }

    //println!("({}, {}, {})", colors[0], colors[1], colors[2]);

    colors
}

fn get_game_id(line: &String) -> i32 {
    let colon_position = line.find(':').unwrap();
    let space_position = line.find(' ').unwrap();
    let i = (&line[(space_position + 1) .. colon_position]).parse::<i32>().unwrap();

    i
}

pub fn main() -> Result<(), std::io::Error> {
    let data = read_file("input02.txt")?;

    /*let data = vec!{
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    };*/

    println!("Day 02:");

    let max_colors = Colors { red: 12, green: 13, blue: 14 };
    let mut id_sum = 0;
    let mut total_cube_power = 0;

    for line in &data {
        let id = get_game_id(&line);
        let colors = get_max_cube_colors(&line);

        //print!("Game ID: {} | Colors: (R: {}, G: {}, B: {})", id, colors.red, colors.green, colors.blue);

        let cube_power = colors.red * colors.green * colors.blue;
        //print!(" Cube power: {} ", cube_power);
        total_cube_power += cube_power;
        
        if colors.red > max_colors.red || colors.green > max_colors.green || colors.blue > max_colors.blue {
            //println!("!Impossible")
        } else {
            id_sum += id;
            //println!("");
        }
    }

    println!("\tPart 1 - ID sum: {}", id_sum);
    println!("\tPart 2 - total cube power: {}", total_cube_power);

    drop(data);

    Ok(())
}