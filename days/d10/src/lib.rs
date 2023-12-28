use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Node {
    id: String,
    row: usize,
    col: usize,
    neighbors: Vec<String>,
    discovered: bool,
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
            let id = format!("{}_{}", row, col);
            let discovered = false;

            Some(Node { id, row, col, neighbors, discovered })
        }
    }

    fn set_discovered(&mut self) {
        self.discovered = true
    }

    fn get_neighbors(&self) -> Vec<String> {
        self.neighbors.clone()
    }

    fn is_discovered(&self) -> bool {
        self.discovered.clone()
    }
}

pub fn parse(path: &str) -> HashMap<String, Node> {

    let mut graph: HashMap<String, Node> = HashMap::new();

    if let Ok(file) = fs::read_to_string(path) {
        println!("file\n{}", file);
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

pub fn dfs(graph: &mut HashMap<String, Node>, id: &String, cycle: &mut Vec<String>) -> &mut Vec<&String> {
    graph.get_mut(id).unwrap().set_discovered();

    let unexplored: Vec<String> = graph
        .get(id)
        .unwrap()
        .get_neighbors()
        .into_iter()
        .filter(|x| !cycle.contains(&x))
        .collect();

    if unexplored.is_empty() {
        todo!()
    } else {
        let mut new_discovered: Vec<String>;
        for n_id in &unexplored {
            new_discovered = vec![n_id.to_string()];
            let is_discovered = graph.get(n_id).unwrap().is_discovered();
            if !is_discovered {
                new_discovered = dfs(graph, n_id, &mut new_discovered);
            }
        }

        cycle.append(&mut new_discovered);
        return cycle
    }
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let path = "./case1.txt";
        let result = parse(path);

        let mut expected: HashMap<String, Node> = HashMap::new();

        expected.insert("1_1".to_string(), Node { id: "1_1".to_string(), row: 1, col: 1, neighbors: vec!["2_1".to_string(), "1_2".to_string()], discovered: false});
        expected.insert("3_3".to_string(), Node { id: "3_3".to_string(), row: 3, col: 3, neighbors: vec!["2_3".to_string(), "3_2".to_string()], discovered: false});
        expected.insert("3_1".to_string(), Node { id: "3_1".to_string(), row: 3, col: 1, neighbors: vec!["2_1".to_string(), "3_2".to_string()], discovered: false});
        expected.insert("2_1".to_string(), Node { id: "2_1".to_string(), row: 2, col: 1, neighbors: vec!["1_1".to_string(), "3_1".to_string()], discovered: false});
        expected.insert("1_3".to_string(), Node { id: "1_3".to_string(), row: 1, col: 3, neighbors: vec!["2_3".to_string(), "1_2".to_string()], discovered: false});
        expected.insert("1_2".to_string(), Node { id: "1_2".to_string(), row: 1, col: 2, neighbors: vec!["1_1".to_string(), "1_3".to_string()], discovered: false});
        expected.insert("3_2".to_string(), Node { id: "3_2".to_string(), row: 3, col: 2, neighbors: vec!["3_1".to_string(), "3_3".to_string()], discovered: false});
        expected.insert("2_3".to_string(), Node { id: "2_3".to_string(), row: 2, col: 3, neighbors: vec!["1_3".to_string(), "3_3".to_string()], discovered: false});

        for (key, val) in expected.iter() {
            let res_val = result.get(key).unwrap();
            assert_eq!(val.row, res_val.row);
            assert_eq!(val.col, res_val.col);
            assert_eq!(val.neighbors, res_val.neighbors);
        }
    }

    #[test]
    fn test_cycle_case_1() {
        let path = "./case1.txt";
        let mut result = parse(path);
        println!("before\n{:?}", result);

        dfs(&mut result, &"1_1".to_string());

        println!("after\n{:?}", result);
        panic!();
    }
}
