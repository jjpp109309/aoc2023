use std::collections::HashMap;

pub fn navigate(
    instructions: String,
    map: HashMap<String, HashMap<char, String>>
) -> u32 {
    let mut counts = 0;
    let mut locations: Vec<String> = map
        .keys()
        .into_iter()
        .filter(|m| m.ends_with("A"))
        .map(|m| m.to_owned())
        .collect();
    
    for instruction in instructions.chars().cycle() {
        counts += 1;
        for i in 0..locations.len() {
            locations[i] = map
                .get(&locations[i]).unwrap()
                .get(&instruction)
                .unwrap()
                .to_string();
        }
    
        if locations.iter().all(|loc| loc.ends_with("Z")) {
            break
        }
    }

    counts
}
