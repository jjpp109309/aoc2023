use d09::*;

fn main() {
    // part 1
    let mut input = parse("./input.txt");
    let total: i64 = input.iter().map(predict).sum();

    println!("Part 1: {}", total);

    // part 2
    let total: i64 = input
        .iter_mut()
        .map(|line|{
            line.reverse();
            predict(line)
        }).sum();

    println!("Part 2: {}", total);
}
