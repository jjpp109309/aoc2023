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

fn count_arrangements(row: &SpringRow, idx: usize, mut counter: usize) -> usize {
    if let Some(group_size) = row.groups.iter().next() {
        for start in idx..=row.conditions.len() - group_size {

            println!("-------------------------------");
            println!("cond {}", row.conditions.len() - group_size);
            println!("row {:?}", row);
            println!("idx {:?}", idx);
            println!("counter {:?}", counter);
            println!("start: {}", start);
            let prev_operational = if idx == 0 {
                true
            } else {
                match row.conditions[start-1] {
                    Condition::Operational => true,
                    Condition::Damaged => continue,
                    Condition::Unknown => true,
                }
            };
            println!("prev: {}", prev_operational);

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
            println!("is_fit: {}", is_fit);

            let next_operational = if start+group_size == row.conditions.len() {
                true
            } else {
                match row.conditions[start+group_size] {
                    Condition::Operational => true,
                    Condition::Damaged => false,
                    Condition::Unknown => true,
                }
            };


            println!("next: {}", next_operational);

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
                println!("new row{:?}", new_row);
                counter = count_arrangements(&new_row, start+group_size, counter);
            }
        }  
    } else {
        return counter + 1
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
        let output = count_arrangements(&row, 0, 0);
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
        let output = count_arrangements(&row, 0, 0);
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
        let output = count_arrangements(&row, 0, 0);
        assert_eq!(expected, output);
    }
}
