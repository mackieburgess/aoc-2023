use std::{fs, collections::HashMap};

enum Direction {
    North,
    East,
    South,
    West
}

fn build_map(input: String) -> HashMap<(usize, usize), Vec<Direction>> {
    let input_as_chars = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut nodes: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();

    // Find the directions you can go from each path.
    input_as_chars
        .iter()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .iter()
                .enumerate()
                .for_each(|(x, c)| {
                    if y != input_as_chars.len() - 1 && y != 0 && *c != '#' {
                        let mut directions = vec![];

                        if ['.', '^'].contains(&input_as_chars[y-1][x]) && (x, y) != (1, 1) {
                            directions.push(Direction::North);
                        }

                        if ['.', '<'].contains(&input_as_chars[y][x-1]) {
                            directions.push(Direction::West);
                        }

                        if ['.', '>'].contains(&input_as_chars[y][x+1]) {
                            directions.push(Direction::East);
                        }

                        if ['.', 'v'].contains(&input_as_chars[y+1][x]) {
                            directions.push(Direction::South);
                        }

                        nodes.insert((x, y), directions);
                    }
                });
        });



    return nodes;
}

fn find_longest_path(
    map: &HashMap<(usize, usize), Vec<Direction>>,
    mut found: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize)
) -> usize {
    found.push(start);

    if start == end {
        return found.len();
    }

    if let Some(directions) = map.get(&start) {
        let result = directions
            .iter()
            .filter(|direction| {
                match direction {
                    Direction::North => !found.contains(&(start.0, start.1 - 1)),
                    Direction::West  => !found.contains(&(start.0 - 1, start.1)),
                    Direction::East  => !found.contains(&(start.0 + 1, start.1)),
                    Direction::South => !found.contains(&(start.0, start.1 + 1)),
                }
            })
            .map(|direction| {
                match direction {
                    Direction::North => find_longest_path(map, found.clone(), (start.0, start.1 - 1), end),
                    Direction::West  => find_longest_path(map, found.clone(), (start.0 - 1, start.1), end),
                    Direction::East  => find_longest_path(map, found.clone(), (start.0 + 1, start.1), end),
                    Direction::South => find_longest_path(map, found.clone(), (start.0, start.1 + 1), end),
                }
            })
            .max() // Longest option.
            .unwrap();

        return result;
    } else {
        panic!("unknown step taken: {}, {}", start.0, start.1);
    }
}

fn longest_path() -> usize {
    if let Some(input) = fs::read_to_string("data/23.input").ok() {
        let map = build_map(input.clone());

        let start = (1, 1);
        let end = (
            input.lines().nth(0).unwrap().chars().count() - 2,
            input.lines().count() - 2
        );

        return find_longest_path(&map, vec![], start, end) + 1; // + 1 for the end tile.
    } else {
        panic!("file not found")
    }
}

fn main() {
    println!("part one: {}", longest_path());
}

