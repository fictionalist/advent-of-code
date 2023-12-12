/*fn get_test_data() -> String {
    "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45".to_string()
}*/

fn get_derivatives_until_constant(initial_values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut derivatives: Vec<Vec<i64>> = Vec::new();

    derivatives.push(initial_values);

    loop {
        let mut deriv_iter = derivatives[derivatives.len() - 1].iter();
        let mut left = deriv_iter.next().unwrap();

        let mut temp_vec: Vec<i64> = Vec::new();
        while let Some(right) = &deriv_iter.next() {
            let diff = *right - left;
            temp_vec.push(diff);
            left = right;
        }

        if temp_vec.iter().all(|i| *i == 0) {
            break;
        }
        derivatives.push(temp_vec);
    }

    derivatives
}

fn extrapolate_next(line: &str) -> i64 {
    let nums_str: Vec<&str> = line.trim().split_whitespace().collect();

    let mut nums: Vec<i64> = Vec::new();
    for num in nums_str {
        //print!("{} ", num);
        nums.push(num.parse::<i64>().unwrap());
    }
    //println!("");
    let mut derivatives = get_derivatives_until_constant(nums);

    let mut rev_derivs = derivatives.iter_mut().rev();
    let mut diff = *rev_derivs.next().unwrap().last().unwrap();
    for derivs in rev_derivs {
        let item = derivs.last().unwrap_or(&0);
        let d = item + diff;
        derivs.push(d);
        diff = d;

        //println!("{:?}", derivs);
    }

    *derivatives.first().unwrap().last().unwrap()
}

fn extrapolate_previous(line: &str) -> i64 {
    let nums_str: Vec<&str> = line.trim().split_whitespace().collect();

    let mut nums: Vec<i64> = Vec::new();
    for num in nums_str {
        //print!("{} ", num);
        nums.push(num.parse::<i64>().unwrap());
    }
    //println!("");
    let mut derivatives = get_derivatives_until_constant(nums);

    let mut rev_derivs = derivatives.iter_mut().rev();
    let mut diff = *rev_derivs.next().unwrap().first().unwrap();
    for derivs in rev_derivs {
        let item = derivs.first().unwrap_or(&0);
        let d = item - diff;
        derivs.insert(0, d);
        diff = d;

        //println!("{:?}", derivs);
    }

    *derivatives.first().unwrap().first().unwrap()
}

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input09.txt")?;

    //let data = get_test_data();

    let lines: Vec<&str> = data.trim().lines().collect();

    println!("Day 09:");

    let start = std::time::Instant::now();

    let mut sum = 0;
    for line in &lines {
        sum += extrapolate_next(line);
    }

    println!("\tPart 1 - Sum of extrapolated values: {} ({} ms)", sum, start.elapsed().as_millis());

    let start = std::time::Instant::now();
    let mut sum = 0;
    for line in &lines {
        sum += extrapolate_previous(line);
    }
    println!("\tPart 2 - Sum of extrapolated previous values: {} ({} ms)", sum, start.elapsed().as_millis());

    Ok(())
}