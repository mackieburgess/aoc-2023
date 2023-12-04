use std::fs;
use std::collections::HashSet;

fn winnings() -> usize {
    // Get the sum of 2^wins for each card.

    if let Some(input) = fs::read_to_string("data/4.input").ok() {
        return input.lines().filter_map(|line| {
            if let Some((_, data)) = line.split_once(": ") {
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

                    // Danger arithmetic, casting here can’t be *too bad*.
                    if wins == 0 {
                        return None
                    } else {
                        return Some(2_i32.pow(wins as u32 - 1) as usize);
                    }
                }
            }

            None::<usize>
        }).sum::<usize>();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", winnings());
}
