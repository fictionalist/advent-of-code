#[derive(Debug, PartialEq)]
enum Direction {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    StartPos,
    Empty
}

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

struct Node {
    direction: Direction,
}

/*
fn get_test_data() -> String {
    "
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...
    ".to_string()
    // custom test case, result is 8 steps
}
*/

fn get_opposite_direction(v: (i32, i32)) -> (i32, i32) {
    match v {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        _ => (0, 0)
    }
}

fn get_node_constraints(direction: &Direction) -> Vec<(i32, i32)> {
    let mut constraints: Vec<(i32, i32)> = Vec::new();
    match direction {
        Direction::StartPos => constraints = vec![LEFT, UP, RIGHT, DOWN],
        Direction::UpDown => constraints = vec![UP, DOWN],
        Direction::LeftRight => constraints = vec![LEFT, RIGHT],
        Direction::UpRight => constraints = vec![UP, RIGHT],
        Direction::UpLeft => constraints = vec![UP, LEFT],
        Direction::DownRight => constraints = vec![DOWN, RIGHT],
        Direction::DownLeft => constraints = vec![DOWN, LEFT],
        _ => {}
    }

    constraints
}

fn get_number_of_steps(lines: Vec<&str>) -> i32 { 
    let mut starting_position = (0, 0);
    let mut node_map: Vec<Vec<Node>> = Vec::new();
    let mut steps = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<Node> = Vec::new();
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '|' => row.push(Node { direction: Direction::UpDown }),
                '-' => row.push(Node { direction: Direction::LeftRight }),
                'L' => row.push(Node { direction: Direction::UpRight }),
                'J' => row.push(Node { direction: Direction::UpLeft }),
                '7' => row.push(Node { direction: Direction::DownLeft }),
                'F' => row.push(Node { direction: Direction::DownRight }),
                'S' => {starting_position = (x as i32, y as i32); row.push(Node { direction: Direction::StartPos }); },
                _ => row.push(Node { direction: Direction::Empty })
            }
        }
        node_map.push(row);
    }

    // Left, Up, Right, Down
    let check_positions: Vec<(i32, i32)> = vec![LEFT, UP, RIGHT, DOWN];

    let mut cursor = starting_position;
    let mut previous_position = cursor;

    loop {
        //print!("Step {:3}: {:?} {:?} ", steps, cursor, &node_map[cursor.1 as usize][cursor.0 as usize].direction);
        for (_idx, pos) in check_positions.iter().enumerate() {
            let x = cursor.0 + pos.0;
            let y = cursor.1 + pos.1;
            let test_pos = (x, y);
            
            if (test_pos.0 >= 0) && (test_pos.0 <= (node_map[0].len() - 1) as i32) &&
                (test_pos.1 >= 0) && (test_pos.1 <= (node_map.len() - 1) as i32) {
                if test_pos == previous_position {
                    continue;
                }

                let test_node_constraints = get_node_constraints(&node_map[test_pos.1 as usize][test_pos.0 as usize].direction);
                let node_constraints = get_node_constraints(&node_map[cursor.1 as usize][cursor.0 as usize].direction);

                let mut found = false;
                for constraint in &node_constraints {
                    if test_node_constraints.iter().any(|dir| *constraint == get_opposite_direction(*dir)) && constraint == pos {
                        previous_position = cursor;
                        cursor = test_pos;
                        found = true;
                    }
                }
                if found {
                    break;
                }
            }
        }
        steps += 1;
        if (node_map[cursor.1 as usize][cursor.0 as usize].direction == Direction::StartPos && previous_position != cursor) {
            break;
        }
    }
    steps / 2
}

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input10.txt")?;

    //let data = get_test_data();
    let lines: Vec<&str> = data.trim().lines().collect();

    println!("Day 10:");

    let start = std::time::Instant::now();
    let num_steps = get_number_of_steps(lines);

    println!("\tPart 1 - Largest number of steps: {} ({} ms)", num_steps, start.elapsed().as_millis());

    Ok(())
}