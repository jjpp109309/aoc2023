use std::fs;
use std::collections::HashMap;

enum Location {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Node {
    c: char,
    row: usize,
    col: usize,
}

pub fn parse(path: &str) -> HashMap<String, Vec<String>> {
    if let Ok(file) = fs::read_to_string(path) {
        let max_rows = file.lines().count();

        let mut nodes: HashMap<String, Node> = file
            .lines()
            .enumerate()
            .map(|(row, line)| from_line(line, row))
            .flatten()
            .collect();

        make_graph(nodes)
    } else {
        panic!("File not found");
    }
}

fn from_line(line: &str, row: usize) -> Vec<(String, Node)> {
    let max_cols = line.chars().count();
    line
        .chars()
        .enumerate()
        .filter(|(_, c)| c!=&'.')
        .map(|(col, c)| (format!("{}_{}", row, col), Node { c, row, col }) )
        .collect()
}

fn make_graph(nodes: HashMap<String, Node>) -> HashMap<String, Vec<String>> {
    
    let locations: HashMap<String, Vec<Location>> = nodes
        .iter()
        .map(|(id, n)| (id.to_string(), get_locations(n.c)))
        .collect();

    nodes
        .iter()
        .map(|(id, node)| get_neighbors(id, node, locations))

    todo!();
    // if locations.is_empty() {
    //     None
    // } else {
    //     for location in locations {
    //         match location {
    //             Location::North => if row > 0 {
    //                 neighbors.push(format!("{}_{}", row - 1, col))
    //             },
    //             Location::South => if row < max_rows - 1 {
    //                 neighbors.push(format!("{}_{}", row + 1, col))
    //             },
    //             Location::West => if col > 0 {
    //                 neighbors.push(format!("{}_{}", row, col - 1))
    //             },
    //             Location::East => if col < max_cols - 1 {
    //                 neighbors.push(format!("{}_{}", row, col + 1))
    //             },
    //         }
    //     }
    //     let id = format!("{}_{}", row, col);
    //
    //     Some((id, neighbors))
    // }
}

fn get_locations(c: char) -> Vec<Location> {
    match c {
        '|' => vec![Location::North, Location::South],
        '-' => vec![Location::West, Location::East],
        'L' => vec![Location::North, Location::East],
        'J' => vec![Location::North, Location::West],
        '7' => vec![Location::South, Location::West],
        'F' => vec![Location::South, Location::East],
        'S' => vec![Location::North, Location::South, Location::East, Location::West],
        '.' => vec![],
        _ => panic!("Unrecognized character"),
    }
}

fn get_neighbors(
    id: String,
    node: Node,
    locations: HashMap<String, Vec<Location>>
) -> (String, Vec<String>) {
    todo!()
}

fn find_loop(
    graph: &HashMap<String, Vec<String>>,
    current: &String,
    parent: &String,
    mut visited: Vec<String>
) -> Vec<String> {
    visited.push(current.to_string());

    for neighbor in graph.get(current).unwrap() {

        if neighbor != parent {
            if visited.contains(neighbor) {
                return vec![current.to_string(), neighbor.to_string()];
            }

            let mut loop_path = find_loop(graph, neighbor, current, visited.clone());

            if !loop_path.is_empty() {
                if loop_path.contains(current) {
                    return loop_path
                } else {
                    let mut path = vec![current.to_string()];
                    path.append(&mut loop_path);

                    return path
                }
            }
        }
    }
    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_parse() {
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

    #[test]
    fn p1_parse_case4() {
        let path = "./case4.txt";
        let result = parse(path);

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


    #[test]
    fn p1_solve() {
        let path = "./case1.txt";
        let graph = parse(path);
        let mut loop_path = find_loop(&graph, &"1_1".to_string(), &"".to_string(), vec![]);
        loop_path.reverse();

        let expected = vec![
            "1_1".to_string(),
            "1_2".to_string(),
            "1_3".to_string(),
            "2_3".to_string(),
            "3_3".to_string(),
            "3_2".to_string(),
            "3_1".to_string(),
            "2_1".to_string()
        ];

        println!("loop {:?}", loop_path);
        assert_eq!(loop_path, expected);
    }
    
    #[test]
    fn p1_solve2() {
        let path = "./case2.txt";
        let graph = parse(path);

        println!("graph\n\n{:?}", graph);

        let mut loop_path = find_loop(&graph, &"2_0".to_string(), &"".to_string(), vec![]);
        loop_path.reverse();

        println!("loop {:?}", loop_path);

        let expected = vec![
            "2_0",
            "2_1",
            "1_1",
            "1_2",
            "0_2",
            "0_3",
            "1_3",
            "2_3",
            "2_4",
            "3_4",
            "3_3",
            "3_2",
            "3_1",
            "4_1",
            "4_0",
            "3_0",
        ];

        assert_eq!(expected, loop_path);
    }

    #[test]
    fn solve_case3() {
        let path = "./case3.txt";
        let graph = parse(path);

        println!("graph\n\n{:?}", graph);

        let start = "3_0".to_string();
        let mut loop_path = find_loop(&graph, &start, &"".to_string(), vec![]);
        if loop_path[0] != start {
            loop_path.reverse()
        }

        let expected = vec![
            "3_0",
            "3_1",
            "2_1",
            "2_2",
            "1_2",
            "1_3",
            "2_3",
            "3_3",
            "3_4",
            "4_4",
            "4_3",
            "4_2",
            "4_1",
            "5_1",
            "5_0",
            "4_0",
        ];

        assert_eq!(expected, loop_path);
    }
}
