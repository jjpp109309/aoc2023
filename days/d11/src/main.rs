use d11::*;

fn main() {
    let input = "./p1.txt";
    let output = parse_input(input, 2).expect("file not found");
    let total = sum_shortest_paths(&output);
    println!("Part 1: {}", total);
}
