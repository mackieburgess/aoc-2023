use std::fs;

fn find_valid_start_point(map: &Vec<Vec<char>>, cursor: (usize, usize)) -> (usize, usize) {
    // Get a valid starting direction.

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

    // Unwrap: it is guaranteed by the input that two pipes connect to S.
    return agenda.into_iter().nth(0).unwrap();
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
            'S' => vec![find_valid_start_point(map, cursor)],
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

                // 
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

fn loop_size() -> usize {
    // Find the biggest loop of pipes which contains 'S'.
    // We are given some helpful invariants:
    //   - S is only part of one loop,
    //   - There are only two pipes which point into S.
    // This means we can find S, follow one of the pipes which points into it, keep track of which
    // pipes we've seen, and exit once we find a pipe we've seen before (which will be S).

    if let Some(pipes) = fs::read_to_string("data/10.input").ok() {
        // Parse pipe map.
        let pipe_map: Vec<Vec<char>> = pipes
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut start = (0, 0);

        // Find the starting point.
        pipe_map
            .iter()
            .enumerate()
            .for_each(|(y, line)|
                line
                    .iter()
                    .enumerate()
                    .for_each(|(x, c)| {
                        if c == &'S' { start = (x, y) }
                    })
            );

        // The problem actually wants the furthest we get from S, which is length / 2.
        return get_loop(&pipe_map, start).len() / 2;
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", loop_size());
}
