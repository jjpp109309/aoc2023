use std::collections::HashMap;

pub fn navigate(
    instructions: String,
    map: HashMap<String, HashMap<char, String>>
) -> u64 {
    let locations: Vec<String> = map
        .keys()
        .into_iter()
        .filter(|m| m.ends_with("A"))
        .map(|m| m.to_owned())
        .collect();

    let mut instruction: char;
    let mut steps: Vec<u64> = Vec::new();
    let mut count: u64;
    for location in locations {
        count = 0;
        let mut instructions_iter = instructions.chars().cycle();
        
        let mut new_location = location.clone();
        while !new_location.ends_with("Z") {
            instruction = instructions_iter.next().unwrap();
            
            new_location = map
                .get(&new_location).unwrap()
                .get(&instruction).unwrap()
                .to_string();

            count += 1;
        }
        steps.push(count) 
    }
    println!("{:?}", steps);
    
    steps.into_iter().reduce(lcm).unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}
