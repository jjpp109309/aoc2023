use std::fs;
use std::error::Error;
use std::fmt;
use regex::Regex;

    #[derive(Debug, PartialEq)]
pub struct Onsen {
    pub springs: Vec<SpringRow>
}

impl Onsen {
    fn new(springs: Vec<SpringRow>) -> Onsen {
        Onsen { springs }
    }
}

#[derive(PartialEq)]
pub struct SpringRow {
    pub conditions: Vec<Condition>,
    pub groups: Vec<usize>,
}

impl fmt::Debug for SpringRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let springs: String = self.conditions
            .iter()
            .map(|c| {
                match c {
                    Condition::Operational => '.',
                    Condition::Damaged => '#',
                    Condition::Unknown => '?',
                }
            })
        .collect();

        write!(f, "springs: {}", springs)
    }
}

impl fmt::Display for SpringRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let springs: String = self.conditions
            .iter()
            .map(|c| {
                match c {
                    Condition::Operational => '.',
                    Condition::Damaged => '#',
                    Condition::Unknown => '?',
                }
            })
        .collect();

        write!(f, "{}", springs)
    }
}

impl SpringRow {
    fn new(conditions: Vec<Condition>, groups: Vec<usize>) -> SpringRow {
        SpringRow { conditions, groups }
    }

    fn parse(row: &str) -> SpringRow {
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

        SpringRow { conditions, groups }
    }

    fn parse2(row: &str) -> SpringRow {
        let mut parts = row.split(" ");

        let multiplied_row = vec![parts.next().unwrap(); 5].join("");

        let conditions: Vec<Condition> = multiplied_row
            .chars()
            .map(Condition::parse_char)
            .collect();

        let multiplied_row = vec![parts.next().unwrap(); 5].join(",");

        let groups = multiplied_row
            .split(",")
            .map(|d| d.parse::<usize>().ok().unwrap())
            .collect();

        SpringRow { conditions, groups }
    }
    
    fn is_valid(&self, original: &Vec<usize>) -> bool {
        let re = Regex::new(r"#+").unwrap();
        let string = format!("{}", &self);

        let groups: Vec<usize> = re
            .find_iter(&string)
            .map(|g| g.as_str().to_string().len())
            .collect();

        groups == *original
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Condition {
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

pub fn parse_input(path: &str) -> Result<Onsen, Box<dyn Error>>{
    let input = fs::read_to_string(path)?;
    
    let springs: Vec<SpringRow> = input
        .lines()
        .map(SpringRow::parse)
        .collect();

    Ok(Onsen::new(springs))
}

pub fn parse_input2(path: &str) -> Result<Onsen, Box<dyn Error>>{
    let input = fs::read_to_string(path)?;
    
    let springs: Vec<SpringRow> = input
        .lines()
        .map(SpringRow::parse2)
        .collect();

    Ok(Onsen::new(springs))
}

pub fn count_arrangements(
    row: &SpringRow,
    idx: usize,
    mut counter: usize,
    original: &Vec<usize>
) -> usize {
    if let Some(group_size) = row.groups.iter().next() {
        for start in idx..=row.conditions.len() - group_size {

            let prev_operational = if start == 0 {
                true
            } else {
                match row.conditions[start-1] {
                    Condition::Operational => true,
                    Condition::Damaged => continue,
                    Condition::Unknown => true,
                }
            };

            let is_fit = match row.conditions[start] {
                Condition::Operational => continue,
                _ => { 
                    row.conditions[start..start+group_size]
                        .iter()
                        .all(|c| -> bool {
                            match c {
                                Condition::Operational => false,
                                Condition::Damaged => true,
                                Condition::Unknown => true,
                            }
                        })
                    }
                };

            let next_operational = if start+group_size == row.conditions.len() {
                true
            } else {
                match row.conditions[start+group_size] {
                    Condition::Operational => true,
                    Condition::Damaged => false,
                    Condition::Unknown => true,
                }
            };

            if prev_operational && is_fit && next_operational {
                let end = start + group_size;

                let groups = if row.groups.len() > 1 {
                    row.groups[1..].to_vec()
                } else {
                    vec![]
                };

                let fill_conditions = vec![Condition::Damaged; *group_size];
                let mut conditions = row.conditions.to_vec();
                conditions[start..end].copy_from_slice(&fill_conditions);
                if end < conditions.len() {
                    conditions[end] = Condition::Operational;
                }
                
                let new_row = SpringRow::new(conditions, groups);
                counter = count_arrangements(&new_row, start+group_size, counter, original);
                
            }
        }  
    } else {
        if row.is_valid(original) {
            return counter + 1
        }
    }

    counter
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

    #[test]
    fn count_arrangements_1() {
        let groups = vec![1, 1, 3];
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
        let row = SpringRow::new(conditions, groups);
        
        let expected = 4;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }
    
    #[test]
    fn count_arrangements_2() {
        let groups = vec![3, 2, 1];
        let conditions = vec![
            Condition::Unknown,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
        ];
        let row = SpringRow::new(conditions, groups);
        
        let expected = 10;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }

    #[test]
    fn count_arrangements_3() {
        let groups = vec![1, 1, 3];
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
        ];
        let row = SpringRow::new(conditions, groups);
        
        let expected = 1;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }

    #[test]
    fn count_arrangements_4() {
        let groups = vec![1, 3, 1, 6];
        let conditions = vec![
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
        ];
        let row = SpringRow::new(conditions, groups);

        let expected = 1;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }

    #[test]
    fn count_arrangements_5() {
        let groups = vec![1, 6, 5];
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Operational,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Operational,
        ];
        let row = SpringRow::new(conditions, groups);

        let expected = 4;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }
    // ??.??#?###?

    #[test]
    fn count_arrangements_6() {
        let groups = vec![1, 7];
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Unknown,
        ];
        let row = SpringRow::new(conditions, groups);

        let expected = 4;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }

    #[test]
    fn count_arrangements_7() {
        let groups = vec![1, 1, 2];
        let conditions = vec![
            Condition::Unknown,
            Condition::Operational,
            Condition::Operational,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Operational,
        ];
        let row = SpringRow::new(conditions, groups);

        let expected = 7;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }

    #[test]
    fn count_arrangements_8() {
        let groups = vec![2, 1, 1, 1];
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Damaged,
        ];
        let row = SpringRow::new(conditions, groups);

        let expected = 11;
        let output = count_arrangements(&row, 0, 0, &row.groups.clone());
        assert_eq!(expected, output);
    }

    #[test]
    fn parse_input2_1() {
        let input = "./test_files/p2_case1.txt";
        let output = parse_input2(input).unwrap();

        let mut spring_rows: Vec<SpringRow> = vec![];
        let conditions = vec![
            Condition::Operational,
            Condition::Damaged,
            Condition::Operational,
            Condition::Damaged,
            Condition::Operational,
            Condition::Damaged,
            Condition::Operational,
            Condition::Damaged,
            Condition::Operational,
            Condition::Damaged,
        ];
        let groups: Vec<usize> = vec![1, 1, 1, 1, 1];
        spring_rows.push(SpringRow { conditions, groups });
        let expected = Onsen::new(spring_rows);

        assert_eq!(expected, output);
    }

    #[test]
    fn parse_input2_2() {
        let input = "./test_files/p2_case2.txt";
        let output = parse_input2(input).unwrap();

        let mut spring_rows: Vec<SpringRow> = vec![];
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,

        ];
        let groups: Vec<usize> = vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3];
        spring_rows.push(SpringRow { conditions, groups });
        let expected = Onsen::new(spring_rows);

        assert_eq!(expected, output);
    }
}
