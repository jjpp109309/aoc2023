#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::cmp;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct RangeMap {
    pub start: i64,
    pub end: i64,
    pub delta: Option<i64>,
}

impl PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.start < other.start {
            Some(cmp::Ordering::Less)
        } else if self.start > other.start {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Equal)
        }
    }
}

impl PartialEq for RangeMap {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Ord for RangeMap {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.start < other.start {
            cmp::Ordering::Less
        } else if self.start > other.start {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    }
}

impl Eq for RangeMap {}

impl RangeMap {
    fn intersect(&self, other: &Self) -> Intersection {
        let start = cmp::max(self.start, other.start);
        let end = cmp::min(self.end, other.end);

        match start < end {
            true => {
                match other.end <= self.end {
                    true => Intersection::Proper(other.to_owned()),
                    false => Intersection::Improper(
                        RangeMap { start, end: self.end, delta: None },
                        RangeMap { start: self.end, end: other.end, delta: None }
                    ),
                }
            },
            false => Intersection::Null
        }

    }
}

enum Intersection {
    Proper(RangeMap),
    Improper(RangeMap, RangeMap),
    Null,
}

pub fn parse_seeds(input: &str) -> Vec<i64> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.lines().next().unwrap())
        .map(|x| x.as_str().parse::<i64>().unwrap())
        .collect()
}

pub fn parse_mappings(input: &str) -> HashMap<String, Vec<RangeMap>> {
    let mut range_mapping: HashMap<String, Vec<RangeMap>> = HashMap::new();

    let re_maps = Regex::new(r"\w+-\w+-\w+ map:\n(\d+ \d+ \d+\n)+").unwrap();
    let re_name = Regex::new(r"\w+-\w+-\w+").unwrap();
    let re_ranges = Regex::new(r"\n(\d+) (\d+) (\d+)").unwrap();

    let mappings_iter = re_maps.find_iter(input).map(|m| m.as_str());

    for mapping in mappings_iter {
        let name = String::from(re_name.find(mapping).unwrap().as_str());

        let mut ranges: Vec<RangeMap> = Vec::new();

        for range in re_ranges.captures_iter(mapping) {
            let (_, [dst, src, len]) = range.extract();

            let dst: i64 = dst.parse().unwrap();
            let src: i64 = src.parse().unwrap();
            let len: i64 = len.parse().unwrap();

            let start = src;
            let end = src + len;
            let delta = Some(dst - src);

            ranges.push( RangeMap { start, end, delta });
        }
        ranges.sort();
        ranges = fill_range(&ranges);

        range_mapping.insert(name, ranges);
    }

    range_mapping
}

fn fill_range(ranges: &Vec<RangeMap>) -> Vec<RangeMap> {
    let mut index = 0;
    let mut filled_ranges: Vec<RangeMap> = Vec::new();
   
    for range in ranges.iter() {
        if range.start > index {
            let filler = RangeMap { start: index, end: range.start, delta: Some(0) };
            filled_ranges.push(filler);
        }

        filled_ranges.push(range.to_owned());
        index = range.end;
    }

    let filler = RangeMap { start: index, end: i64::MAX, delta: Some(0)};
    filled_ranges.push(filler);

    filled_ranges
}

pub fn map_ranges(inputs: &Vec<RangeMap>, mappings: &Vec<RangeMap>) -> Vec<RangeMap>{   
    let mut mapped_ranges: Vec<RangeMap> = Vec::new();

    let input_iter = inputs.iter();

    for input in inputs {
        let mut input_ = input.to_owned();

        for map in mappings {
            let disj: Option<RangeMap> = match map.intersect(&input_) {
                Intersection::Proper(x) => {
                    let intersection = RangeMap {
                        start: x.start + map.delta.unwrap(),
                        end: x.end + map.delta.unwrap(),
                        delta: None,
                    };

                    mapped_ranges.push(intersection);
                    break;
                },
                Intersection::Improper(x, disj) => {
                    let intersection = RangeMap {
                        start: x.start + map.delta.unwrap(),
                        end: x.end + map.delta.unwrap(),
                        delta: None,
                    };

                    mapped_ranges.push(intersection);

                    Some(disj.clone())
                },
                Intersection::Null => None,
            };

            if let Some(i) = disj {
                input_ = i;
            }
        }
    }

    mapped_ranges
}
