use d03::*;

fn main() {
    let path = "test_input.txt";
    let input = parse_input(path);
    let part_numbers = get_part_numbers(&input);
    let mut gear_numbers = get_gears(&input);
    let gears = get_true_gears(&mut gear_numbers);

    println!("Test: {}", part_numbers.iter().sum::<u32>());
    println!("Test p2: {}", gears.iter().sum::<u32>());

    let path = "input.txt";
    let input = parse_input(path);
    let part_numbers = get_part_numbers(&input);
    let mut gear_numbers = get_gears(&input);
    let gears = get_true_gears(&mut gear_numbers);

    println!("Part 1: {}", part_numbers.iter().sum::<u32>());
    println!("Part 2: {}", gears.iter().sum::<u32>());
}
