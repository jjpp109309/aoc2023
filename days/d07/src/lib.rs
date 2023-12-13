use std::cmp::Ordering;

enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    A,
}

impl Card {
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
            Card::Ten => 10,
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
