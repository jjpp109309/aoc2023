use std::fs;
use regex::Regex;
use std::cmp::Ordering;
use counter::Counter;

enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    A,
}

impl Card {
    fn new(input: char) -> Card {
        match input {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::A,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Jack => 11,
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

enum HandStrength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    HighCard,
}

impl PartialEq for HandStrength {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl HandStrength {
    fn parse(hand: String) -> HandStrength {
        let by_common = hand.chars().collect::<Counter<_>>().most_common_ordered();

        if by_common.iter().map(|(c, val)| val == 5.into()).any() {
            HandStrength::FiveOfAKind
        }
    }

    fn value(&self) -> u32 {
        match self {
            HandStrength::FiveOfAKind => 1,
            HandStrength::FourOfAKind => 2,
            HandStrength::FullHouse => 3,
            HandStrength::ThreeOfAKind => 4,
            HandStrength::TwoPair => 5,
            HandStrength::HighCard => 6,
        }
    }
}

impl PartialOrd for HandStrength {
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

struct Hand {
    cards: Vec<Card>,
    bid: u32,
    strength: HandStrength,
}

impl Hand {
    fn new(cards: String, bid: u32) -> Hand {
        let cards: Vec<Card> = cards.chars().map(|c| Card::new(c)).collect();
        
    }
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    let hands: Vec<Hand> = Vec::new();

    if let Ok(s) = fs::read_to_string(input) {
        for line in s.lines() {
            re.captures_iter(line).map(|caps| {
                let (_, [cards, bid]) = caps.extract();
                let cards: String = cards.to_string();
                let bid: u32 = bid.parse().unwrap();

                let hand = Hand::new(cards, bid);
            });
        }
    } else {
        panic!("File not found :(")
    }

    todo!()
}
