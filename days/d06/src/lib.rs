use regex::Regex;

#[derive(Debug)]
pub struct Race {
    time: i32,
    distance: i32,
}

pub fn parse_input(string: &str) -> Vec<Race> {
    let re_digis = Regex::new(r"\d+").unwrap();
    
    let mut lines = string.lines();

    let times: Vec<i32> = re_digis
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let distances: Vec<i32> = re_digis
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    
    println!("{:?}", times);
    println!("{:?}", distances);

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race { time: *t, distance: *d } )
        .collect()
}
