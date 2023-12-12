#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum Space {
    #[default] Empty,
    Expand,
    Galaxy
}
/*
fn get_test_data() -> String {
    "
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    ".to_string()
}
*/
fn transpose_map<T>(map: &Vec<Vec<T>>) -> Vec<Vec<T>> where T : Copy + Default {
    let mut out: Vec<Vec<T>> = Vec::new();
    out.resize(map[0].len(), Default::default());

    for y in 0 .. out.len() {
        let mut row: Vec<T> = Vec::new();
        row.resize(map.len(), Default::default());
        for x in 0 .. row.len() {
            row[x] = map[x][y];
        }
        out[y] = row;
    }

    out
}

fn dilate_map(map: Vec<Vec<Space>>, substitute: bool) -> Vec<Vec<Space>> {
    let mut dilate_y: Vec<Vec<Space>> = Vec::new();

    for row in &map {
        if row.iter().all(|p| *p == Space::Empty) {
            if substitute {
                let mut v = Vec::new();
                v.resize(row.len(), Space::Expand);
                dilate_y.push(v);
            } else {
                dilate_y.push(row.clone());
                dilate_y.push(row.clone());
            }
        } else {
            dilate_y.push(row.clone());
        }
    }

    let transposed = transpose_map(&dilate_y);

    let mut out: Vec<Vec<Space>> = Vec::new();
    for row in &transposed {
        if row.iter().all(|p| *p == Space::Empty || *p == Space::Expand) {
            if substitute {
                let mut v = Vec::new();
                v.resize(row.len(), Space::Expand);
                out.push(v);
            } else {
                out.push(row.clone());
                out.push(row.clone());
            }
        } else {
            out.push(row.clone());
        }
    }

    transpose_map(&out)
}

fn part_one(lines: &Vec<&str>) -> i32 {
    let mut sum = 0;

    let mut map: Vec<Vec<Space>> = Vec::new();

    for line in lines.iter() {
        let mut row : Vec<Space> = Vec::new();
        for c in line.trim().chars() {
            match c {
                '.' => row.push(Space::Empty),
                _ => row.push(Space::Galaxy)
            }
        }
        map.push(row);
    }

    let dilated_map = dilate_map(map, false);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (y, row) in dilated_map.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if *i == Space::Galaxy {
                galaxies.push((x, y));
            }
        }
    }

    while galaxies.len() > 1 {
        let left_galaxy = galaxies[0];
        for galaxy in galaxies.iter().skip(1) {
            let manhattan_distance = usize::abs_diff(left_galaxy.0, galaxy.0) + usize::abs_diff(left_galaxy.1, galaxy.1);
            sum += manhattan_distance as i32;
        }
        galaxies.remove(0);
    }

    sum
}

fn part_two(lines: &Vec<&str>) -> i64 {
    let mut sum = 0;

    let mut map: Vec<Vec<Space>> = Vec::new();

    for line in lines.iter() {
        let mut row : Vec<Space> = Vec::new();
        for c in line.trim().chars() {
            match c {
                '.' => row.push(Space::Empty),
                _ => row.push(Space::Galaxy)
            }
        }
        map.push(row);
    }

    let dilated_map = dilate_map(map, true);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    const EXPAND_DISTANCE: usize = 1_000_000;

    let mut expand_x: usize = 0;
    let mut expand_y: usize = 0;
    for (y, row) in dilated_map.iter().enumerate() {
        if row.iter().all(|i| *i == Space::Expand) {
            expand_y += 1;
        } else {
            for (x, i) in row.iter().enumerate() {
                match *i {
                    Space::Empty => {},
                    Space::Expand => {
                        expand_x += 1;
                    },
                    Space::Galaxy => {
                        galaxies.push((x + (EXPAND_DISTANCE * expand_x) - expand_x, y + (EXPAND_DISTANCE * expand_y) - expand_y));
                    }
                }
            }
            expand_x = 0;
        }
    }

    while galaxies.len() > 1 {
        let left_galaxy = galaxies[0];
        for galaxy in galaxies.iter().skip(1) {
            let manhattan_distance = usize::abs_diff(left_galaxy.0, galaxy.0) + usize::abs_diff(left_galaxy.1, galaxy.1);
            //println!("{:?} -> {:?} = {}", left_galaxy, *galaxy, manhattan_distance);
            sum += manhattan_distance as i64;
            //return sum;
        }
        galaxies.remove(0);
    }

    sum
}

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input11.txt")?;
    //let data = get_test_data();

    println!("Day 11:");

    let lines: Vec<&str> = data.trim().lines().collect();

    let start = std::time::Instant::now();

    let sum = part_one(&lines);

    println!("\tPart 1 - Sum of lengths between galaxies: {} ({} ms)", sum, start.elapsed().as_millis());

    let start = std::time::Instant::now();
    
    let sum = part_two(&lines);

    println!("\tPart 2 - Sum of lengths between very old galaxies: {} ({} ms)", sum, start.elapsed().as_millis());
    
    Ok(())
}