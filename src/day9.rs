use std::fs;

enum Direction {
    Future,
    History
}

fn unfurl_reading(reading: Vec<isize>, direction: &Direction) -> isize {
    // Extrapolate by creating new layers, which represent the difference between windows of the
    // previous layer. Repeat this until the most recent layer is all zero, which means the pattern
    // has been found.

    let mut extrapolations = vec![reading];

    while extrapolations
        .iter()
        .last()
        .unwrap() // Extrapolation always has at least one element.
        .iter()
        .filter(|v| v != &&0_isize)
        .count() > 0
    {
        // The differences between each element in the most recent extrapolation.
        let new_extrapolation = extrapolations
            .iter()
            .last()
            .unwrap()
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect::<Vec<isize>>();

        extrapolations.push(new_extrapolation);
    }

    // For looking at the history, calculate from the deepest layer upwards, subtracting the
    // accumulation from the next layer up.
    return match direction {
        Direction::Future => extrapolations.iter().fold(0, |acc, x| acc + x.iter().last().unwrap()),
        Direction::History => extrapolations.iter().rev().fold(0, |acc, x| x.iter().nth(0).unwrap() - acc)
    };
}

fn oasis_scan(direction: Direction) -> isize {
    // For each line, take it as a list of integers and calculate the next integer in the pattern.
    // This can be applied forwards and backwards.

    if let Some(readings) = fs::read_to_string("data/9.input").ok() {
        let readings: Vec<Vec<isize>> = readings.lines().map(|line| {
            line.split_whitespace().filter_map(|v| v.parse::<isize>().ok()).collect()
        }).collect();

        return readings.into_iter().map(|reading| unfurl_reading(reading, &direction)).sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", oasis_scan(Direction::Future));
    println!("part two: {}", oasis_scan(Direction::History));
}
