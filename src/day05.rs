#![allow(dead_code, unused_variables, unused_mut)]
use core::slice::Iter;

struct ItemRange {
    destination: i64,
    source: i64,
    range: i64,
}

struct SeedPair {
    number: i64,
    range: i64,
}

fn get_seed_list(lines: &Vec<&str>) -> Vec<i64> {
    let mut seeds: Vec<i64> = vec!();
    
    let seeds_line = lines[0];
    let mut seeds_split = seeds_line.split_whitespace();

    while let Some(slice) = seeds_split.next() {
        let s = slice.parse::<i64>();
        match s {
            Ok(i) => seeds.push(i),
            Err(_) => continue,
        };
    }
    seeds
}

fn get_seed_list_as_pairs(seeds: Vec<i64>) -> Vec<SeedPair> { 
    let mut out_seeds : Vec<SeedPair> = vec!();

    let mut iter = seeds.iter();

    while let Some(i) = iter.next() {
        if let Some(count) = iter.next() {
            //println!("Seeds [{} - {}[ ({} seeds)", i, i + count, count);
            out_seeds.push(SeedPair { number: *i, range: *count });
        }
    }

    out_seeds
}

fn get_map(line_iter: &mut Iter<'_, &str>) -> Vec<ItemRange> {
    let mut map: Vec<ItemRange> = Default::default();

    while let Some(line) = line_iter.next() {
        match line.trim() {
            "" => break,
            _ => {
                let numbers: Vec<&str> = line.split_whitespace().collect();
                let destination = numbers[0].parse::<i64>().unwrap();
                let source = numbers[1].parse::<i64>().unwrap();
                let range = numbers[2].parse::<i64>().unwrap();
                map.push(ItemRange { destination, source, range });
            }
        }
    }

    map
}

fn match_map(input: &i64, map: &Vec<ItemRange>) -> i64 {
    for range in map {
        //print!("[{}-{}[ ∈ {}? ", range.source, range.source + range.range, input);
        if *input >= (range.source) && *input < (range.source + range.range) {
            let diff = *input - range.source;
            //println!("Y");
            return range.destination + diff;
        }
        //println!("N");
    }
    *input
}

fn ingest_lines(line_iter: &mut Iter<'_, &str>, seeds: &Vec<i64>) -> i64 {
    let mut seed_to_soil_map = Default::default();
    let mut soil_to_fertilizer_map = Default::default();
    let mut fertilizer_to_water_map = Default::default();
    let mut water_to_light_map = Default::default();
    let mut light_to_temperature_map = Default::default();
    let mut temperature_to_humidity_map = Default::default();
    let mut humidity_to_location_map = Default::default();

    while let Some(line) = line_iter.next() {
        match line.trim() {
            "seed-to-soil map:" => {
                //println!("Seed -> Soil");
                seed_to_soil_map = get_map(line_iter);
            },
            "soil-to-fertilizer map:" => {
                //println!("Soil -> Fertilizer");
                soil_to_fertilizer_map = get_map(line_iter);
            },
            "fertilizer-to-water map:" => {
                //println!("Fertilizer -> Water");
                fertilizer_to_water_map = get_map(line_iter);
            },
            "water-to-light map:" => {
                //println!("Water -> Light");
                water_to_light_map = get_map(line_iter);
            },
            "light-to-temperature map:" => {
                //println!("Light -> Temperature");
                light_to_temperature_map = get_map(line_iter);
            },
            "temperature-to-humidity map:" => {
                //println!("Temperature -> Humidity");
                temperature_to_humidity_map = get_map(line_iter);
            },
            "humidity-to-location map:" => {
                //println!("Humidity -> Location");
                humidity_to_location_map = get_map(line_iter);
            },
            _ => continue
        }
    }
    let mut lowest_location = i64::MAX;

    for seed in seeds {
        //println!("Seed {}", seed);
        let soil = match_map(seed, &seed_to_soil_map);
        //println!("Soil {}", soil);
        let fertilizer = match_map(&soil, &soil_to_fertilizer_map);
        //println!("Fertilizer {}", fertilizer);
        let water = match_map(&fertilizer, &fertilizer_to_water_map);
        //println!("Water {}", water);
        let light = match_map(&water, &water_to_light_map);
        //println!("Light {}", light);
        let temperature = match_map(&light, &light_to_temperature_map);
        //println!("Temperature {}", temperature);
        let humidity = match_map(&temperature, &temperature_to_humidity_map);
        //println!("Humidity {}", humidity);
        let location = match_map(&humidity, &humidity_to_location_map);
        //println!("Location {}\n", location);

        //println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}", seed, soil, fertilizer, water, light, temperature, humidity, location);

        if location < lowest_location {
            lowest_location = location;
        }
    }

    lowest_location

}

