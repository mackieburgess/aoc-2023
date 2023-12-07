use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq)]
enum Rank {
    Five    = 7,
    Four    = 6,
    Full    = 5,
    Three   = 4,
    TwoPair = 3,
    OnePair = 2,
    High   = 1
}

#[derive(PartialEq, Eq)]
struct Hand {
    rank: Rank,
    cards: Vec<usize>,
    bid: usize
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        if self.rank > other.rank {
            return Some(Ordering::Greater);
        } else if self.rank < other.rank {
            return Some(Ordering::Less);
        } else {
            for (a, b) in self.cards.iter().zip(&other.cards) {
                if a > &b {
                    return Some(Ordering::Greater);
                } else if a < &b {
                    return Some(Ordering::Less);
                }
            }
        }

        return Some(Ordering::Equal);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.partial_cmp(other) {
            Some(r) => r,
            None => Ordering::Equal
        }
    }
}

fn determine_rank(cards: &Vec<usize>) -> Rank {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    cards.iter().for_each(|card| {
        counts.entry(*card).and_modify(|v| *v += 1).or_insert(1);
    });

    let mut cols = counts.into_values().collect::<Vec<usize>>();
    cols.sort_by(|a, b| b.cmp(a));

    return match cols.iter().nth(0) {
        Some(5) => Rank::Five,
        Some(4) => Rank::Four,
        Some(3) if cols.iter().nth(1) == Some(&2) => Rank::Full,
        Some(3) => Rank::Three,
        Some(2) if cols.iter().nth(1) == Some(&2) => Rank::TwoPair,
        Some(2) => Rank::OnePair,
        _ => Rank::High
    }
}

fn parse_line(line: &str) -> Option<Hand> {
    if let Some((cards, bid)) = line.split_once(" ") {
        let cards = cards
            .chars()
            .filter_map(|c| {
                return match c.to_digit(10) {
                    Some(v) => Some(v as usize),
                    None => match c {
                        'T' => Some(10),
                        'J' => Some(11),
                        'Q' => Some(12),
                        'K' => Some(13),
                        'A' => Some(14),
                        _ => None
                    }
                }
            }).collect::<Vec<usize>>();

        if let Some(bid) = bid.parse::<usize>().ok() {
            let rank = determine_rank(&cards);
            return Some(Hand { rank, cards, bid })
        }
    }

    None
}

fn sum_of_winnings() -> usize {
    if let Some(input) = fs::read_to_string("data/7.input").ok() {
        let mut winnings = input
            .lines()
            .filter_map(|line| parse_line(line))
            .collect::<Vec<Hand>>();

        winnings.sort();

        return winnings.iter().enumerate().map(|(idx, w)| (idx + 1) * w.bid).sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", sum_of_winnings());
    // Too low.
}
