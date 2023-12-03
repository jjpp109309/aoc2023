use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    let config = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let path = String::from("./games.txt");
    let contents = fs::read_to_string(&path).unwrap();
    let mut games: Vec<(u32, HashMap<String, i32>)> = Vec::new();

    let re_id = Regex::new(r"\d+").unwrap();
    let re_color = Regex::new(r"[a-z]+").unwrap();
    let re_n = Regex::new(r"\d+").unwrap();

    for line in contents.lines() {
        // games.push(parse_game(&line.to_string()));
        println!("{}", line);
        let split_line: Vec<&str> = line.split(":").collect();
        let game_id =
            re_id
                .find_iter(split_line[0])
                .map(|m| m.as_str())
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

        println!("game {}", line);
        println!("id {:?}", game_id);
        for draw in split_line[1].split(';') {
            println!("{:?}", draw);

            for color in draw.split(',') {
                let c =
                    re_color
                        .find_iter(color)
                        .map(|m| m.as_str())
                        .next()
                        .unwrap();

                let n =
                    re_n
                        .find_iter(color)
                        .map(|m| m.as_str())
                        .next()
                        .unwrap();

                println!("c: {}, n: {}", c, n);
            }
        }
    
    }
}

// fn parse_game(line: &String) -> (u32, HashMap<String, i32>) {
//     let mut game = HashMap::from([
//         ("red", 0),
//         ("green", 0),
//         ("blue", 0),
//     ]);
//
//
// }
