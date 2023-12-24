use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    row: usize,
    col: usize,
    visited: bool,
    neighbors: Vec<String>,
}

#[derive(Debug)]
pub enum Location {
    North,
    South,
    East,
    West,
}

impl Node {
    fn from_line(line: &str, row: usize, max_rows: usize) -> Vec<Node> {
        let max_cols = line.chars().count();
        line
            .chars()
            .enumerate()
            .map(|(col, c)| Self::make_node(c, row, col, max_rows, max_cols))
            .flatten()
            .collect()
    }

    fn make_node(c: char, row: usize, col: usize, max_rows: usize, max_cols: usize) -> Option<Node> {
        
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
                    Location::North => if row > 0 {neighbors.push(format!("{}_{}", row - 1, col))},
                    Location::South => if row < max_rows - 1 {neighbors.push(format!("{}_{}", row + 1, col))},
                    Location::West => if col > 0 {neighbors.push(format!("{}_{}", row, col - 1))},
                    Location::East => if col < max_cols - 1 {neighbors.push(format!("{}_{}", row, col + 1))},
                }
            }
            Some(Node { row, col, visited: false, neighbors })
        }
    }

}

pub fn parse(path: &str) -> HashMap<String, Node> {

    let mut graph: HashMap<String, Node> = HashMap::new();

    if let Ok(file) = fs::read_to_string(path) {
        let max_rows = file.lines().count();
        let nodes: Vec<Node> = file
            .lines()
            .enumerate()
            .map(|(row, line)| Node::from_line(line, row, max_rows))
            .flatten()
            .collect();

        for node in nodes {
            graph.insert(format!("{}_{}", node.row, node.col), node);
        }
    } else {
        panic!("File not found");
    }

    graph
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let path = "./case1.txt";
        let result = parse(path);

        let mut expected: HashMap<String, Node> = HashMap::new();

        expected.insert("1_1".to_string(), Node { row: 1, col: 1, visited: false, neighbors: vec!["2_1".to_string(), "1_2".to_string()] });
        expected.insert("3_3".to_string(), Node { row: 3, col: 3, visited: false, neighbors: vec!["2_3".to_string(), "3_2".to_string()] });
        expected.insert("3_1".to_string(), Node { row: 3, col: 1, visited: false, neighbors: vec!["2_1".to_string(), "3_2".to_string()] });
        expected.insert("2_1".to_string(), Node { row: 2, col: 1, visited: false, neighbors: vec!["1_1".to_string(), "3_1".to_string()] });
        expected.insert("1_3".to_string(), Node { row: 1, col: 3, visited: false, neighbors: vec!["2_3".to_string(), "1_2".to_string()] });
        expected.insert("1_2".to_string(), Node { row: 1, col: 2, visited: false, neighbors: vec!["1_1".to_string(), "1_3".to_string()] });
        expected.insert("3_2".to_string(), Node { row: 3, col: 2, visited: false, neighbors: vec!["3_1".to_string(), "3_3".to_string()] });
        expected.insert("2_3".to_string(), Node { row: 2, col: 3, visited: false, neighbors: vec!["1_3".to_string(), "3_3".to_string()] });

        for (key, val) in expected.iter() {
            let res_val = result.get(key).unwrap();
            assert_eq!(val.row, res_val.row);
            assert_eq!(val.col, res_val.col);
            assert_eq!(val.visited, res_val.visited);
            assert_eq!(val.neighbors, res_val.neighbors);
        }
    }
}
