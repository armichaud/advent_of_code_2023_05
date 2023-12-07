use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
struct SeedMap {
    source_range_start: usize,
    destination_range_start: usize,
    range_length: usize
}

const KEYS: [&str; 7] = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity"];

fn part_1(file_path: &str) -> f64 {
    let contents = read_to_string(file_path).expect("Something went wrong reading the file");
    let mut sections = contents.as_str().split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>()
        .unwrap())
        .collect::<Vec<_>>();
    let mut maps = HashMap::<&str, Vec<SeedMap>>::new();
    // iterate through remainder of sections and map to SeedMap
    for section in sections {
        let mut lines = section.lines();
        let mut map_type = lines.next().unwrap().split(" ").nth(0).unwrap().split("-");
        let source_category = map_type.nth(0).unwrap();
        // let destination_category = map_type.nth(1).unwrap();
        for line in lines {
            let nums = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let seed_map = SeedMap {
                source_range_start: nums[1],
                destination_range_start: nums[0],
                range_length: nums[2]
            };
            // insert seed map into vec for source category key
            maps.entry(source_category).or_insert_with(Vec::new).push(seed_map);
        }
    }

    let mut min_location = f64::INFINITY;
    for seed in seeds {
        let mut n = seed;
        for key in KEYS {
            let seed_maps = maps.get(key).unwrap();
            for seed_map in seed_maps {
                if n >= seed_map.source_range_start && n < seed_map.source_range_start + seed_map.range_length {
                    n = seed_map.destination_range_start + (n - seed_map.source_range_start);
                    break;
                }
            }
        }
        min_location = min_location.min(n as f64);
    }
    min_location
}

fn part_2_slow(file_path: &str) -> f64  {
    let contents = read_to_string(file_path).expect("Something went wrong reading the file");
    let mut sections = contents.as_str().split("\n\n");
    let seed_ranges = sections
        .next()
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>()
        .unwrap())
        .collect::<Vec<_>>();
    let mut seeds = Vec::<usize>::new();
    for seed in seed_ranges.chunks(2) {
        for i in seed[0]..seed[1] {
            seeds.push(i);
        }
    }
    let mut maps = HashMap::<&str, Vec<SeedMap>>::new();
    // iterate through remainder of sections and map to SeedMap
    for section in sections {
        let mut lines = section.lines();
        let mut map_type = lines.next().unwrap().split(" ").nth(0).unwrap().split("-");
        let source_category = map_type.nth(0).unwrap();
        // let destination_category = map_type.nth(1).unwrap();
        for line in lines {
            let nums = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let seed_map = SeedMap {
                source_range_start: nums[1],
                destination_range_start: nums[0],
                range_length: nums[2]
            };
            // insert seed map into vec for source category key
            maps.entry(source_category).or_insert_with(Vec::new).push(seed_map);
        }
    }

    let mut min_location = f64::INFINITY;
    for seed in seeds {
        let mut n = seed;
        for key in KEYS {
            let seed_maps = maps.get(key).unwrap();
            for seed_map in seed_maps {
                if n >= seed_map.source_range_start && n < seed_map.source_range_start + seed_map.range_length {
                    n = seed_map.destination_range_start + (n - seed_map.source_range_start);
                    break;
                }
            }
        }
        min_location = min_location.min(n as f64);
    }
    min_location
}

fn part_2_build_single_map(file_path: &str) -> f64  {
    let contents = read_to_string(file_path).expect("Something went wrong reading the file");
    let mut sections = contents.as_str().split("\n\n");
    let seed_ranges = sections
        .next()
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>()
        .unwrap())
        .collect::<Vec<_>>();
    let mut seeds = Vec::<usize>::new();
    for seed in seed_ranges.chunks(2) {
        for i in seed[0]..seed[1] {
            seeds.push(i);
        }
    }
    let mut maps = HashMap::<&str, Vec<SeedMap>>::new();
    // iterate through remainder of sections and map to SeedMap
    for section in sections {
        let mut lines = section.lines();
        let mut map_type = lines.next().unwrap().split(" ").nth(0).unwrap().split("-");
        let source_category = map_type.nth(0).unwrap();
        // let destination_category = map_type.nth(1).unwrap();
        for line in lines {
            let nums = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let seed_map = SeedMap {
                source_range_start: nums[1],
                destination_range_start: nums[0],
                range_length: nums[2]
            };
            // insert seed map into vec for source category key
            maps.entry(source_category).or_insert_with(Vec::new).push(seed_map);
        }
    }

    let mut min_location = f64::INFINITY;
    let mut master_map = HashMap::<usize, usize>::new();

    for key in KEYS.iter().rev() {
        for i in 0..maps.get(key).unwrap().len() {
            let seed_map = &maps.get(key).unwrap()[i];
            for source in seed_map.source_range_start..seed_map.source_range_start + seed_map.range_length {
                let destination = seed_map.destination_range_start + (source - seed_map.source_range_start);
                let master_map_entry = master_map.get_mut(&destination);
                if master_map_entry.is_some() {
                    let old_destination = *master_map_entry.unwrap();
                    master_map.remove(&destination);
                    master_map.insert(source, old_destination);
                } else {
                    master_map.insert(source, destination);
                }
            }
        }
    }
    for seed in seeds {
        let mut n = seed;
        if master_map.contains_key(&seed) {
            n = *master_map.get(&seed).unwrap();
        }
        min_location = min_location.min(n as f64);
    }

    min_location
}

fn main() {
    assert_eq!(part_1("example.txt"), 35 as f64);
    assert_eq!(part_1("input.txt"), 535088217 as f64);
    // println!("Part 2: {}", part_2_slow("input.txt"));
    // println!("Part 2: {}", part_2_build_single_map("input.txt"));
}
