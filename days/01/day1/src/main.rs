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
        
    }

    let result: u32 = products.iter().sum();

    println!("Total sum {}", result);
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
        println!("line {}, index {}", line, index);
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
                        println!("\tindex {}, name {}, line_len {}", index_f, name, line.len());
                        
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
