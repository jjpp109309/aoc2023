use std::fs;
use std::collections::HashMap;
use std::ops::Range;
use regex::Regex;

struct AlmaMap<Idx> {
    source: Range<Idx>,
    destination: Range<Idx>,
}

fn main() {
    let input = match fs::read_to_string("./test.txt") {
        Ok(string) => string,
        Err(_) => panic!("File not found :("),
    };

    let seeds: Vec<u32> = parse_seeds(&input);
    println!("Test seeds: {:?}", seeds);
    let mappings: HashMap<String, AlmaMap<u32>> = parse_mappings(&input);
    
}

fn parse_seeds(input: &str) -> Vec<u32> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.lines().next().unwrap())
        .map(|x| x.as_str().parse::<u32>().unwrap())
        .collect()
}

fn parse_mappings(input: &str) -> HashMap<String, AlmaMap<u32>> {
    todo!()
}






enum number {
    seed,
    soil,
    fertilizer,
    water,
    light,
    temperature,
    humidity,
}

impl number {
    fn map_arm(&self) -> number {
        match self {
            number::seed => number::soil,
            number::soil => number::fertilizer,
            number::fertilizer => number::water,
            number::water => number::light,
            number::light => number::temperature,
            number::temperature => number::humidity,
            number::humidity => todo!(),
        }
    }
}
