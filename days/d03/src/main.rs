use d03::*;

fn main() {
    let path = "test_input.txt";
    let input = parse_input(path);
    let part_numbers = get_part_numbers(&input);
    let gear_numbers = get_gears(&input);

    println!("Input:\n{}", input);
    println!("Test: {}", part_numbers.iter().sum::<u32>());
    println!("Gear Numbers:\n{:?}", gear_numbers);

    let path = "input.txt";
    let input = parse_input(path);
    let part_numbers = get_part_numbers(&input);

    println!("Part 1: {}", part_numbers.iter().sum::<u32>());
}
