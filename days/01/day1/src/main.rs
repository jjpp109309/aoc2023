use std::fs;

fn main() {
    let path = String::from("./calibration_doc.txt");
    let contents = fs::read_to_string(&path).unwrap();
    
    let mut products: Vec<u32> = Vec::new();
    let mut products2: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let first_digit = get_first_digit(&line);
        let second_digit = get_second_digit(&line);

        let mut number = first_digit.to_string();
        number.push(second_digit);

        products.push(number.parse::<u32>().unwrap());

        let first_digit = get_first_number(&line);
        
    }

    let result: u32 = products.iter().sum();

    println!("Total sum {}", result);
}

fn get_first_number(line: &str) -> _ {
    for (index, value) in line.chars().enumerate() {
        
    }
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
