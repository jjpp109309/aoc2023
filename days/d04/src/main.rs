use d04::{
    parse_input,
    score
};

fn main() {
    let path = "./test.txt";
    let cards = parse_input(path);
    let total = score(&cards);
    println!("test: {}", total);

    let path = "./input.txt";
    let cards = parse_input(path);
    let total = score(&cards);
    println!("Part 1: {}", total);

}
