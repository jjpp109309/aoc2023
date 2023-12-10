#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
pub struct RangeMap<T> {
    start: T,
    end: T,
    delta: T,
}

pub fn parse_seeds(input: &str) -> Vec<i64> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.lines().next().unwrap())
        .map(|x| x.as_str().parse::<i64>().unwrap())
        .collect()
}

pub fn parse_mappings(input: &str) -> HashMap<String, Vec<RangeMap<i64>>> {
    let mut range_mapping: HashMap<String, Vec<RangeMap<i64>>> = HashMap::new();

    let re_maps = Regex::new(r"\w+-\w+-\w+ map:\n(\d+ \d+ \d+\n)+").unwrap();
    let re_name = Regex::new(r"\w+-\w+-\w+").unwrap();
    let re_ranges = Regex::new(r"\n(\d+) (\d+) (\d+)").unwrap();

    let mappings_iter = re_maps.find_iter(input).map(|m| m.as_str());

    for mapping in mappings_iter {
        let name = String::from(re_name.find(mapping).unwrap().as_str());

        let mut ranges: Vec<RangeMap<i64>> = Vec::new();

        for range in re_ranges.captures_iter(mapping) {
            let (_, [dst, src, len]) = range.extract();

            let dst: i64 = dst.parse().unwrap();
            let src: i64 = src.parse().unwrap();
            let len: i64 = len.parse().unwrap();

            let start = src;
            let end = src + len;
            let delta = dst - src;

            ranges.push( RangeMap { start, end, delta });
        }

        range_mapping.insert(name, ranges);
    }

    range_mapping
}
