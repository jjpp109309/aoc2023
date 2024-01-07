use regex::Regex;
use std::fs;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Galaxy {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}

impl Galaxy {
    fn new(id: u32, x: i32, y: i32) -> Galaxy {
        Galaxy { id, x, y }
    }

    fn distance(&self, other: &Galaxy) -> i32 {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();

        x_dist + y_dist
    }
}

#[derive(Debug, PartialEq)]
pub struct Space {
    pub galaxies: Vec<Galaxy>,
}

impl Space {
    fn new(v: Vec<Galaxy>) -> Space {
        Space { galaxies: v }
    }
    
    fn add_galaxy(&mut self, g: Galaxy) {
        self.galaxies.push(g)
    }
}

pub fn parse_input(input: &str) -> Result<Space, Box<dyn Error>> {
    let re = Regex::new(r"#").unwrap();

    let input = fs::read_to_string(input)?;
    let mut space = Space::new(vec![]);
    
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let col_deltas = get_column_deltas(&matrix, &'.');

    let mut id = 0;
    let mut row = 0;

    for line in input.lines() {
        let n = re.find_iter(line).map(|m| {
            let mut col = m.start() as i32;
            col += col_deltas
                .iter()
                .filter(|d| &col > d)
                .count() as i32;

            let galaxy = Galaxy::new(id, row, col);
            space.add_galaxy(galaxy);
            
            id += 1;
        }).count();

        if n == 0 {
            row += 1;
        }
        row += 1;
    }

    Ok(space)
}

fn get_column_deltas(matrix: &Vec<Vec<char>>, empty: &char) -> Vec<i32> {
    let mut deltas = Vec::new();

    for col in 0..matrix[0].len() {
        let column: Vec<char> = matrix.into_iter().map(|r| r[col]).collect();
        if column.iter().all(|c| c == empty ) {
            deltas.push(col as i32);
        } 
    }

    deltas
}

fn find_smallest_distance(space: Space) -> i32 {
    let mut min_distance = i32::MAX;
    let n_galaxies = space.galaxies.len() as i32;
    
    for i in 0..(n_galaxies-1) {
        for j in (i+1)..n_galaxies {
            let distance = space.galaxies[i as usize]
                .distance(&space.galaxies[j as usize]);

            if distance < min_distance {
                min_distance = distance
            }
        }
    }

    min_distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_1() {
        let input = "./test_1.txt";
        let output = parse_input(input).expect("file not found");

        let galaxies = vec![
            Galaxy::new(0, 0, 4),
            Galaxy::new(1, 1, 9),
            Galaxy::new(2, 2, 0),
            Galaxy::new(3, 5, 8),
            Galaxy::new(4, 6, 1),
            Galaxy::new(5, 7, 12),
            Galaxy::new(6, 10, 9),
            Galaxy::new(7, 11, 0),
            Galaxy::new(8, 11, 5),
        ];
        let expected = Space::new(galaxies);
        assert_eq!(expected, output);
    }

    #[test]
    fn distance_1() {
        let g1 = Galaxy::new(4, 6, 1);
        let g2 = Galaxy::new(8, 11, 5);

        let expected = 9;
        let output = g1.distance(&g2);

        assert_eq!(expected, output);
    }
}
