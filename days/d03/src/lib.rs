use std::fs;

pub fn parse_input(path: &str) -> String {

    let input = fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|x| format!("{}{}{}", ".", x, "."))
        .collect::<Vec<String>>()
        .join("\n");

    let line_len = input.lines().next().unwrap().len();
    println!("{}", line_len);

    let mut pad_string_vec: Vec<String> = Vec::new();
    for _ in 0..line_len {
        pad_string_vec.push(".".to_string())
    }
    let pad_string = pad_string_vec.join("");
    
    vec![pad_string.clone(), input, pad_string.clone()].join("\n")
}

struct Lexer<'a> {
    input: std::str::Lines<'a>,
    prv_line: String,
    cur_line: String,
    nxt_line: String
}

impl Lexer<'_> {

    fn new(input: &String) -> Lexer {
        let mut lexer = Lexer {
            input: input.lines(),
            prv_line: "".to_string(),
            cur_line: "".to_string(),
            nxt_line: "".to_string(),
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
    }
}

pub fn get_part_numbers(input: &String) -> Vec<u32> {

    vec![32]
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
}
