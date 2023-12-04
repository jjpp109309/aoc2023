use std::fs;

fn parse_input(path: &String) -> String {

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

        let path = "./test_input.txt".to_string();
        let input = parse_input(&path);

        assert_eq!(expected, input, "\nExpected:\n{} \n\n Got:\n{}", expected, input);
    }
}
