use regex::Regex;

#[derive(Debug)]
pub struct Race {
    time: i32,
    distance: i32,
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
