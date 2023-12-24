use d09::*;

fn main() {
    let input = parse("./input.txt");
    let total: i64 = input.iter().map(predict).sum();

    println!("Part 1: {}", total);
}
