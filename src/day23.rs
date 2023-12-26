use std::{fs, collections::HashMap};

enum Slopes {
    Insurmountable,
    Scalable
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

type Connections = HashMap<(usize, usize), Vec<((usize, usize), usize)>>;

impl Direction {
    fn invert(&self) -> Direction {
        return match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
        }
    }
}

fn build_map(input: String, slopes: Slopes) -> HashMap<(usize, usize), Vec<Direction>> {
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

                        match slopes {
                            Slopes::Insurmountable => {
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
                            },
                            Slopes::Scalable => {
                                if input_as_chars[y-1][x] != '#' && y != 1 {
                                    directions.push(Direction::North);
                                }

                                if input_as_chars[y][x-1] != '#' {
                                    directions.push(Direction::West);
                                }

                                if input_as_chars[y][x+1] != '#' {
                                    directions.push(Direction::East);
                                }

                                if input_as_chars[y+1][x] != '#' && y != input.lines().count() - 2 {
                                    directions.push(Direction::South);
                                }
                            }
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
    end: (usize, usize),
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
            .unwrap_or(0); // This is logically incorrect but if there is no path to success
                           // this option will be considered as “the worst choice”.

        return result;
    } else {
        panic!("unknown step taken: {}, {}", start.0, start.1);
    }
}

fn longest_path() -> usize {
    if let Some(input) = fs::read_to_string("data/23.input").ok() {
        let map = build_map(input.clone(), Slopes::Insurmountable);

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

fn compress_path(map: HashMap<(usize, usize), Vec<Direction>>) -> Connections {
    // Convert a list of directions into destinations with a cost.

    let mut connections: Connections = HashMap::new();

    map
        .iter()
        .filter(|(_start, directions)| directions.len() != 2)
        .for_each(|(start, directions)| {
            for direction in directions {
                let mut chain = vec![];
                let mut cursor = *start;
                let mut heading = direction;

                loop {
                    cursor = match heading {
                        Direction::North => (cursor.0, cursor.1 - 1),
                        Direction::West  => (cursor.0 - 1, cursor.1),
                        Direction::East  => (cursor.0 + 1, cursor.1),
                        Direction::South => (cursor.0, cursor.1 + 1),
                    };

                    chain.push(cursor);

                    if let Some(headings) = map.get(&cursor) {
                        if headings.len() != 2 {
                            connections
                                .entry(*start)
                                .and_modify(|cs| { cs.push((cursor, chain.len())); })
                                .or_insert(vec![(cursor, chain.len())]);

                            break;
                        } else {
                            if headings[0].invert() == *heading {
                                // Heading 0 would take us backwards.
                                heading = &headings[1];
                            } else {
                                // Heading 0 is fair enough.
                                heading = &headings[0];
                            }
                        }
                    }
                }
            }
        });

    return connections;
}

fn find_path_quickly(
    map: &Connections,
    start: (usize, usize),
    end: (usize, usize),
    mut been_through: Vec<(usize, usize)>,
    accum: usize
) -> usize {
    if start == end {
        return accum;
    }

    if let Some(paths) = map.get(&start) {
        been_through.push(start);

        return paths
            .iter()
            .filter(|(tail, _cost)| !been_through.contains(&tail))
            .map(|(tail, cost)| find_path_quickly(map, *tail, end, been_through.clone(), accum + cost))
            .max()
            .unwrap_or(0);
    } else {
        panic!("wandered astray during pathfinding")
    }
}

fn easy_path() -> usize {
    if let Some(input) = fs::read_to_string("data/23.input").ok() {
        let map = build_map(input.clone(), Slopes::Scalable);
        let map = compress_path(map);

        let start = (1, 1);
        let end = (
            input.lines().nth(0).unwrap().chars().count() - 2,
            input.lines().count() - 2
        );

        // + 1 for the end tile.
        // + 1 for (1, 1) being ignored.
        return find_path_quickly(&map, start, end, vec![], 0) + 2;
    } else {
        panic!("file not found")
    }
}

fn main() {
    println!("part one: {}", longest_path());
    println!("part two: {}", easy_path());
}

