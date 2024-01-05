mod display;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

enum Location {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    c: char,
    row: usize,
    col: usize,
}

pub fn parse(path: &str) -> (HashMap<String, Node>, HashMap<String, Vec<String>>) {
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

        (nodes.clone(), make_graph(nodes.clone(), max_row, max_col))
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

fn format_path(
    path: &Vec<String>,
) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
    let mut rows: HashMap<String, Vec<String>> = HashMap::new();
    let mut cols: HashMap<String, Vec<String>> = HashMap::new();

    for index in path.iter() {
        let values: Vec<String> = index
            .split("_")
            .map(|s| s.to_string())
            .collect();

        let (row, col) = (&values[0], &values[1]);

        rows.entry(row.into()).or_insert(vec![]).push(index.into());
        cols.entry(col.into()).or_insert(vec![]).push(index.into());

    }

    for value in rows.values_mut() {
        value.sort();
    }

    for value in cols.values_mut() {
        value.sort();
    }

    (rows, cols)
}

pub fn ids2coords(ids: Vec<String>) -> Vec<(i32, i32)> {
    let re = Regex::new(r"(?<row>\d+)_(?<col>\d+)").unwrap();

    let x = ids
        .iter()
        .map(|id| -> i32 {
            let caps = re.captures(id).unwrap();
            caps["col"].to_string().parse().ok().unwrap()
        });

    let mut y: Vec<i32> = ids
        .iter()
        .map(|id| -> i32 {
            let caps = re.captures(id).unwrap();
            caps["row"].to_string().parse().ok().unwrap()
        })
        .collect();

    let y_max = y.iter().max().unwrap();
    y = y.iter().map(|t| y_max - t).collect();

    x.zip(y).collect()
}

pub fn point_in_polygon(point: &(i32, i32), polygon: &Vec<(i32, i32)>) -> bool {
    let (x, y) = point;
    let n = polygon.len();
    let mut inside = false;

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i+1) % n];

        if ((&y1 <= &y) && (y < &y2)) || ((&y2 <= &y) && (y < &y1)) {
            if x < &((x2 - x1) * (y - y1) / (y2 - y1) + x1) {
                inside = !inside;
            }
        }
    }

    inside
}

pub fn count_enclosed(polygon: &Vec<(i32, i32)>) -> i32 {
    let x_min = polygon.iter().map(|(x, _)| x).min().unwrap().clone();
    let x_max = polygon.iter().map(|(x, _)| x).max().unwrap().clone();

    let y_min = polygon.iter().map(|(_, y)| y).min().unwrap().clone();
    let y_max = polygon.iter().map(|(_, y)| y).max().unwrap().clone();

    let mut count = 0;
    for x in x_min..x_max {
        for y in y_min..y_max {
            let point: (i32, i32) = (x, y);

            if polygon.contains(&point) {
                continue;
            } else if point_in_polygon(&point, &polygon) {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug)]
enum PipeState {
    Inside,
    Outside,
}

impl PipeState {
    fn toggle(&mut self) {
        *self = match self {
            PipeState::Inside => PipeState::Outside,
            PipeState::Outside => PipeState::Inside,
        };
    }
}

enum VecType {
    Row,
    Col,
}

fn scan_vec(
    v: Vec<String>,
    mut state: PipeState,
    mut enclosed: Vec<String>,
    graph: &HashMap<String, Node>,
    vec_type: VecType,
) -> Vec<String> {
    println!("v: {:?}", v);
    println!("state: {:?}", state);
    if v.len() == 1 {
        enclosed
    } else {
        enclosed = match state {
            PipeState::Inside => {
                let current: Vec<String> = v[0]
                    .split("_")
                    .map(|s| s.to_string())
                    .collect();

                let next: Vec<String> = v[1]
                    .split("_")
                    .map(|s| s.to_string())
                    .collect();

                let cur_row: u32 = current[0].parse().unwrap();
                let nxt_row: u32 = next[0].parse().unwrap();

                let cur_col: u32 = current[1].parse().unwrap();
                let nxt_col: u32 = next[1].parse().unwrap();

                let mut keys: Vec<String> = match vec_type {
                    VecType::Row => {
                        (cur_col+1..nxt_col)
                            .into_iter()
                            .map(|n| format!("{}_{}", cur_row, n))
                            .collect()
                    },
                    VecType::Col => {
                        (cur_row+1..nxt_row)
                            .into_iter()
                            .map(|n| format!("{}_{}", n, cur_col))
                            .collect()
                    },
                };

                enclosed.append(&mut keys);
                enclosed
            },
            PipeState::Outside => enclosed,
        };

        if let Some(node) = graph.get(&v[1]) {
            match vec_type {
                VecType::Row => match node.c {
                    '|'|'J'|'7' => state.toggle(),
                    _ => {},
                },
                VecType::Col => match node.c {
                    '-'|'L'|'F' => state.toggle(),
                    _ => {},
                },
            }
        }
        println!("enclosed: {:?}", enclosed);

        scan_vec(v[1..].to_vec(), state, enclosed, graph, vec_type)
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(coords: &(i32, i32)) -> Self {
        let x = coords.0;
        let y = coords.1;
        Point { x, y }
    }
}

fn is_point_in_polygon(point: &Point, polygon: &Vec<(i32, i32)>) -> bool {
    let n = polygon.len();
    let mut odd_intersections = false;

    for i in 0..n {
        let p1 = Point::new(&polygon[i]);
        let p2 = Point::new(&polygon[(i+1) % n]);

        if p1.y == p2.y {
            let intersects_y = p1.y == point.y;
            let intersects_x = (p1.x.min(p2.x)..=p1.x.max(p2.x)).contains(&point.x);

            if intersects_y && intersects_x {
                return true
            }
        } else {
            let within_y = (p1.y..p2.y).contains(&point.y) || (p2.y..p1.y).contains(&point.y);
            let within_x = point.x < (p2.x - p1.x) * (point.y - p1.y) / (p2.y - p1.y) + p1.x;

            if within_y && within_x {
                odd_intersections = !odd_intersections;
            }
        }

    }

    odd_intersections
}

fn loop2polygon(coords: &Vec<String>) -> Vec<(i32, i32)> {
    let re_coords = Regex::new(r"(\d+)_(\d+)").unwrap();
    let x: Vec<i32> = coords
        .iter()
        .map(|s| -> i32 {
            let (_, [_, x]) = re_coords.captures(s).unwrap().extract();
            let x = x.to_string().parse::<i32>().ok().unwrap();
            x
        })
        .collect();

    let mut y: Vec<i32> = coords
        .iter()
        .map(|s| -> i32 {
            let (_, [y, _]) = re_coords.captures(s).unwrap().extract();
            let y = y.to_string().parse::<i32>().ok().unwrap();
            y
        })
        .collect();

    let y_max = y.iter().max().unwrap();

    y = y.iter().map(|t| y_max - t).collect();

    x.into_iter().zip(y.into_iter()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_parse() {
        let path = "./case1.txt";
        let (_, result) = parse(path);
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
        let (_, result) = parse(path);

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
        let (_, graph) = parse(path);
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
        let (_, graph) = parse(path);

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
        let (_, graph) = parse(path);

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
