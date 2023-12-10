use std::fs;

fn find_valid_start_points(map: &Vec<Vec<char>>, cursor: (usize, usize)) -> Vec<(usize, usize)> {
    // Get all valid starting directions.

    let mut agenda = vec![];

    // Left.
    if cursor.0 > 0 && ['-', 'L', 'F'].contains(&map[cursor.1][cursor.0 - 1]) {
        agenda.push((cursor.0 - 1, cursor.1));
    }

    // Top.
    if cursor.1 > 0 && ['|', '7', 'F'].contains(&map[cursor.1 - 1][cursor.0]) {
        agenda.push((cursor.0, cursor.1 - 1));
    }

    // Right.
    if cursor.0 <= map[0].len() && ['-', '7', 'J'].contains(&map[cursor.1][cursor.0 + 1]) {
        agenda.push((cursor.0 + 1, cursor.1));
    }

    // Bottom.
    if cursor.1 <= map.len() && ['|', 'L', 'J'].contains(&map[cursor.1 + 1][cursor.0]) {
        agenda.push((cursor.0, cursor.1 + 1));
    }

    return agenda;
}

fn get_loop(
    map: &Vec<Vec<char>>,
    mut cursor: (usize, usize)) -> Vec<(usize, usize)>
{
    // Keep track of everything we've seen so far.
    let mut visited: Vec<(usize, usize)> = vec![];

    // We keep finding the next element until visited causes us to break.
    loop {
        visited.push(cursor);

        let next = match map[cursor.1][cursor.0] {
            'S' => find_valid_start_points(map, cursor),
            other => {
                let possible_directions: Vec<(usize, usize)> = match other {
                    '-' => vec![(cursor.0 - 1, cursor.1), (cursor.0 + 1, cursor.1)],
                    '|' => vec![(cursor.0, cursor.1 - 1), (cursor.0, cursor.1 + 1)],
                    'J' => vec![(cursor.0, cursor.1 - 1), (cursor.0 - 1, cursor.1)],
                    'F' => vec![(cursor.0 + 1, cursor.1), (cursor.0, cursor.1 + 1)],
                    'L' => vec![(cursor.0, cursor.1 - 1), (cursor.0 + 1, cursor.1)],
                    '7' => vec![(cursor.0 - 1, cursor.1), (cursor.0, cursor.1 + 1)],
                    other => panic!("Bad input: {other}")
                };

                possible_directions
                    .into_iter()
                    .filter(|cur| !visited.contains(cur))
                    .collect()
            }
        };

        // If there is no next element, that means we've found 'S', because 'S' is the first part
        // of the loop inside visited. This means we can return the loop.
        if next.len() == 0 {
            return visited;
        } else {
            // If the loop hasn't been found, there should be exactly one element in next.
            cursor = *next.iter().nth(0).unwrap();
        }
    };
}

fn get_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    // Find 'S' in the map
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'S' {
                return Some((x, y))
            }
        }
    }

    return None;
}

fn loop_size() -> usize {
    // Find the biggest loop of pipes which contains 'S'.
    // We are given some helpful invariants:
    //   - S is only part of one loop,
    //   - There are only two pipes which point into S.
    // This means we can find S, follow one of the pipes which points into it, keep track of which
    // pipes we've seen, and exit once we find a pipe we've seen before (which will be S).

    if let Some(pipes) = fs::read_to_string("data/10.input").ok() {
        // Parse pipe map.
        let map: Vec<Vec<char>> = pipes
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Get starting point.
        if let Some(start) = get_start(&map) {
            // The problem actually wants the furthest we get from S, which is length / 2.
            return get_loop(&map, start).len() / 2;
        } else {
            panic!("No 'S' starting point found")
        }

    }

    panic!("file not found")
}

fn remove_s(map: &Vec<Vec<char>>, char: char, location: (usize, usize)) -> char {
    // Convert S into the character it acts as.
    // This is hairy manual logic.

    if char != 'S' {
        return char;
    }

    // Check whether S connects to each cardinal direction.
    let left  = location.0 > 0 && ['-', 'F', 'L'].contains(&map[location.0 - 1][location.1]);
    let top   = location.1 > 0 && ['|', 'F', '7'].contains(&map[location.0][location.1 - 1]);
    let right = location.0 <= map[0].len() && ['-', 'J', '7'].contains(&map[location.0 + 1][location.1]);
    let down  = location.1 <= map.len() && ['|', 'L', 'J'].contains(&map[location.0][location.1 + 1]);

    // Match the possible combinations.
    return match (left, top, right, down) {
        (_, true, _, true) => '|',
        (true, _, true, _) => '-',
        (_, _, true, true) => 'F',
        (true, true, _, _) => 'J',
        (true, _, _, true) => '7',
        (_, true, true, _) => 'L',

        // S is guaranteed by the input to contain exactly two connections.
        _ => panic!("Bad value for S")
    };
}

fn enclosed_tiles(map: &Vec<Vec<char>>, walls: Vec<(usize, usize)>) -> usize {
    // Count enclosed tiles.
    // This involves quite involved logic.
    //   - At any point in time we track whether we're inside or outside.
    //   - If a tile isn't part of walls, and we're inside, increment a counter.
    //   - If a tile is part of walls, modify the inside/outside tracker:
    //      Wall is 'S' => find out what 'S' emulates, then act accordingly.
    //      Wall is '|' => flip state.
    //      Wall is 'F'/'L' => flip state, but set the "unflipper" to '7'/'J' respectively.
    //      Wall is '7'/'J' => if is set as the flipper, flip state. Otherwise do nothing.
    //      Wall is '-' => do nothing.

    let mut count = 0;

    for (y, line) in map.iter().enumerate() {
        let mut enclosed = false;
        let mut unflip = '.';

        for (x, c) in line.iter().enumerate() {
            if walls.contains(&(x, y)) {
                match remove_s(map, *c, (x, y)) {
                    '|' => enclosed = !enclosed,
                    'F' => {
                        enclosed = !enclosed;
                        unflip = '7';
                    },
                    'L' => {
                        enclosed = !enclosed;
                        unflip = 'J';
                    },
                    v if v == unflip => {
                        enclosed = !enclosed;
                    },
                    _ => {}
                }
            } else {
                if enclosed { count += 1 }
            }
        }
    }

    return count;
}

fn nest_zone() -> usize {
    // Find how much empty space is in the loop.
    //
    // The hard part of this is not counting elements which aren't actually inside the loop.
    // For instance:
    //   F----7
    //   |F--7|
    //   ||..||
    //   |L7FJ|
    //   L-JL-J
    // Even though .. looks "inside" the loop, it's not "inside" because of the dual walls.

    if let Some(pipes) = fs::read_to_string("data/10.input").ok() {
        let map: Vec<Vec<char>> = pipes
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Get starting point.
        if let Some(start) = get_start(&map) {
            let enclosing_loop = get_loop(&map, start);

            return enclosed_tiles(&map, enclosing_loop);
        } else {
            panic!("No 'S' starting point found")
        }
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", loop_size());
    println!("part two: {}", nest_zone());
}
