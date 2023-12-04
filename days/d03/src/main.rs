use d03::*;

fn main() {
    let path = "test_input.txt";
    let input = parse_input(path);
    let part_numbers = get_part_numbers(&input);

    println!("Test: {}", part_numbers.iter().sum::<u32>());


    let path = "input.txt";
    let input = parse_input(path);
    let part_numbers = get_part_numbers(&input);

    println!("Part 1: {}", part_numbers.iter().sum::<u32>());
}
