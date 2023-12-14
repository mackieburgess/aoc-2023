use std::fs;

fn north_wall_load() -> usize {
    // If all the round rocks (O) go north until they hit a cube rock (#), how much pressure does
    // this put on the north wall (grid height - idx).

    if let Some(grid) = fs::read_to_string("data/14.input").ok() {
        let grid_height = grid.lines().count();

        let mut round_rocks: Vec<(usize, usize)> = vec![];

        // Include 0 for the north wall.
        let mut cube_rocks: Vec<Vec<usize>> = vec![vec![0]; grid.lines().nth(0).unwrap().len()];

        // Simultaneously build the map of round rocks and cube rocks.
        grid.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '#' => cube_rocks[x].push(y + 1),
                    'O' => round_rocks.push((x, y)),
                    _ => {}
                }
            });
        });

        // Include the south wall.
        cube_rocks.iter_mut().for_each(|column| {
            column.push(grid_height);
        });

        return cube_rocks.iter().enumerate().map(|(x, column)| {
            column.windows(2).map(|pair| {
                let mut height = grid_height - pair[0];

                round_rocks
                    .iter()
                    .filter(|rock| rock.0 == x && pair[0] <= rock.1 && rock.1 < pair[1])
                    .map(|_rock| {
                        height -= 1;
                        return height + 1;
                    }).sum::<usize>()
            }).sum::<usize>()
        }).sum::<usize>();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", north_wall_load());
}
