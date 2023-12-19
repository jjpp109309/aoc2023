use d08::parse_input;
mod p1;
mod p2;

fn main() {
    // test
    let path = "./test.txt";
    let (instructions, map) = parse_input(path);

    let total = p1::navigate(instructions, map);
    println!("Test: {:?}", total);

    // part 1
    let path = "./input.txt";
    let (instructions, map) = parse_input(path);

    let total = p1::navigate(instructions, map);
    println!("Part 1: {:?}", total);

    // test 2
    let path = "./test2.txt";
    let (instructions, map) = parse_input(path);

    let total = p2::navigate(instructions, map);
    println!("Test 2: {:?}", total);

    // part 2
    let path = "./input.txt";
    let (instructions, map) = parse_input(path);

    let total = p2::navigate(instructions, map);
    println!("Part 2: {:?}", total);

}
