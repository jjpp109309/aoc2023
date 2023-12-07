use std::fs;
use std::collections::HashMap;
use std::ops::Range;
use regex::Regex;

#[derive(Debug)]
struct RangeMap<T> {
    source: Range<T>,
    destination: Range<T>,
}

fn main() {
    let input = match fs::read_to_string("./test.txt") {
        Ok(string) => string,
        Err(_) => panic!("File not found :("),
    };

    let seeds: Vec<u32> = parse_seeds(&input);
    println!("Test seeds: {:?}", seeds);
    let mappings: HashMap<String, Vec<RangeMap<u32>>> = parse_mappings(&input);

    for i in mappings.get("seed-to-soil").unwrap() {
        println!("{:?}", i);
        for j in i.source.clone() {
            println!("{}", j);
        }
    }
    
}

fn parse_seeds(input: &str) -> Vec<u32> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.lines().next().unwrap())
        .map(|x| x.as_str().parse::<u32>().unwrap())
        .collect()
}

fn parse_mappings(input: &str) -> HashMap<String, Vec<RangeMap<u32>>> {
    let mut range_mapping: HashMap<String, Vec<RangeMap<u32>>> = HashMap::new();

    let re_maps = Regex::new(r"\w+-\w+-\w+ map:\n(\d+ \d+ \d+\n)+").unwrap();
    let re_name = Regex::new(r"\w+-\w+-\w+").unwrap();
    let re_ranges = Regex::new(r"\n(\d+) (\d+) (\d+)").unwrap();

    let mappings_iter = re_maps.find_iter(input).map(|m| m.as_str());

    for mapping in mappings_iter {
        let name = String::from(re_name.find(mapping).unwrap().as_str());

        let mut ranges: Vec<RangeMap<u32>> = Vec::new();

        for range in re_ranges.captures_iter(mapping) {
            let (_, [dst, src, len]) = range.extract();

            let dst: u32 = dst.parse().unwrap();
            let src: u32 = src.parse().unwrap();
            let len: u32 = len.parse().unwrap();

            let source = Range { start: src, end: src + len };
            let destination = Range { start: dst, end: dst + len };

            ranges.push( RangeMap { source, destination });
        }

        range_mapping.insert(name, ranges);
    }

    range_mapping
}

impl RangeMap<u32> {
    fn contains(&self, num: u32) -> bool {
        self.source.contains(&num)
    }
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
