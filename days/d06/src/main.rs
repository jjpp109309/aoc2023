use std::fs;
use d06::{
    parse_input,
};

fn main() {
    let input = match fs::read_to_string("./test.txt") {
        Ok(string) => parse_input(&string),
        Err(_) => panic!("File not found :("),
    };
    println!("Test input: {:?}", input);
}
