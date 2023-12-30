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
        let max_row = file.lines().count() - 1;
        let max_col = file
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .len() - 1;

        let nodes: HashMap<String, Node> = file
            .lines()
            .enumerate()
            .map(|(row, line)| from_line(line, row))
            .flatten()
            .collect();

        make_graph(nodes, max_row, max_col)
    } else {
        panic!("File not found");
    }
}

fn from_line(line: &str, row: usize) -> Vec<(String, Node)> {
    line
        .chars()
        .enumerate()
        .filter(|(_, c)| c!=&'.')
        .map(|(col, c)| (format!("{}_{}", row, col), Node { c, row, col }) )
        .collect()
}

fn make_graph(
    nodes: HashMap<String, Node>,
    max_rows: usize,
    max_cols: usize
) -> HashMap<String, Vec<String>> {
    
    let neighbor_locations: HashMap<String, Vec<Location>> = nodes
        .iter()
        .map(|(id, n)| (id.to_string(), get_locations(n.c)))
        .collect();

    nodes
        .iter()
        .map(|(id, node)| get_neighbors(id, node, &neighbor_locations, max_rows, max_cols))
        .collect()
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
    id: &String,
    node: &Node,
    locations: &HashMap<String, Vec<Location>>,
    max_row: usize,
    max_col: usize,
) -> (String, Vec<String>) {
    let mut neighbors = Vec::new();

    for location in locations.get(id).unwrap() {
        let mut n_id = None;
        let (n_id, is_neighbor): (Option<String>, fn(&Location)->bool) =
            match location {
                Location::North => {
                    let is_neighbor = |l: &Location| match l {
                        Location::South => true,
                        _ => false
                    };
                    if node.row > 0 {
                        n_id = Some(format!("{}_{}", node.row-1, node.col));
                    }

                    (n_id, is_neighbor)
                },
                Location::South => {
                    let is_neighbor = |l: &Location| match l {
                        Location::North => true,
                        _ => false
                    };
                    if node.row < max_row {
                        n_id = Some(format!("{}_{}", node.row + 1, node.col));
                    }

                    (n_id, is_neighbor)
                },
                Location::East => {
                    let is_neighbor = |l: &Location| match l {
                        Location::West => true,
                        _ => false
                    };
                    if node.col < max_col {
                        n_id = Some(format!("{}_{}", node.row, node.col + 1));
                    }

                    (n_id, is_neighbor)
                },
                Location::West => {
                    let is_neighbor = |l: &Location| match l {
                        Location::East => true,
                        _ => false
                    };
                    if node.col > 0 {
                        n_id = Some(format!("{}_{}", node.row, node.col - 1));
                    }

                    (n_id, is_neighbor)
                },
        };

        if let Some(s) = neighbor(n_id, locations, is_neighbor) {
            neighbors.push(s);
        }
    }

    (id.to_string(), neighbors)
}

fn neighbor<F>(
    n_id: Option<String>,
    locations: &HashMap<String, Vec<Location>>,
    is_neighbor: F
) -> Option<String>
where
    F: Fn(&Location) -> bool,
{
    if let Some(n_id) = &n_id {
        if let Some(locs) = locations.get(n_id) {
            return locs.iter().any(is_neighbor).then(|| n_id).cloned()
        }
    }
    None
}

pub fn find_loop(
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

fn format_path(path: Vec<String>) -> (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) {
    let mut rows: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut cols: HashMap<u32, Vec<u32>> = HashMap::new();

    for index in path.iter() {
        let values: Vec<u32> = index
            .split("_")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let (row, col) = (values[0], values[1]);

        rows.entry(row).or_insert(vec![]).push(col);
        cols.entry(col).or_insert(vec![]).push(row);

    }

    for value in rows.values_mut() {
        value.sort();
    }

    for value in cols.values_mut() {
        value.sort();
    }

    (rows, cols)
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
        expected.insert("0_1".to_string(), vec![]);
        expected.insert("4_3".to_string(), vec![]);

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

    #[test]
    fn path_dicts() {
        let input = vec!["1_3".to_string(), "1_2".to_string()];
        let (rows, cols) = format_path(input);

        let mut e_rows: HashMap<u32, Vec<u32>> = HashMap::new();
        e_rows.insert(1, vec![2, 3]);

        let mut e_cols: HashMap<u32, Vec<u32>> = HashMap::new();
        e_cols.insert(2, vec![1]);
        e_cols.insert(3, vec![1]);

        assert_eq!(rows, e_rows);
        assert_eq!(cols, e_cols);
    }

    #[test]
    fn enclosed_case1() {
        let graph = parse("./enclosed_1.txt");
        let start = "1_1".to_string();
        let mut loop_path = find_loop(&graph, &start, &"".to_string(), vec![]);
        loop_path.sort();
        println!("loop path\n\n{:?}", loop_path);
        
        panic!()
    }
}
