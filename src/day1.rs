use std::fs;

fn trebuchet_values() -> usize {
    if let Some(values) = fs::read_to_string("data/1.input").ok() {
        return values
            .lines()
            .filter_map(|line| {
                // Get all ascii digits.
                let nums: Vec<char> = line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect();

                match nums.len() {
                    0 => None,
                    _ => {
                        // Get the first and last number.
                        let first = nums.first().unwrap();
                        let last = nums.last().unwrap();

                        // Concat and parse as a number.
                        return format!("{}{}", first, last).parse::<usize>().ok();
                    }
                }
            }).sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", trebuchet_values());
}
