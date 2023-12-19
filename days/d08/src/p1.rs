use std::collections::HashMap;

pub fn navigate(
    instructions: String,
    map: HashMap<String, HashMap<char, String>>
) -> u32 {
    let mut counts = 0;
    let mut location = &String::from("AAA");
    
    for instruction in instructions.chars().cycle() {
        counts += 1;
        location = map.get(location).unwrap().get(&instruction).unwrap();

        if location == "ZZZ" {
            break
        }
    }

    counts
}
