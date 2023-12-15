use std::fs;

fn hash_algorithm(v: &str) -> usize {
    v.chars().fold(0, |acc, x| {
        return ((acc + u64::from(x) as usize) * 17) % 256;
    })
}

fn hash_result() -> usize {
    if let Some(input) = fs::read_to_string("data/15.input").ok() {
        return input
            .trim()
            .split(',')
            .map(hash_algorithm)
            .sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", hash_result());
}
