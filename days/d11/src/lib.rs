use regex::Regex;
use std::fs;
use std::error::Error;

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

    let mut id = 0;
    let mut row = 0;

    for line in input.lines() {
        let n = re.find_iter(line).map(|m| {
            let galaxy = Galaxy::new(id, row, m.start() as i32);
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

mod test {
    use super::*;

    #[test]
    fn parse_1() {
        let input = "./test_1.txt";
        let output = parse_input(input).expect("file not found");

        let galaxies = vec![
            Galaxy::new(0, 0, 3),
            Galaxy::new(1, 1, 7),
            Galaxy::new(2, 2, 0),
            Galaxy::new(3, 5, 6),
            Galaxy::new(4, 6, 1),
            Galaxy::new(5, 7, 9),
            Galaxy::new(6, 10, 7),
            Galaxy::new(7, 11, 0),
            Galaxy::new(8, 11, 4),
        ];
        let expected = Space::new(galaxies);
        assert_eq!(expected, output);
    }
}
