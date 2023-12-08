use std::fs;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;

fn gcd(mut a: usize, mut b: usize) -> usize {
    // gcd snippet.

    if a == b { return a; }
    (a, b) = (a.min(b), a.max(b));
    while b > 0 {
        (a, b) = (b, a % b);
    }

    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    // lcm snippet

    return a * (b / gcd(a, b));
}

fn find_steps(
    network: &HashMap<String, (String, String)>,
    start: String,
    end: &str,
    instructions: &mut Cycle<Chars<'_>>,
    count: usize
) -> usize {
    if start.ends_with(&end) {
        return count;
    }

    if let Some(node) = network.get(&start) {
        let next = match instructions.next() {
            Some('L') => &node.0,
            Some('R') => &node.1,
            _ => unreachable!()
        };

        return find_steps(network, next.to_string(), end, instructions, count + 1);
    }

    panic!("can't find node {start}")
}

fn path_steps() -> usize {
    if let Some(input) = fs::read_to_string("data/8.input").ok() {
        if let Some((instructions, network)) = input.split_once("\n\n") {
            let mut instructions = instructions.chars().cycle();
            let network = network.lines().filter_map(|node| {
                let node = node.replace("(", "").replace(")", "");

                if let Some((name, coords)) = node.split_once(" = ") {
                    if let Some((left, right)) = coords.split_once(", ") {
                        return Some((
                            name.to_string(),
                            (left.to_string(), right.to_string())
                        ));
                    }
                }

                None
            }).collect::<HashMap<String, (String, String)>>();

            return find_steps(&network, "AAA".to_string(), "ZZZ", &mut instructions, 0);

        }

        panic!("improper file format");
    }

    panic!("file not found")
}

fn ghost_steps() -> usize {
    if let Some(input) = fs::read_to_string("data/8.input").ok() {
        if let Some((instructions, network)) = input.split_once("\n\n") {
            let mut instruction_cycle = instructions.chars().cycle();
            let network = network.lines().filter_map(|node| {
                let node = node.replace("(", "").replace(")", "");

                if let Some((name, coords)) = node.split_once(" = ") {
                    if let Some((left, right)) = coords.split_once(", ") {
                        return Some((
                            name.to_string(),
                            (left.to_string(), right.to_string())
                        ));
                    }
                }

                None
            }).collect::<HashMap<String, (String, String)>>();

            // Find the lcm of each path, against the length of the instruction loop.
            return network
                .clone()
                .into_keys()
                .filter(|k| k.ends_with("A"))
                .map(|start| {
                    find_steps(&network, start.to_string(), "Z", &mut instruction_cycle, 0)
                })
                .fold(instructions.len(), |acc, x| lcm(acc, x));
        }

        panic!("improper file format");
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", path_steps());
    println!("part two: {}", ghost_steps());
}
