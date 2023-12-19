use std::collections::HashMap;

pub fn navigate(
    instructions: String,
    map: HashMap<String, HashMap<char, String>>
) -> u64 {
    let mut counts: u64 = 0;
    let mut locations: Vec<String> = map
        .keys()
        .into_iter()
        .filter(|m| m.ends_with("A"))
        .map(|m| m.to_owned())
        .collect();
    
    for instruction in instructions.chars().cycle() {
        counts += 1;

        locations = locations.iter().map(|l| { map
                .get(l).unwrap()
                .get(&instruction)
                .unwrap()
                .to_string()
        }).collect();

        if locations.iter().all(|loc| loc.ends_with("Z")) {
            break
        }
    }

    counts
}
