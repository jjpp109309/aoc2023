#![allow(unused_variables, dead_code)]

use std::fs;
use d05::*;
use std::collections::HashMap;

fn main() {
    let keys = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    // ------------------------------------------------------------------------
    // part 1: test
    // ------------------------------------------------------------------------

    let input = match fs::read_to_string("./test.txt") {
        Ok(string) => string,
        Err(_) => panic!("File not found :("),
    };

    let seeds: Vec<i64> = parse_seeds(&input);
    println!("Test seeds: {:?}", seeds);
    let mappings: HashMap<String, Vec<RangeMap>> = parse_mappings(&input);

    let ranges = mappings.get("seed-to-soil").unwrap();
    let seed = vec![RangeMap { start: 45, end:55, delta: None }];

    println!("test input\n{:?}\nmapping\n{:?}", seed, ranges);
    let outputs = map_ranges(&seed, ranges);
    println!("test outputs\n{:?}", outputs);

        for seed in seeds {
        let mut seed_range = vec![RangeMap {start: seed, end:seed+1, delta: None}];

        for key in &keys {
            let mapping = mappings.get(&key.to_string()).unwrap();
            seed_range = map_ranges(&seed_range, mapping);
        }

        println!("seed {:?}", seed);
        println!("output\n{:?}", seed_range);
    }
    // ------------------------------------------------------------------------
    // part 1
    // ------------------------------------------------------------------------

    let input = match fs::read_to_string("./input.txt") {
        Ok(string) => string,
        Err(_) => panic!("File not found :("),
    };

    let seeds: Vec<i64> = parse_seeds(&input);
    println!("seeds: {:?}", seeds);
    let mappings: HashMap<String, Vec<RangeMap>> = parse_mappings(&input);

    let mut locations: Vec<RangeMap> = Vec::new();

    for seed in seeds {
        let mut seed_range = vec![RangeMap {start: seed, end:seed+1, delta: None}];

        for key in &keys {
            let mapping = mappings.get(&key.to_string()).unwrap();
            seed_range = map_ranges(&seed_range, mapping);
        }
        locations.append(&mut seed_range);
    }
    println!("Part 1:\n{:?}", locations.iter().map(|x| x.start).min());
}


