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

    let input = match fs::read_to_string("./test.txt") {
        Ok(string) => string,
        Err(_) => panic!("File not found :("),
    };

    let seeds: Vec<i64> = parse_seeds(&input);
    println!("Test seeds: {:?}", seeds);
    let mappings: HashMap<String, Vec<RangeMap>> = parse_mappings(&input);
    println!("{:?}", mappings.get("seed-to-soil").unwrap());
}


