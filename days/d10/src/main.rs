mod display;
use std::fs;
use d10::*;

fn main() {
    let (_, graph) = parse("./input.txt");
    println!("Finished parsing");
    let start = "54_15".to_string();
    let loop_path = find_loop(&graph, &start, &"".to_string(), vec![]);
    println!("Part 1:\n{:?}", loop_path.len()/2);

    let loop_string = format!("{:?}", loop_path);
    let loop_file = "./loop.txt";
    
    // fs::write(loop_file, loop_string).expect("path not found");
    let polygon = ids2coords(loop_path);
    let points = count_enclosed(&polygon);

    println!("total points {:?}", points);
}

