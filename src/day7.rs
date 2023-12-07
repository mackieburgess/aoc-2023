use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

enum Ruleset {
    Standard,
    Jokers
}

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
    bid: usize,
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

fn determine_rank(cards: &Vec<usize>, ruleset: &Ruleset) -> Rank {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    match ruleset {
        Ruleset::Standard => {
            cards.iter().for_each(|card| {
                counts.entry(*card).and_modify(|v| *v += 1).or_insert(1);
            });
        },
        Ruleset::Jokers => {
            // Find the best card number to add your jokers to.

            cards.iter().for_each(|card| {
                if card != &1 {
                    counts.entry(*card).and_modify(|v| *v += 1).or_insert(1);
                }
            });

            let mut best_to_add_to = counts.clone().into_iter().collect::<Vec<(usize, usize)>>();
            let number_of_jokers = cards.iter().filter(|card| *card == &1).count();

            best_to_add_to.sort_by(|a, b| {
                // Sort by number of appearances, then by highest card.
                return match b.1.cmp(&a.1) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => b.0.cmp(&a.0)
                }
            });

            if best_to_add_to.len() == 0 {
                // Only thing is jokers, you have five-of-a-kind jokers.
                counts
                    .entry(1)
                    .or_insert(5);
            } else {
                counts
                    .entry(best_to_add_to[0].0)
                    .and_modify(|v| *v += number_of_jokers);
            }

        }
    }

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

fn parse_line(line: &str, ruleset: &Ruleset) -> Option<Hand> {
    if let Some((cards, bid)) = line.split_once(" ") {
        let cards = cards
            .chars()
            .filter_map(|c| {
                return match c.to_digit(10) {
                    Some(v) => Some(v as usize),
                    None => match c {
                        'T' => Some(10),
                        'J' => match ruleset {
                            Ruleset::Standard => Some(11),
                            Ruleset::Jokers => Some(1),
                        },
                        'Q' => Some(12),
                        'K' => Some(13),
                        'A' => Some(14),
                        _ => None
                    }
                }
            }).collect::<Vec<usize>>();

        if let Some(bid) = bid.parse::<usize>().ok() {
            let rank = determine_rank(&cards, ruleset);
            return Some(Hand { rank, cards, bid })
        }
    }

    None
}

fn sum_of_winnings(ruleset: Ruleset) -> usize {
    // Build cards based on a ruleset, since jokers mode only changes a few things.
    if let Some(input) = fs::read_to_string("data/7.input").ok() {
        let mut winnings = input
            .lines()
            .filter_map(|line| parse_line(line, &ruleset))
            .collect::<Vec<Hand>>();

        winnings.sort();

        return winnings.iter().enumerate().map(|(idx, w)| (idx + 1) * w.bid).sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", sum_of_winnings(Ruleset::Standard));
    println!("part two: {}", sum_of_winnings(Ruleset::Jokers));
}