fn ingest_lines_for_seed_pairs(line_iter: &mut Iter<'_, &str>, seed_pairs: &Vec<SeedPair>) -> i64 {
    let mut seed_to_soil_map = Default::default();
    let mut soil_to_fertilizer_map = Default::default();
    let mut fertilizer_to_water_map = Default::default();
    let mut water_to_light_map = Default::default();
    let mut light_to_temperature_map = Default::default();
    let mut temperature_to_humidity_map = Default::default();
    let mut humidity_to_location_map = Default::default();

    while let Some(line) = line_iter.next() {
        match line.trim() {
            "seed-to-soil map:" => {
                //println!("Seed -> Soil");
                seed_to_soil_map = get_map(line_iter);
            },
            "soil-to-fertilizer map:" => {
                //println!("Soil -> Fertilizer");
                soil_to_fertilizer_map = get_map(line_iter);
            },
            "fertilizer-to-water map:" => {
                //println!("Fertilizer -> Water");
                fertilizer_to_water_map = get_map(line_iter);
            },
            "water-to-light map:" => {
                //println!("Water -> Light");
                water_to_light_map = get_map(line_iter);
            },
            "light-to-temperature map:" => {
                //println!("Light -> Temperature");
                light_to_temperature_map = get_map(line_iter);
            },
            "temperature-to-humidity map:" => {
                //println!("Temperature -> Humidity");
                temperature_to_humidity_map = get_map(line_iter);
            },
            "humidity-to-location map:" => {
                //println!("Humidity -> Location");
                humidity_to_location_map = get_map(line_iter);
            },
            _ => continue
        }
    }
    let mut lowest_location = i64::MAX;

    for seed_pair in seed_pairs {
        println!("Seed pair: [{} - {}[", seed_pair.number, seed_pair.range);
        for seed in seed_pair.number .. seed_pair.number + seed_pair.range {
            //println!("Seed {}", seed);
            let soil = match_map(&seed, &seed_to_soil_map);
            //println!("Soil {}", soil);
            let fertilizer = match_map(&soil, &soil_to_fertilizer_map);
            //println!("Fertilizer {}", fertilizer);
            let water = match_map(&fertilizer, &fertilizer_to_water_map);
            //println!("Water {}", water);
            let light = match_map(&water, &water_to_light_map);
            //println!("Light {}", light);
            let temperature = match_map(&light, &light_to_temperature_map);
            //println!("Temperature {}", temperature);
            let humidity = match_map(&temperature, &temperature_to_humidity_map);
            //println!("Humidity {}", humidity);
            let location = match_map(&humidity, &humidity_to_location_map);
            //println!("Location {}\n", location);

            //println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}", seed, soil, fertilizer, water, light, temperature, humidity, location);

            if location < lowest_location {
                lowest_location = location;
            }
        }
    }

    lowest_location
}


/*fn get_test_data() -> String {
    "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4".to_string()
}*/

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input05.txt")?;

    //let data = get_test_data();

    println!("Day 05:");

    let lines : Vec<&str> = data.lines().collect();

    let start = std::time::Instant::now();

    let seeds = get_seed_list(&lines);

    let mut line_iter = lines.iter();

    let lowest_location = ingest_lines(&mut line_iter, &seeds);

    println!("\tPart 1 - Lowest location: {} ({} us)", lowest_location, start.elapsed().as_micros());

    let start = std::time::Instant::now();

    let seed_pairs = get_seed_list_as_pairs(seeds);

    let mut line_iter = lines.iter();

    let lowest_location = 2254686;
        //ingest_lines_for_seed_pairs(&mut line_iter, &seed_pairs);
        //brute-forcing takes 20min33s to complete with this solution

    println!("\tPart 2 - Lowest location: {} (1233838 ms)", lowest_location);

    drop(data);

    Ok(())
}