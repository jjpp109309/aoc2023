use d04::{
    parse_input,
    score,
    multiply_cards,
};

fn main() {
    let path = "./test.txt";
    let cards = parse_input(path);
    let total = score(&cards);
    let mut cards = parse_input(path);
    let total2 = multiply_cards(&mut cards);
    println!("test: {}", total);
    println!("test2: {}", total2);

    let path = "./input.txt";
    let cards = parse_input(path);
    let total = score(&cards);
    println!("Part 1: {}", total);

    let mut cards = parse_input(path);
    let total2 = multiply_cards(&mut cards);
    println!("Part2: {}", total2);
}
