use std::fs;

fn trebuchet_values(lines: Vec<String>) -> usize {
    // Find the first and last digit on each line, concatenate these two values together, return
    // the sum of all lines.

    lines
        .iter()
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
        }).sum()
}

fn get_lines() -> Vec<String> {
    // Get each line as a string.

    if let Some(values) = fs::read_to_string("data/1.input").ok() {
        return values
            .lines()
            .map(|line| line.to_string())
            .collect();
    } else {
        panic!("file not found");
    }
}

fn preprocess_lines() -> Vec<String> {
    // Get each line as a string, replacing spelled numbers with digits.
    // To account for multiple characters using the same digit, I leave any reusable starts and ends.

    get_lines()
        .into_iter()
        .map(|line| {
            return line
                .replace("one","o1e")
                .replace("two","t2o")
                .replace("three","t3e")
                .replace("four","4")
                .replace("five","5e")
                .replace("six","6")
                .replace("seven","7n")
                .replace("eight","e8t")
                .replace("nine","n9e")
                .replace("zero","0o")
        }).collect()
}


fn main() {
    println!("part one: {}", trebuchet_values(get_lines()));
    println!("part two: {}", trebuchet_values(preprocess_lines()));
}
