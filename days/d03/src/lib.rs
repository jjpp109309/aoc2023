use std::fs;
use regex::Regex;
use std::collections::{HashMap, hash_map::Entry};

pub fn parse_input(path: &str) -> String {

    let input = fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|x| format!("{}{}{}", ".", x, "."))
        .collect::<Vec<String>>()
        .join("\n");

    let line_len = input.lines().next().unwrap().len();

    let mut pad_string_vec: Vec<String> = Vec::new();
    for _ in 0..line_len {
        pad_string_vec.push(".".to_string())
    }
    let pad_string = pad_string_vec.join("");
    
    vec![
        pad_string.clone(),
        input,
        pad_string.clone(), 
        "<EoF>".to_string()
    ].join("\n")
}

struct Lexer<'a> {
    input: std::str::Lines<'a>,
    prv_line: String,
    cur_line: String,
    nxt_line: String,
    cur_line_number: i32
}

impl Lexer<'_> {

    fn new(input: &String) -> Lexer {
        let mut lexer = Lexer {
            input: input.lines(),
            prv_line: "".to_string(),
            cur_line: "".to_string(),
            nxt_line: "".to_string(),
            cur_line_number: -2,
        };

        lexer.next_line();
        lexer.next_line();
        lexer.next_line();

        lexer
    }

    fn next_line(&mut self) {
        self.prv_line = self.cur_line.to_owned();
        self.cur_line = self.nxt_line.to_owned();
        self.nxt_line = self.input.next().unwrap_or("").to_string();

        self.cur_line_number += 1
    }
}

pub fn get_part_numbers(input: &String) -> Vec<u32> {

    let mut lexer = Lexer::new(input);
    let re_find_digits = Regex::new(r"\d+").unwrap();

    let mut part_numbers: Vec<u32> = Vec::new();

    while lexer.nxt_line != "<EoF>".to_string() {

        let numbers = re_find_digits.find_iter(&lexer.cur_line);

        for number in numbers {
            let start = number.start();
            let end = number.end();
            
            let upper_slice = lexer
                .prv_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[start-1..end+1]
                .join("")
                .to_owned();

            let lower_slice = lexer
                .nxt_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[start-1..end+1]
                .join("")
                .to_owned();

            let left_char = lexer
                .cur_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[start-1]
                .to_owned();

            let right_char = lexer
                .cur_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[end]
                .to_owned();

            let margin = vec![lower_slice, upper_slice, left_char, right_char].join("");

            if margin.chars().any(|x| x != '.') {
                
                let part_number: u32 = lexer
                    .cur_line
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()[start..end]
                    .join("")
                    .parse()
                    .unwrap();
                
                part_numbers.push(part_number);
            }
        }

        lexer.next_line();
    }

    part_numbers
}

#[derive(Debug)]
pub struct GearNumber {
    pub number: u32,
    pub gear_id: Vec<String>
}

pub fn get_gears(input: &String) -> Vec<GearNumber> {

    let mut lexer = Lexer::new(input);
    let re_find_digits = Regex::new(r"\d+").unwrap();
    let re_gear = Regex::new(r"\*").unwrap();

    let mut gear_numbers: Vec<GearNumber> = Vec::new();

    while lexer.nxt_line != "<EoF>".to_string() {

        let numbers = re_find_digits.find_iter(&lexer.cur_line);

        for number in numbers {
            let start = number.start();
            let end = number.end();
            
            let upper_slice = lexer
                .prv_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[start-1..end+1]
                .join("")
                .to_owned();

            let lower_slice = lexer
                .nxt_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[start-1..end+1]
                .join("")
                .to_owned();

            let left_char = lexer
                .cur_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[start-1]
                .to_owned();

            let right_char = lexer
                .cur_line
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[end]
                .to_owned();

            let part_number: u32 = lexer
                    .cur_line
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()[start..end]
                    .join("")
                    .parse()
                    .unwrap();

            let mut gear_number = GearNumber {
                number: part_number,
                gear_id: Vec::new()
            };

            if upper_slice.chars().any(|x| x == '*') {
                for gear in re_gear.find_iter(&upper_slice) {
                    let gear_id = format!(
                        "{}-{}",
                        lexer.cur_line_number - 1,
                        number.start() + gear.start() - 1
                    );
                    
                    gear_number.gear_id.push(gear_id);
                }
            }

            if lower_slice.chars().any(|x| x == '*') {
                for gear in re_gear.find_iter(&lower_slice) {
                    let gear_id = format!(
                        "{}-{}",
                        lexer.cur_line_number+1,
                        number.start() + gear.start() - 1
                    );
                    
                    gear_number.gear_id.push(gear_id);
                }
            }

            if left_char == '*'.to_string() {
                let gear_id = format!(
                    "{}-{}",
                    lexer.cur_line_number,
                    number.start() - 1
                    );

                gear_number.gear_id.push(gear_id);
            }

            if right_char == '*'.to_string() {
                let gear_id = format!(
                    "{}-{}",
                    lexer.cur_line_number,
                    number.end()
                    );

                gear_number.gear_id.push(gear_id);
            }

            if gear_number.gear_id.len() > 0 {
                gear_numbers.push(gear_number)
            }
        }

        lexer.next_line();
    }

    gear_numbers
}

pub fn get_true_gears(gear_numbers: &mut Vec<GearNumber>) -> Vec<u32> {
    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();

    for gear_number in gear_numbers {
        for gear_id in &gear_number.gear_id {
            let token = gear_number.number;

            match gears.entry(gear_id.to_owned()) {
                Entry::Vacant(e) => { e.insert(vec![token]); },
                Entry::Occupied(mut e) => e.get_mut().push(token)
            };
        }
    }

    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .collect::<Vec<u32>>()

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let expected = "\
............
.467..114...
....*.......
...35..633..
.......#....
.617*.......
......+.58..
...592......
.......755..
....$.*.....
..664.598...
............".to_string();

        let path = "./test_input.txt";
        let input = parse_input(&path);

        assert_eq!(expected, input, "\nExpected:\n{} \n\n Got:\n{}", expected, input);
    }

    #[test]
    fn generate_lexer() {

        let path = "./test_input.txt";
        let input = parse_input(&path);
        let lexer = Lexer::new(&input);

        assert_eq!("............".to_string(), lexer.prv_line);
        assert_eq!(".467..114...".to_string(), lexer.cur_line);
        assert_eq!("....*.......".to_string(), lexer.nxt_line);
    }
}
