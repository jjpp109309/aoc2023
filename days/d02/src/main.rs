use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    let config: HashMap<String, u32> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    let path = String::from("./games.txt");
    let contents = fs::read_to_string(&path).unwrap();
    let mut games: Vec<(u32, HashMap<String, u32>)> = Vec::new();

    
    for line in contents.lines() {
        let (game_id, draws_string) = get_game_id(&line);
        let min_game = get_draws(&draws_string);

        games.push((game_id, min_game));
    }

    let total_valid_games: u32 = games
        .iter()
        .map(|x| add_valid_game(&config, x))
        .sum();
        
    println!("total valid games {}", total_valid_games);

    let sum_of_powers: u32 = games
        .iter()
        .map(|x| game_power(x))
        .sum();

    println!("sum of powers {}", sum_of_powers);

}

fn get_game_id(line: &str) -> (u32, String) {
    let split_line: Vec<&str> = line.split(":").collect();
    let re_id = Regex::new(r"\d+").unwrap();

    let game_id = re_id
        .find_iter(split_line[0])
        .map(|m| m.as_str())
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    (game_id, split_line[1].to_string())
}

fn get_draws(draws_string: &String) -> HashMap<String, u32> {

    let re_color = Regex::new(r"[a-z]+").unwrap();
    let re_n = Regex::new(r"\d+").unwrap();

    let mut max_game = HashMap::from([
        ("red".to_string(), 0),
        ("green".to_string(), 0),
        ("blue".to_string(), 0),
    ]);


    for draw in draws_string.split(';') {
        for color in draw.split(',') {

            let die = re_color
                .find_iter(color)
                .map(|m| m.as_str())
                .next()
                .unwrap()
                .to_string();

            let count: u32 = re_n
                .find_iter(color)
                .map(|m| m.as_str())
                .next()
                .unwrap()
                .parse()
                .unwrap();

            if &count > max_game.get(&die).unwrap() {
                max_game.insert(die, count);
            };
        }
    }

    max_game
}

fn add_valid_game(
    config: &HashMap<String, u32>,
    min_game: &(u32, HashMap<String, u32>)
) -> u32 {

    let (game_id, game) = min_game;

    for (color, count) in game {
        if config.get(color).unwrap() < count {
            return 0
        }
    }

    *game_id
}

fn game_power(min_game: &(u32, HashMap<String, u32>)) -> u32 {

    let (_, game) = min_game;

    let mut total: u32 = 1;
    for (_, count) in game {
        total *= count
    }

    total
}
