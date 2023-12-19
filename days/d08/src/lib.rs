use std::fs;
use regex::Regex;
use std::collections::HashMap;


pub fn parse_input(path: &str) -> (String, HashMap<String, HashMap<char, String>>) {

    let re_nodes = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut nodes: HashMap<String, HashMap<char, String>> = HashMap::new();

    let instructions: String;
    if let Ok(s) = fs::read_to_string(path) {
        let mut lines = s.lines();

        instructions = lines.next().unwrap().to_string();

        lines.next();

        for line in lines {
            for (_, [key, left, right]) in re_nodes.captures_iter(line).map(|c| c.extract()) {

                let mut mapping: HashMap<char, String> = HashMap::new();
                mapping.insert('L', left.to_string());
                mapping.insert('R', right.to_string());

                nodes.insert(key.to_string(), mapping);
            }
        }
        
    }
    else {
        panic!("File not found :(")
    }

    (instructions, nodes)
}
