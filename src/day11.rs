use std::fs;

fn manhattan(i: &(usize, usize), j: &(usize, usize)) -> usize {
    return i.0.abs_diff(j.0) + i.1.abs_diff(j.1);
}

fn galaxy_distances(expansion: usize) -> usize {
    // Double the size of any empty rows/columns,
    // then get the sum of all distances between galaxys.

    if let Some(map) = fs::read_to_string("data/11.input").ok() {
        let mut galaxies = vec![];

        let mut empty_rows = vec![];
        let mut empty_cols = vec![];

        // Get galaxies an empty zones.
        for (y, line) in map.lines().enumerate() {
            if line.chars().filter(|c| *c == '#').count() == 0 {
                empty_rows.push(y);
            }

            for (x, c) in line.chars().enumerate() {
                if y == 0 && map.lines().filter(|l| l.chars().nth(x) == Some('#')).count() == 0 {
                    empty_cols.push(x)
                }

                if c == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        // Expand rows.
        for row in empty_rows.iter().rev() {
            for mut galaxy in galaxies.iter_mut() {
                if galaxy.1 > *row { galaxy.1 += expansion }
            }
        }

        // Expand columns.
        for col in empty_cols.iter().rev() {
            for mut galaxy in galaxies.iter_mut() {
                if galaxy.0 > *col { galaxy.0 += expansion }
            }
        }

        // debug_galaxies(&galaxies);

        let mut distance_between_galaxies = 0;

        for i in galaxies.iter() {
            for j in galaxies.iter() {
                if i.0 < j.0 || (i.0 == j.0 && i.1 < j.1) {
                    distance_between_galaxies += manhattan(i, j);
                }
            }
        }

        return distance_between_galaxies;
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", galaxy_distances(1));
    println!("part two: {}", galaxy_distances(999_999));
}
