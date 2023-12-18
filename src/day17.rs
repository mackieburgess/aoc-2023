use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Heading {
    Up,
    Right,
    Down,
    Left
}

struct Path {
    x: usize,
    y: usize,
    heading: Heading,
    streak: usize,
    cost: usize
}

fn build_map(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as usize)
            .collect()
        ).collect()
}

fn traversable(
    map: &Vec<Vec<usize>>,
    filed: &HashSet<(usize, usize, Heading, usize)>,
    path: &Path,
    heading: Heading,
    range: (usize, usize)
) -> bool {
    // Determine whether we can go in a given direction from our current tile.

    // No backtracking rule.
    // Took 3 hours for me to realise I need this...
    if match heading {
        Heading::Up => path.heading == Heading::Down,
        Heading::Right => path.heading == Heading::Left,
        Heading::Down => path.heading == Heading::Up,
        Heading::Left => path.heading == Heading::Right,
    } { return false; }

    // Not hitting the edge of the board.
    if match heading {
        Heading::Up => path.y != 0,
        Heading::Right => path.x != map[path.y].len() - 1,
        Heading::Down => path.y != map.len() - 1,
        Heading::Left => path.x != 0,
    } {
        // Calculating the new position.
        let new_value = match heading {
            Heading::Up    => (path.x, path.y - 1),
            Heading::Right => (path.x + 1, path.y),
            Heading::Down  => (path.x, path.y + 1),
            Heading::Left  => (path.x - 1, path.y),
        };

        let new_streak = match heading == path.heading {
            true => path.streak + 1,
            false => 1
        };

        // Checking we haven’t done the exact same rule before.
        if !filed.contains(&(new_value.0, new_value.1, heading.clone(), new_streak)) {
            if heading == path.heading {
                // Checking we haven’t walked too far in a straight line.
                return new_streak <= range.1;
            } else {
                // For crucibles with lower bounds, we need to make sure we’ve gone far enough.
                return range.0 <= path.streak;
            }

        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn warmest_path(range: (usize, usize)) -> usize {
    // For part one we find the lowest cost path through the maze, without
    // taking more than three steps in the same direction.
    //
    // For part two, we make this generic for sticking between a “turning range”.
    // This means we have to have gone a certain distance before we can turn, and
    // we have to turn before we hit a certain number of steps in a row.

    if let Some(input) = fs::read_to_string("data/17.input").ok() {
        let map = build_map(input);

        let mut filed: HashSet<(usize, usize, Heading, usize)> = HashSet::new();

        let mut paths: Vec<Path> = vec![];

        // For the generic solution, we need to ensure our start headings are good.
        // If we only started with one direction, we wouldn’t be able to turn immediately when a
        // lower bound is set.
        paths.push(Path {
            x: 0,
            y: 0,
            heading: Heading::Right,
            streak: 0,
            cost: 0
        });

        paths.push(Path {
            x: 0,
            y: 0,
            heading: Heading::Down,
            streak: 0,
            cost: 0
        });

        filed.insert((0, 0, Heading::Right, 0));
        filed.insert((0, 0, Heading::Down, 0));

        while let Some(path) = paths.pop() {
            // Check our exit condition. We’re on the exit tile and our streak is good.
            if path.y == map.len() - 1 &&
                path.x == map[path.y].len() - 1 &&
                path.streak >= range.0
            {
                return path.cost;
            }

            // Left.
            if traversable(&map, &filed, &path, Heading::Left, range) {
                let new_streak = match path.heading == Heading::Left {
                    true => path.streak + 1,
                    false => 1
                };

                filed.insert((path.x - 1, path.y, Heading::Left, new_streak));

                paths.push(Path {
                    x: path.x - 1,
                    y: path.y,
                    heading: Heading::Left,
                    streak: new_streak,
                    cost: path.cost + map[path.y][path.x - 1]
                });
            }

            // Right.
            if traversable(&map, &filed, &path, Heading::Right, range) {
                let new_streak = match path.heading == Heading::Right {
                    true => path.streak + 1,
                    false => 1
                };

                filed.insert((path.x + 1, path.y, Heading::Right, new_streak));

                paths.push(Path {
                    x: path.x + 1,
                    y: path.y,
                    heading: Heading::Right,
                    streak: new_streak,
                    cost: path.cost + map[path.y][path.x + 1]
                });
            }

            // Up.
            if traversable(&map, &filed, &path, Heading::Up, range) {
                let new_streak = match path.heading == Heading::Up {
                    true => path.streak + 1,
                    false => 1
                };

                filed.insert((path.x, path.y - 1, Heading::Up, new_streak));

                paths.push(Path {
                    x: path.x,
                    y: path.y - 1,
                    heading: Heading::Up,
                    streak: new_streak,
                    cost: path.cost + map[path.y - 1][path.x]
                });
            }

            // Down.
            if traversable(&map, &filed, &path, Heading::Down, range) {
                let new_streak = match path.heading == Heading::Down {
                    true => path.streak + 1,
                    false => 1
                };

                filed.insert((path.x, path.y + 1, Heading::Down, new_streak));

                paths.push(Path {
                    x: path.x,
                    y: path.y + 1,
                    heading: Heading::Down,
                    streak: new_streak,
                    cost: path.cost + map[path.y + 1][path.x]
                });
            }

            // Sort to get the current “best looking” tile.
            paths.sort_by(|a, b| b.cost.cmp(&a.cost));
        }

        // If we hit here, there is something wrong with our input.
        unreachable!();
    } else {
        panic!("file not found")
    }

}

fn main() {
    println!("part one: {}", warmest_path((1, 3)));
    println!("part two: {}", warmest_path((4, 10)));
}
