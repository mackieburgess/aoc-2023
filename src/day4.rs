use std::fs;
use std::collections::HashSet;

fn wins(card: &str) -> usize {
    if let Some((_, data)) = card.split_once(": ") {
        if let Some((winning, ours)) = data.split_once(" | ") {
            let winning = winning
                .split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<HashSet<usize>>();

            let wins = ours
                .split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .filter(|v| winning.contains(v))
                .collect::<Vec<usize>>()
                .len();

            return wins;
        }
    }
    panic!("improper file format: {card}")
}

fn total_winnings() -> usize {
    // Get the sum of 2^wins for each card.

    if let Some(input) = fs::read_to_string("data/4.input").ok() {
        return input.lines().map(|card| {
            let wins = wins(card);

            if wins == 0 {
                return 0;
            } else {
                return 1 << (wins - 1);
            }
        }).sum();
    }

    panic!("file not found")
}

fn scratchcard_quantities() -> usize {
    if let Some(input) = fs::read_to_string("data/4.input").ok() {
        let lines = input.lines().collect::<Vec<&str>>();

        // Winning n gives you an extra card for the next n cards.
        // This compounds strongly so we need to keep adding extra cards consistently.
        //
        // We do this by tracking the quantities of each card, and adding to it as we go.
        let mut quantities = vec![1; lines.len()];

        lines.iter().enumerate().for_each(|(idx, line)| {
            let wins = wins(line);

            for i in idx+1..=idx+wins {
                quantities[i] += quantities[idx];
            }
        });

        return quantities.iter().sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", total_winnings());
    println!("part two: {}", scratchcard_quantities());
}
