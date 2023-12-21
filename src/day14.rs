use std::fs;

fn north_wall_load(
    walls: Vec<(usize, usize)>,
    rocks: Vec<(usize, usize)>,
    grid_height: usize,
    grid_width: usize
) -> usize {
    // Include rocks below the lowest wall of each row.
    let south_wall: Vec<_> = (0..grid_width).map(|idx| (idx, grid_height)).collect();

    walls
        .iter()
        .chain(south_wall.iter())
        .map(|wall| {
            // Get the first place you can put a rock.
            let to_place = walls
                .iter()
                .filter(|other| other.0 == wall.0 && other.1 < wall.1)
                .map(|other| other.1 + 1)
                .max()
                .unwrap_or(0);

            // Get the number of rocks between the two walls.
            let moved_rocks = rocks
                .iter()
                .filter(|rock|
                    rock.0 == wall.0 &&
                    to_place <= rock.1 &&
                    rock.1 < wall.1
                ).count();

            let mut accum = 0;

            // Count the positions from the top, add to accumulator.
            for idx in to_place..(to_place + moved_rocks) {
                accum += grid_height - idx;
            }

            return accum;
        }).sum()
}

fn total_load() -> usize {
    if let Some(grid) = fs::read_to_string("data/14.input").ok() {
        let mut round_rocks = vec![];
        let mut cube_rocks = vec![];

        grid.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '#' => cube_rocks.push((x, y)),
                    'O' => round_rocks.push((x, y)),
                    _ => {}
                }
            });
        });

        return north_wall_load(
            cube_rocks,
            round_rocks,
            grid.lines().count(),
            grid.lines().nth(0).unwrap().chars().count()
        );
    } else {
        panic!("file not found")
    }

}

fn spin_cycle(
    walls: &Vec<(usize, usize)>,
    rocks: &mut Vec<(usize, usize)>,
    grid_height: usize,
    grid_width: usize
) {
    // Mutate all the values of round_rocks up, then left, then down, then right.
    // Uses imperative control flow because it’s easy.

    rocks.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        } else {
            return a.1.cmp(&b.1);
        }
    });

    // Up.
    for idx in 0..rocks.len() {
        let mut new_y = walls
            .iter()
            .filter(|wall| wall.0 == rocks[idx].0 && wall.1 < rocks[idx].1)
            .map(|wall| wall.1 + 1)
            .max()
            .unwrap_or(0);

        while rocks[idx].1 != new_y && rocks.contains(&(rocks[idx].0, new_y)) { new_y += 1 }

        rocks[idx].1 = new_y;
    }

    rocks.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        } else {
            return a.1.cmp(&b.1);
        }
    });

    // Left.
    for idx in 0..rocks.len() {
        let mut new_x = walls
            .iter()
            .filter(|wall| wall.1 == rocks[idx].1 && wall.0 < rocks[idx].0)
            .map(|wall| wall.0 + 1)
            .max()
            .unwrap_or(0);

        while rocks[idx].0 != new_x && rocks.contains(&(new_x, rocks[idx].1)) { new_x += 1 }

        rocks[idx].0 = new_x;
    }

    rocks.sort_by(|a, b| {
        if a.1 == b.1 {
            return b.0.cmp(&a.0);
        } else {
            return b.1.cmp(&a.1);
        }
    });

    // Down.
    for idx in 0..rocks.len() {
        let mut new_y = walls
            .iter()
            .filter(|wall| wall.0 == rocks[idx].0 && wall.1 > rocks[idx].1)
            .map(|wall| wall.1 - 1)
            .min()
            .unwrap_or(grid_height - 1);

        while rocks[idx].1 != new_y && rocks.contains(&(rocks[idx].0, new_y)) { new_y -= 1 }

        rocks[idx].1 = new_y;
    }

    rocks.sort_by(|a, b| {
        if a.1 == b.1 {
            return b.0.cmp(&a.0);
        } else {
            return b.1.cmp(&a.1);
        }
    });

    // Right.
    for idx in 0..rocks.len() {
        let mut new_x = walls
            .iter()
            .filter(|wall| wall.1 == rocks[idx].1 && wall.0 > rocks[idx].0)
            .map(|wall| wall.0 - 1)
            .min()
            .unwrap_or(grid_width - 1);

        while rocks[idx].0 != new_x && rocks.contains(&(new_x, rocks[idx].1)) { new_x -= 1 }

        rocks[idx].0 = new_x;
    }
}

fn spin_cycles() -> usize {
    // Move all rounded rocks to the top, left, bottom, right. Repeat 1_000_000_000 times. At the
    // end, calculate the load on the north wall.
    //
    // To cut down on calculations, keep track of the rock position after each cycle. If it doesn’t
    // change, we can exit early.

    if let Some(grid) = fs::read_to_string("data/14.input").ok() {
        let mut walls = vec![];
        let mut rocks = vec![];

        grid.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '#' => walls.push((x, y)),
                    'O' => rocks.push((x, y)),
                    _ => {}
                }
            });
        });

        rocks.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            } else {
                return a.1.cmp(&b.1);
            }
        });

        let grid_height = grid.lines().count();
        let grid_width = grid.lines().nth(0).unwrap().chars().count();

        let mut cache = vec![rocks.clone()];

        for idx in 1..=1_000_000_000 {
            if idx % 100_000 == 0 {
                println!("spin {idx}");
            }

            spin_cycle(&walls, &mut rocks, grid_height, grid_width);

            if cache.contains(&rocks) {
                // Find the period between the previous occurrence of rocks and now.
                //   b  b  b  b  b  r  b  b  b  r  b  b  b  r  b  b
                //                  |..........=|
                //                              -> period of 4.
                //
                // This means we can start at the first occurrence and modulo the period -> x. The value at
                // the end should be the same as the first occurrence + x.

                let first = cache.iter().position(|r| **r == rocks).unwrap();

                let period = idx - first;

                let adjusted = (1_000_000_000 - first) % period;

                rocks = cache[first + adjusted].clone();

                break;
            } else {
                cache.push(rocks.clone());
            }
        }

        return rocks.iter().map(|rock| grid_height - rock.1).sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", total_load());
    println!("part two: {}", spin_cycles());
    // NOTE: part two is completely screwed, it doesn’t even move the rocks right.
}
