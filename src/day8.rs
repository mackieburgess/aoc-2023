use std::fs;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;

fn find_steps(
    network: &HashMap<String, (String, String)>,
    start: String,
    end: String,
    instructions: &mut Cycle<Chars<'_>>,
    count: usize
) -> usize {
    if start == end {
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

            return find_steps(&network, "AAA".to_string(), "ZZZ".to_string(), &mut instructions, 0);

        }

        panic!("improper file format");
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", path_steps());
}
