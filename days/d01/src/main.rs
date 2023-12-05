use std::fs;
use std::collections::HashMap;

fn main() {
    let path = String::from("./calibration_doc.txt");
    let contents = fs::read_to_string(&path)
        .map(|x| x.to_string())
        .unwrap();
    
    let mut products: Vec<u32> = Vec::new();
    let mut products2: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let first_digit = get_first_digit(&line);
        let second_digit = get_second_digit(&line);

        let mut number = first_digit.to_string();
        number.push(second_digit);

        products.push(number.parse::<u32>().unwrap());

        let first_digit = get_first_number(&line.to_string()).unwrap();
        let second_digit = get_second_number(&line.to_string()).unwrap();

        let mut number = first_digit.to_string();
        number.push(second_digit);


        products2.push(number.parse::<u32>().unwrap());
    }

    let result: u32 = products.iter().sum();
    let result2: u32 = products2.iter().sum();

    println!("Total sum {}", result);
    println!("Total sum2 {}", result2);
}

fn get_first_number(line: &String) -> Option<char> {
    let names = HashMap::from([
        ('o', vec![("one".to_string(), '1')]),
        ('t', vec![
            ("two".to_string(), '2'),
            ("three".to_string(), '3')]
        ),
        ('f', vec![
            ("four".to_string(), '4'),
            ("five".to_string(), '5'),
        ]),
        ('s', vec![
            ("six".to_string(), '6'),
            ("seven".to_string(), '7')]
        ),
        ('e', vec![("eight".to_string(), '8')]),
        ('n', vec![("nine".to_string(), '9')]),
        ('z', vec![("zero".to_string(), '0')]),
    ]);

    let iterator = line.chars().enumerate();
    let mut return_val = None;

    'outer: for (index, value) in iterator {
        match value {
            '0'..='9' => {
                return_val =  Some(value);
                break 'outer
            },
            _ => {
                if  names.contains_key(&value) {
                    let elements = names.get(&value).unwrap();
        
                    for (name, name_val) in elements {
                        let index_f = index + name.len();

                        if index_f - 1 >= line.len() {
                            continue;
                        }

                        let peek_word =
                            &line
                            .chars()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            [index..index_f]
                            .join("");

                        if name == peek_word {
                            return_val = Some(*name_val);
                            break 'outer
                        }
                    }
                }
            }
        }
    }
    return_val
}

fn get_second_number(line: &String) -> Option<char> {
    let names = HashMap::from([
        ('e', vec![
            ("eno".to_string(), '1'),
            ("eerht".to_string(), '3'),
            ("evif".to_string(), '5'),
            ("enin".to_string(), '9'),
        ]),
        ('o', vec![
            ("owt".to_string(), '2'),
            ("orez".to_string(), '0'),
        ]),
        ('r', vec![
            ("ruof".to_string(), '4'),
        ]),
        ('n', vec![
            ("neves".to_string(), '7')
        ]),
        ('x', vec![
            ("xis".to_string(), '6'),
        ]),
        ('t', vec![
            ("thgie".to_string(), '8')
        ]),
    ]);

    let line = &line.chars().rev().collect::<String>();
    let iterator = line.chars().enumerate();
    let mut return_val = None;

    'outer: for (index, value) in iterator {
        match value {
            '0'..='9' => {
                return_val =  Some(value);
                break 'outer
            },
            _ => {
                if  names.contains_key(&value) {
                    let elements = names.get(&value).unwrap();

                    for (name, name_val) in elements {
                        let index_f = index + name.len();
                        
                        if index_f - 1 >= line.len() {
                            continue;
                        }

                        let peek_word =
                            &line
                            .chars()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            [index..index_f]
                            .join("");

                        if name == peek_word {
                            return_val = Some(*name_val);
                            break 'outer
                        }
                    }
                }
            }
        }
    }
    return_val
}

fn get_first_digit(line: &str) -> char {
    line
        .chars()
        .filter(|x| x.is_digit(10))
        .next()
        .unwrap()
}

fn get_second_digit(line: &str) -> char {
    line
        .chars()
        .rev()
        .filter(|x| x.is_digit(10))
        .next()
        .unwrap()
}
