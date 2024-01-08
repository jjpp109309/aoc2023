use std::fs;
use std::error::Error;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Onsen {
    springs: Vec<SpringRow>
}

impl Onsen {
    fn new(springs: Vec<SpringRow>) -> Onsen {
        Onsen { springs }
    }
}

#[derive(Debug, PartialEq)]
struct SpringRow {
    conditions: Vec<Condition>,
    groups: Vec<usize>,
}

impl SpringRow {
    fn parse(row: &str) -> SpringRow {
        println!("row {:?}", row);
        let mut parts = row.split(" ");

        let conditions: Vec<Condition> = parts
            .next()
            .unwrap()
            .chars()
            .map(Condition::parse_char)
            .collect();

        let groups = parts
            .next()
            .unwrap()
            .split(",")
            .map(|d| d.parse::<usize>().ok().unwrap())
            .collect();
        println!("conditions {:?}", conditions);
        println!("groups {:?}", groups);

        SpringRow { conditions, groups }
    }
}

#[derive(Debug, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn parse_char(record: char) -> Condition {
        match record {
            '#' => Condition::Damaged,
            '.' => Condition::Operational,
            '?' => Condition::Unknown,
            _ => panic!("Unrecognized condition"),
        }
    }
}

fn parse_input(path: &str) -> Result<Onsen, Box<dyn Error>>{
    let input = fs::read_to_string(path)?;
    
    let springs: Vec<SpringRow> = input
        .lines()
        .map(SpringRow::parse)
        .collect();

    Ok(Onsen::new(springs))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn parse_input_1() {
        let input = "./test_files/case_1.txt";
        let output = parse_input(input).unwrap();

        let mut spring_rows: Vec<SpringRow> = vec![];
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
        ];
        let groups: Vec<usize> = vec![1, 1, 3];
        spring_rows.push(SpringRow { conditions, groups });

        let conditions = vec![
            Condition::Operational,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Operational,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Operational,
            Condition::Operational,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Operational,
        ];
        let groups: Vec<usize> = vec![1, 1, 3];
        spring_rows.push(SpringRow { conditions, groups });
        let expected = Onsen::new(spring_rows);

        assert_eq!(expected, output);
    }
}
