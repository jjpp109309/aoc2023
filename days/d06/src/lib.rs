use regex::Regex;

#[derive(Debug)]
pub struct Race {
    pub time: i32,
    pub distance: i32,
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

pub fn total_wins(race: &Race) -> i32 {
    let mut charge_time = 0;

    while total_distance(&charge_time, &race.time) <= race.distance {
        charge_time += 1;
    }

    let mut wins = 0;
    while total_distance(&charge_time, &race.time) > race.distance {
        wins += 1;
        charge_time += 1;
    }
        
    wins
}

fn total_distance(charge_time: &i32, total_time: &i32) -> i32 {
    (total_time - charge_time) * charge_time
}
