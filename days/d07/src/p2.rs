use std::fs;
use regex::Regex;
use std::cmp::Ordering;
use counter::Counter;

#[derive(Debug)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    A,
}

impl Card {
    fn new(input: char) -> Card {
        match input {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::A,
            _ => panic!("not a card")
        }
    }

    fn value(&self) -> u32 {
        match self {
            Card::Joker => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Queen => 12,
            Card::King => 13,
            Card::A => 14,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x1 = self.value();
        let x2 = other.value();

        match x1 == x2 {
            true => Some(Ordering::Equal),
            false => {
                if x1 > x2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


impl HandStrength {

    fn parse(hand: String) -> HandStrength {
        let counts: Counter<char> = hand
            .chars()
            .collect();

        let by_common = counts
            .most_common()
            .into_iter()
            .filter(|(key, _)| *key!='J')
            .collect::<Vec<(char, usize)>>();

        let strength: HandStrength;
        if Self::is_five_of_a_kind(&by_common) {
            strength = HandStrength::FiveOfAKind;
        } else if Self::is_four_of_a_kind(&by_common) {
            strength = HandStrength::FourOfAKind;
        } else if Self::is_full_house(&by_common) {
            strength = HandStrength::FullHouse;
        } else if Self::is_three_of_a_kind(&by_common) {
            strength = HandStrength::ThreeOfAKind;
        } else if Self::is_two_pair(&by_common) {
            strength = HandStrength::TwoPair;
        } else if Self::is_one_pair(&by_common) {
            strength = HandStrength::OnePair;
        } else {
            strength = HandStrength::HighCard;
        }
        
        match strength {
            HandStrength::FiveOfAKind => strength,
            HandStrength::FourOfAKind => {
                if let Some(_) = counts.get(&'J') {
                    HandStrength::FiveOfAKind
                } else {
                    strength
                }
            },
            HandStrength::FullHouse => strength,
            HandStrength::ThreeOfAKind => {
                if let Some(n_jokers) = counts.get(&'J') {
                    match n_jokers {
                        1 => HandStrength::FourOfAKind,
                        2 => HandStrength::FiveOfAKind,
                        _ => panic!(),
                    }
                } else {
                    strength
                }
            },
            HandStrength::TwoPair => {
                if let Some(_) = counts.get(&'J') {
                    HandStrength::FullHouse
                } else {
                    strength
                }
            },
            HandStrength::OnePair => {
                if let Some(n_jokers) = counts.get(&'J') {
                    match n_jokers {
                        1 => HandStrength::ThreeOfAKind,
                        2 => HandStrength::FourOfAKind,
                        3 => HandStrength::FiveOfAKind,
                        _ => panic!(),
                    }
                } else {
                    strength
                }
            },
            HandStrength::HighCard => {
                if let Some(n_jokers) = counts.get(&'J') {
                    match n_jokers {
                        1 => HandStrength::OnePair,
                        2 => HandStrength::ThreeOfAKind,
                        3 => HandStrength::FourOfAKind,
                        _ => HandStrength::FiveOfAKind,
                    }
                } else {
                    strength
                }
            },
        }
    }

    fn is_five_of_a_kind(hand: &Vec<(char, usize)>) -> bool {
        hand
            .iter()
            .any(|(_, val)| *val == 5) 
    }

    fn is_four_of_a_kind(hand: &Vec<(char, usize)>) -> bool {
        hand
            .iter()
            .any(|(_, val)| *val == 4) 
    }

    fn is_full_house(hand: &Vec<(char, usize)>) -> bool {
        let pair_exists = hand
            .iter()
            .any(|(_, val)| *val == 2);

        let third_exists = hand
            .iter()
            .any(|(_, val)| *val == 3);

        pair_exists && third_exists
    }

    fn is_three_of_a_kind(hand: &Vec<(char, usize)>) -> bool {
        let no_pair_exists = !hand
            .iter()
            .any(|(_, val)| *val == 2);

        let third_exists = hand
            .iter()
            .any(|(_, val)| *val == 3);

        no_pair_exists && third_exists
    }

    fn is_two_pair(hand: &Vec<(char, usize)>) -> bool {
                let total_pairs = hand
                    .iter()
                    .filter(|(_, val)| *val == 2)
                    .collect::<Vec<&(char, usize)>>()
                    .len();

                total_pairs == 2
    }

    fn is_one_pair(hand: &Vec<(char, usize)>) -> bool {
        let total_pairs = hand
            .iter()
            .filter(|(_, val)| *val == 2)
            .collect::<Vec<&(char, usize)>>()
            .len();

        total_pairs == 1
    }

    fn value(&self) -> u32 {
        match self {
            HandStrength::FiveOfAKind => 1,
            HandStrength::FourOfAKind => 2,
            HandStrength::FullHouse => 3,
            HandStrength::ThreeOfAKind => 4,
            HandStrength::TwoPair => 5,
            HandStrength::OnePair => 6,
            HandStrength::HighCard => 7,
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u32,
    pub strength: HandStrength,
    pub key: String,
}

impl Hand {
    fn new(literal: String, bid: u32) -> Hand {
        let cards: Vec<Card> = literal
            .chars()
            .map(|c| Card::new(c))
            .collect();

        let strength = HandStrength::parse(literal);
        
        let mut values: Vec<i32> = cards.iter().map(|c| 14 - (c.value() as i32)).collect();
        let mut key_nums = vec![(strength.value() as i32)];
        key_nums.append(&mut values);
        
        let key = key_nums
            .iter()
            .map(|k| format!("{:0>2}", k))
            .collect::<Vec<String>>()
            .join("");

        Hand { cards, bid, strength, key }
    }
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    let mut hands: Vec<Hand> = Vec::new();

    if let Ok(s) = fs::read_to_string(input) {
        for line in s.lines() {
            let captures_iter = re.captures_iter(line).map(|c| c.extract());
            for (_, [cards, bid]) in captures_iter {
                let cards: String = cards.to_string();
                let bid: u32 = bid.parse().unwrap();

                let hand = Hand::new(cards, bid);
                hands.push(hand);
            }
        }
    } else {
        panic!("File not found :(")
    }

    hands
}
