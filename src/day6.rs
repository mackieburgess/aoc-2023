use std::fs;

fn ways_to_win() -> usize {
    if let Some(input) = fs::read_to_string("data/6.input").ok() {
        if let Some((time, distance)) = input.split_once("\n") {
            let time = time
                .split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            let distance = distance
                .split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            // Count the product of the possible ways to win.
            return time
                .iter()
                .zip(distance)
                .map(|(t, d)| {
                    (0..=*t).enumerate().map(|(strategy, idx)| {
                        (t - strategy) * idx > d
                    }).filter(|n| *n).count()
                }).product::<usize>();
        }

        return 0;
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", ways_to_win());
}
