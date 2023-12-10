#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::ops::Range;
use regex::Regex;

#[derive(Debug)]
pub struct RangeMap<T> {
    source: Range<T>,
    destination: Range<T>,
}

pub fn parse_seeds(input: &str) -> Vec<u64> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.lines().next().unwrap())
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect()
}

pub fn parse_mappings(input: &str) -> HashMap<String, Vec<RangeMap<u64>>> {
    let mut range_mapping: HashMap<String, Vec<RangeMap<u64>>> = HashMap::new();

    let re_maps = Regex::new(r"\w+-\w+-\w+ map:\n(\d+ \d+ \d+\n)+").unwrap();
    let re_name = Regex::new(r"\w+-\w+-\w+").unwrap();
    let re_ranges = Regex::new(r"\n(\d+) (\d+) (\d+)").unwrap();

    let mappings_iter = re_maps.find_iter(input).map(|m| m.as_str());

    for mapping in mappings_iter {
        let name = String::from(re_name.find(mapping).unwrap().as_str());

        let mut ranges: Vec<RangeMap<u64>> = Vec::new();

        for range in re_ranges.captures_iter(mapping) {
            let (_, [dst, src, len]) = range.extract();

            let dst: u64 = dst.parse().unwrap();
            let src: u64 = src.parse().unwrap();
            let len: u64 = len.parse().unwrap();

            let source = Range { start: src, end: src + len };
            let destination = Range { start: dst, end: dst + len };

            ranges.push( RangeMap { source, destination });
        }

        range_mapping.insert(name, ranges);
    }

    range_mapping
}
