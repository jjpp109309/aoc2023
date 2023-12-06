use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug)]
pub struct PunchCard {
    pub id: u32,
    pub winner: HashSet<u32>,
    pub owned: HashSet<u32>,
    pub multiplicity: u32,
}

pub fn parse_input(path: &str) -> Vec<PunchCard> {
    let raw = read_to_string(path).unwrap();
    let mut punch_cards: Vec<PunchCard> = Vec::new();

    for line in raw.lines() {
        let id_idx = line.find(':').unwrap();

        let id: u32 = line[0..id_idx]
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let numbers = line[id_idx+1..]
            .split('|')
            .map(|x| x.split_whitespace().collect())
            .collect::<Vec<Vec<&str>>>();

        let winner: HashSet<u32> = numbers[0]
            .iter()
            .map(|x| x.parse::<u32>()
            .unwrap())
            .collect();

        let owned: HashSet<u32> = numbers[1]
            .iter()
            .map(|x| x.parse::<u32>()
            .unwrap())
            .collect();

        punch_cards.push(PunchCard { id, winner, owned, multiplicity: 1 });
    }

    punch_cards
}
 
pub fn score(cards: &Vec<PunchCard>) -> u32 {
    let score_base: u32 = 2;

    cards
        .iter()
        .map(|x| x.intersection_len())
        .collect::<Vec<u32>>()
        .iter()
        .map(|n| if n > &0 { score_base.pow(n-1) } else { 0 })
        .sum()
}

pub fn multiply_cards(cards: &mut Vec<PunchCard>) -> u32 {

    let mut card_idx = 0;
    while card_idx < cards.len() {
        let matches: usize = cards[card_idx]
            .intersection_len()
            .try_into()
            .unwrap();

        if matches > 0 {
            for _ in 0..cards[card_idx].multiplicity{
                for i in 1..=matches {
                    cards[card_idx+i].multiplicity += 1;
                }
            }
        }
        card_idx += 1;
    }
    
    cards
        .iter()
        .map(|x| x.multiplicity)
        .sum()
}

impl PunchCard {
    fn intersection_len(&self) -> u32 {
        self
            .winner.intersection(&self.owned)
            .collect::<Vec<&u32>>()
            .len()
            .try_into()
            .unwrap()
    }
}
