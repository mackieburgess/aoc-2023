use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Heading {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
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
    heading: Heading
) -> bool {
    // No backtracking rule.
    // This took 3 hours to figure out.
    if match heading {
        Heading::Up => path.heading == Heading::Down,
        Heading::Right => path.heading == Heading::Left,
        Heading::Down => path.heading == Heading::Up,
        Heading::Left => path.heading == Heading::Right,
    } { return false; }

    if match heading {
        Heading::Up => path.y != 0,
        Heading::Right => path.x != map[path.y].len() - 1,
        Heading::Down => path.y != map.len() - 1,
        Heading::Left => path.x != 0,
    } {
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

        if !filed.contains(&(new_value.0, new_value.1, heading, new_streak)) {
            return new_streak <= 3;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn warmest_path() -> usize {
    // Find the lowest cost path through the maze, without taking more than
    // three steps in the same direction.

    if let Some(input) = fs::read_to_string("data/17.input").ok() {
        let map = build_map(input);

        let mut filed: HashSet<(usize, usize, Heading, usize)> = HashSet::new();

        let mut paths: Vec<Path> = vec![];

        // Our starting point.
        paths.push(Path {
            x: 0,
            y: 0,
            heading: Heading::Left,
            streak: 0,
            cost: 0
        });

        filed.insert((0, 0, Heading::Left, 0));

        while let Some(path) = paths.pop() {

            if path.y == map.len() - 1 && path.x == map[path.y].len() - 1 {
                return path.cost;
            }

            // Left.
            if traversable(&map, &filed, &path, Heading::Left) {
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
            if traversable(&map, &filed, &path, Heading::Right) {
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
            if traversable(&map, &filed, &path, Heading::Up) {
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
            if traversable(&map, &filed, &path, Heading::Down) {
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

            paths.sort_by(|a, b| b.cost.cmp(&a.cost));
        }

        panic!("no path found");
    } else {
        panic!("file not found")
    }

}

fn main() {
    println!("part one: {}", warmest_path());
}
