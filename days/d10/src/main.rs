mod display;
use d10::*;

fn main() {
    let (_, graph) = parse("./input.txt");
    println!("Finished parsing");
    let start = "54_15".to_string();
    let loop_path = find_loop(&graph, &start, &"".to_string(), vec![]);
    println!("Part 1:\n{:?}", loop_path.len()/2);
}

