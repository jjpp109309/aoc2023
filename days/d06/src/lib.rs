use regex::Regex;
use std::ops::{Mul, Sub};

#[derive(Debug)]
pub struct Race {
    pub time: i32,
    pub distance: i32,
}

#[derive(Debug)]
pub struct LongRace {
    pub time: i64,
    pub distance: i64,
}

pub fn parse_input(string: &str) -> Vec<Race> {
    let re_digis = Regex::new(r"\d+").unwrap();
    
    let mut lines = string.lines();

    let times = re_digis
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<i32>().unwrap());

    let distances = re_digis
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<i32>().unwrap());
    
    times
        .zip(distances)
        .map(|(t, d)| Race { time: t, distance: d } )
        .collect()
}

pub fn parse_single_input(string: &str) -> LongRace {
    let re_digis = Regex::new(r"\d+").unwrap();
    
    let mut lines = string.lines();

    let time: i64 = re_digis
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let distance: i64 = re_digis
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    LongRace { time, distance }
}

pub fn total_wins(race: &Race) -> i32 {
    let mut charge_time = 0;

    while total_distance(charge_time, race.time) <= race.distance {
        charge_time += 1;
    }

    let mut wins = 0;
    while total_distance(charge_time, race.time) > race.distance {
        wins += 1;
        charge_time += 1;
    }
        
    wins
}

pub fn total_long_wins(race: &LongRace) -> i64 {
    let mut charge_time = 0;

    while total_distance(charge_time, race.time) <= race.distance {
        charge_time += 1;
    }

    let mut wins = 0;
    while total_distance(charge_time, race.time) > race.distance {
        wins += 1;
        charge_time += 1;
    }
        
    wins
}

fn total_distance<T> (charge_time: T, total_time: T) -> T 
where T: Mul<Output=T> + Sub<Output=T> + Copy {
    (total_time - charge_time) * charge_time
}
