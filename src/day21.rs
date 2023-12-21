use std::fs;
use std::collections::{HashMap, VecDeque};

fn simulate_steps(
    iterations: usize,
    walls: Vec<(usize, usize)>,
    start: (usize, usize),
    maxes: (usize, usize)
) -> HashMap<(usize, usize), usize> {
    let mut found = HashMap::new();
    found.insert(start, 0);

    let mut agenda = VecDeque::from([(start, 0)]);

    while let Some((cursor, step_count)) = agenda.pop_front() {
        if step_count < iterations {
            // Up.
            if 0 < cursor.1
                && !walls.contains(&(cursor.0, cursor.1 - 1))
                && !found.contains_key(&(cursor.0, cursor.1 - 1))
            {
                found.insert((cursor.0, cursor.1 - 1), step_count + 1);
                agenda.push_back(((cursor.0, cursor.1 - 1), step_count + 1));
            }

            // Right.
            if cursor.0 < maxes.0 - 1
                && !walls.contains(&(cursor.0 + 1, cursor.1))
                && !found.contains_key(&(cursor.0 + 1, cursor.1))
            {
                found.insert((cursor.0 + 1, cursor.1), step_count + 1);
                agenda.push_back(((cursor.0 + 1, cursor.1), step_count + 1));
            }

            // Down.
            if cursor.1 < maxes.1 - 1
                && !walls.contains(&(cursor.0, cursor.1 + 1))
                && !found.contains_key(&(cursor.0, cursor.1 + 1))
            {
                found.insert((cursor.0, cursor.1 + 1), step_count + 1);
                agenda.push_back(((cursor.0, cursor.1 + 1), step_count + 1));
            }

            // Left.
            if 0 < cursor.0
                && !walls.contains(&(cursor.0 - 1, cursor.1))
                && !found.contains_key(&(cursor.0 - 1, cursor.1))
            {
                found.insert((cursor.0 - 1, cursor.1), step_count + 1);
                agenda.push_back(((cursor.0 - 1, cursor.1), step_count + 1));
            }
        }
    }

    return found;
}

fn plots_reachable() -> usize {
    if let Some(map) = fs::read_to_string("data/21.input").ok() {
        let mut walls = vec![];
        let mut cursor = None;

        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => walls.push((x, y)),
                    'S' => cursor = Some((x, y)),
                    _ => {}
                }
            }
        }

        let maxes = (
            map.lines().nth(0).unwrap().chars().count(),
            map.lines().count()
        );

        if let Some(cursor) = cursor {
            return simulate_steps(64, walls, cursor, maxes)
                .iter()
                .filter(|(_stead, steps_taken)| *steps_taken % 2 == 0)
                .count();
        }
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", plots_reachable());
}
