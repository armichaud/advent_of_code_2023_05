use std::fs::read_to_string;
use std::collections::HashMap;
use std::str::Split;

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
type Legend<'a> = HashMap<&'a str, Vec<SeedMap>>;

fn get_maps<'a>(sections: Split<'a, &'a str>) -> Legend {
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
    maps
}


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
        .collect::<Vec<usize>>();
    let maps = get_maps(sections);
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

fn get_min(maps: &Legend, key_index: usize, range: SeedRange) -> f64 {
    if key_index == KEYS.len() {
        return range.start as f64;
    }
    let mut min_location = f64::INFINITY;
    let mut ranges: Vec<SeedRange> = Vec::from(&[range]);
    let seed_maps = maps.get(KEYS[key_index]).unwrap();
    while ranges.len() > 0 {
        let range = ranges.pop().unwrap();
        for seed_map in seed_maps {
            let map_start = seed_map.source_range_start;
            let map_end = map_start + seed_map.range_length;
            let start_diff = range.start as i128 - map_start as i128;
            let end_diff = map_end as i128 - range.end as i128;
            // No overlap
            if range.end < map_start || range.start > map_end {
                continue;
            }
            // Seed range is entirely within map range
            if map_start <= range.start && range.end <= map_end {
                min_location = min_location.min(
                    get_min(
                        maps, 
                        key_index + 1, 
                        SeedRange {start: seed_map.destination_range_start + start_diff as usize, end: seed_map.destination_range_start + end_diff as usize}
                    )
                );
                break;
            }
            // Seed range starts before map range but ends within map range 
            if range.start < map_end && map_end <= range.end {
                min_location = min_location.min(
                    get_min(
                        maps, 
                        key_index + 1, 
                        SeedRange {start: seed_map.destination_range_start, end: seed_map.destination_range_start + end_diff as usize}
                    )
                );
                ranges.push(SeedRange {start: range.start, end: map_start});
                break;
            }
            // Seed range starts within map range but ends after map range 
            if range.start < map_end && range.end > map_end {
                min_location = min_location.min(
                    get_min(
                        maps, 
                        key_index + 1, 
                        SeedRange {start: seed_map.destination_range_start + start_diff as usize, end: seed_map.destination_range_start + seed_map.range_length}
                    )
                );
                ranges.push(SeedRange {start: map_end, end: range.end});
                break;
            }
            // Map range is entirely within seed range
            if range.start < map_start && map_end < range.end {
                min_location = min_location.min(
                    get_min(
                        maps, 
                        key_index + 1, 
                        SeedRange {start: seed_map.destination_range_start + start_diff as usize, end: seed_map.destination_range_start + end_diff as usize}
                    )
                );
                ranges.push(SeedRange {start: map_start, end: range.start});
                ranges.push(SeedRange {start: map_end, end: range.end});
                break;
            }
            
        }
    }
    min_location
}

fn range_solution(file: &str) -> f64 {
    let contents = read_to_string(file).expect("Something went wrong reading the file");
    let mut sections = contents.as_str().split("\n\n");
    let mut split = sections.next().unwrap().split_whitespace();
    split.next();
    let ranges = split
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|x| { 
            let start = x[0].parse::<usize>().unwrap();
            let end = x[1].parse::<usize>().unwrap() + start;
            SeedRange { start, end } 
        })
        .collect::<Vec<SeedRange>>(); 
    let maps = get_maps(sections);
    let mut min_location = f64::INFINITY;
    for range in ranges {
        min_location = min_location.min(get_min(&maps, 0, range));
    }
    min_location
}

fn main() {
    assert_eq!(part_1("example.txt"), 35 as f64);
    assert_eq!(part_1("input.txt"), 535088217 as f64);
    assert_eq!(range_solution("example.txt"), 46 as f64);
    assert_eq!(range_solution("input.txt"), 0 as f64);
}
