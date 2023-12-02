#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<String>,
    pub position: usize,
    pub read_position: usize,
    pub ch: String,
}

impl Lexer {
    pub fn new(input: &String) -> Self {
        let input_chars = input
            .chars()
            .map(|x| x.to_string())
            .collect();

        let mut l = Lexer {
            input: input_chars,
            position: 0,
            read_position: 0,
            ch: "".to_string(),
        };
        
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = String::from("");
        } else {
            self.ch = self.input[self.read_position].to_owned();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Option<u32> {
        if self.ch.chars().next().unwrap().is_digit(10) {
            return Some(self.ch.parse::<u32>().unwrap())
        }

        let character = self.ch.as_str();

        match character {
            "o" => {
                let idx = self.read_position + 2;
                let word = self.input[self.read_position..idx].join("");

                if word == "one".to_string() {
                    return Some(1)
                }
            },
            "e" => {
                let idx = self.read_position + 4;
                let word = self.input[self.read_position..idx].join("");

                if  word == "eight".to_string() {
                    return Some(9)
                }

            },
            "n" => {
                let idx = self.read_position + 3;
                let word = self.input[self.read_position..idx].join("");

                if  word == "nine".to_string() {
                    return Some(9)
                }
            },
            "f" => {
                let idx = self.read_position + 3;
                let word = self.input[self.read_position..idx].join("");

                if  word == "four".to_string() {
                    return Some(4)
                }

                let idx = self.read_position + 3;
                let word = self.input[self.read_position..idx].join("");

                if  word == "five".to_string() {
                    return Some(5)
                }
            },
            "t" => {
                let idx = self.read_position + 2;
                let word = self.input[self.read_position..idx].join("");

                if  word == "two".to_string() {
                    return Some(2)
                }

                let idx = self.read_position + 4;
                let word = self.input[self.read_position..idx].join("");

                if  word == "three".to_string() {
                    return Some(5)
                }
            },
            "s" => {
                let idx = self.read_position + 2;
                let word = self.input[self.read_position..idx].join("");

                if  word == "six".to_string() {
                    return Some(6)
                }

                let idx = self.read_position + 4;
                let word = self.input[self.read_position..idx].join("");

                if  word == "seven".to_string() {
                    return Some(7)
                }
            }
            _ => self.read_char(),
        }

        None
    }

    fn try_value(&self, digit: usize, name: String) -> Option<u32> {
        let idx = self.read_position + digit;
        let word = self.input[self.read_position..idx].join("");

        if word == name {
            Some(digit as u32)
        } else {
            None
        }
    }
}
