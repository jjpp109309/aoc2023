use std::fs;
use std::collections::HashMap;

enum Location {
    North,
    South,
    East,
    West,
}

pub fn parse(path: &str) -> HashMap<String, Vec<String>> {
    if let Ok(file) = fs::read_to_string(path) {
        let max_rows = file.lines().count();
        file
            .lines()
            .enumerate()
            .map(|(row, line)| from_line(line, row, max_rows))
            .flatten()
            .collect()
    } else {
        panic!("File not found");
    }
}

fn from_line(line: &str, row: usize, max_rows: usize) -> Vec<(String, Vec<String>)> {
    let max_cols = line.chars().count();
    line
        .chars()
        .enumerate()
        .map(|(col, c)| make_node(c, row, col, max_rows, max_cols))
        .flatten()
        .collect()
}

fn make_node(
    c: char,
    row: usize,
    col: usize,
    max_rows: usize,
    max_cols: usize
    ) -> Option<(String, Vec<String>)> {
    
    let locations: Vec<Location> = match c {
        '|' => vec![Location::North, Location::South],
        '-' => vec![Location::West, Location::East],
        'L' => vec![Location::North, Location::East],
        'J' => vec![Location::North, Location::West],
        '7' => vec![Location::South, Location::West],
        'F' => vec![Location::South, Location::East],
        'S' => vec![Location::North, Location::South, Location::East, Location::West],
        '.' => vec![],
        _ => panic!("Unrecognized character"),
    };

    let mut neighbors: Vec<String> = Vec::new();

    if locations.is_empty() {
        None
    } else {
        for location in locations {
            match location {
                Location::North => if row > 0 {
                    neighbors.push(format!("{}_{}", row - 1, col))
                },
                Location::South => if row < max_rows - 1 {
                    neighbors.push(format!("{}_{}", row + 1, col))
                },
                Location::West => if col > 0 {
                    neighbors.push(format!("{}_{}", row, col - 1))
                },
                Location::East => if col < max_cols - 1 {
                    neighbors.push(format!("{}_{}", row, col + 1))
                },
            }
        }
        let id = format!("{}_{}", row, col);

        Some((id, neighbors))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let path = "./case1.txt";
        let result = parse(path);
        println!("{:?}", result);

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();

        expected.insert("1_1".to_string(), vec!["2_1".to_string(), "1_2".to_string()]);
        expected.insert("3_3".to_string(), vec!["2_3".to_string(), "3_2".to_string()]);
        expected.insert("3_1".to_string(), vec!["2_1".to_string(), "3_2".to_string()]);
        expected.insert("2_1".to_string(), vec!["1_1".to_string(), "3_1".to_string()]);
        expected.insert("1_3".to_string(), vec!["2_3".to_string(), "1_2".to_string()]);
        expected.insert("1_2".to_string(), vec!["1_1".to_string(), "1_3".to_string()]);
        expected.insert("3_2".to_string(), vec!["3_1".to_string(), "3_3".to_string()]);
        expected.insert("2_3".to_string(), vec!["1_3".to_string(), "3_3".to_string()]);

        assert_eq!(expected, result);
    }
}
