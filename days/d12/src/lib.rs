use std::fs;
use std::error::Error;

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
    fn new(conditions: Vec<Condition>, groups: Vec<usize>) -> SpringRow {
        SpringRow { conditions, groups }
    }

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

        SpringRow { conditions, groups }
    }

    fn get_fountains(&self) -> Vec<Vec<Condition>> {
        self.groups
            .iter()
            .map(|x| vec![Condition::Operational; *x])
            .collect()
    }

    fn count_arrangements(&self) -> usize {
        let fountains = self.get_fountains();
        
        todo!()
    }

    fn find_next_spot(group: &Vec<Condition>, groups: &Vec<Condition>, idx: usize) -> Option<usize> {
        if group.len() <= groups.len() {
            if group.iter().zip(groups.iter()).all(|(a, b)| a==b) {
                return Some(idx)
            } else {
                return Self::find_next_spot(group, &groups[idx..].to_vec(), idx+1)
            }
        } else {
            return None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

fn find_arrangements(row: SpringRow, idx: usize, mut counter: usize) -> usize {
    if let Some(group_size) = row.groups.iter().next() {
        for i in idx..(row.conditions.len() - group_size) {
            if let Some(start) = next_location(&row.conditions, &i, group_size) {
                let end = start + group_size;

                let fill_conditions = vec![Condition::Damaged; *group_size];
                let mut conditions = row.conditions.to_vec();
                conditions[start..end].copy_from_slice(&fill_conditions);
                conditions[end] = Condition::Operational;
                
                let groups = if row.groups.len() > 1 {
                    row.groups[1..].to_vec()
                } else {
                    vec![]
                };

                let new_row = SpringRow::new(conditions, groups);

                counter = find_arrangements(new_row, i+group_size, counter);
            }
        }

        return counter
    }

    counter + 1
}

fn next_location(conditions: &Vec<Condition>, idx: &usize, group_size: &usize) -> Option<usize> {
    for i in *idx..(conditions.len() - group_size) {
        let is_fit = conditions[i..i+group_size]
            .iter()
            .all(|c| -> bool {
                match c {
                    Condition::Operational => false,
                    Condition::Damaged => true,
                    Condition::Unknown => true,
                }
            });
        
        let next_operational = match conditions[i+group_size] {
            Condition::Operational => true,
            Condition::Damaged => false,
            Condition::Unknown => true,
        };

        if is_fit && next_operational {
            return Some(i)
        }
    }

    None
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
    fn next_location_1() {
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
        
        let expected = Some(1);
        let output = next_location(&conditions, &0, &1);
        assert_eq!(expected, output);

        let expected = Some(5);
        let output = next_location(&conditions, &2, &2);
        assert_eq!(expected, output);

        let expected = Some(10);
        let output = next_location(&conditions, &0, &3);
        assert_eq!(expected, output);

        let expected = None;
        let output = next_location(&conditions, &0, &4);
        assert_eq!(expected, output);
    }
}
