use std::fs;
use d06::{
    parse_input,
    total_wins,
};

fn main() {
    // ------------------------------------------------------------------------
    // Part 1: Test
    // ------------------------------------------------------------------------
    let input = match fs::read_to_string("./test.txt") {
        Ok(string) => parse_input(&string),
        Err(_) => panic!("File not found :("),
    };
    println!("Test input: {:?}", input);

    let wins: i32 = input.iter().map(|r| total_wins(r)).product();
    println!("Test wins: {}", wins);

    // ------------------------------------------------------------------------
    // Part 1
    // ------------------------------------------------------------------------
    let input = match fs::read_to_string("./input.txt") {
        Ok(string) => parse_input(&string),
        Err(_) => panic!("File not found :("),
    };
    println!("input: {:?}", input);

    let wins: i32 = input.iter().map(|r| total_wins(r)).product();
    println!("wins: {}", wins);

}
