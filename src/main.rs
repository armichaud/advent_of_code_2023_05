use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct SeedMap {
    source_range_start: usize,
    destination_range_start: usize,
    range_length: usize
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    start: usize,
    end: usize
}

const KEYS: [&str; 7] = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity"];

fn part_1(file: &str) -> f64 {
    let contents = read_to_string(file).expect("Something went wrong reading the file");
    let mut sections = contents.as_str().split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>().clone();
    let mut maps = HashMap::<&str, Vec<SeedMap>>::new();
    for section in sections {
        let mut lines = section.lines();
        let mut map_type = lines.next().unwrap().split_whitespace().nth(0).unwrap().split("-");
        let source_category = map_type.nth(0).unwrap();
        for line in lines {
            let nums = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let seed_map = SeedMap {
                source_range_start: nums[1],
                destination_range_start: nums[0],
                range_length: nums[2]
            };
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

fn range_solution(file: &str) -> f64 {
    let contents = read_to_string(file).expect("Something went wrong reading the file");
    let mut sections = contents.as_str().split("\n\n");
    let mut split = sections.next().unwrap().split_whitespace();
    // skip first in split
    split.next();
    // map through remainder of split in pairs
    let ranges = split
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|x| { 
            let start = x[0].parse::<usize>().unwrap();
            let end = x[1].parse::<usize>().unwrap();
            SeedRange { start, end } 
        })
        .collect::<Vec<SeedRange>>(); 


    0.0
}

fn main() {
    assert_eq!(part_1("example.txt"), 35 as f64);
    assert_eq!(part_1("input.txt"), 535088217 as f64);
    assert_eq!(range_solution("example.txt"), 46 as f64);
    assert_eq!(range_solution("input.txt"), 0 as f64);
}
