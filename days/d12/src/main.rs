use d12::*;

fn main() {
        let path = "./input.txt";
        let onsen = parse_input(path).unwrap();
        let sum_counts: usize = onsen.springs
            .iter()
            .map(|row| count_arrangements(row, 0, 0, &row.groups.clone()))
            .sum();

        println!("Part 1: {}", sum_counts);
}
