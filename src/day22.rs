use std::fs;

#[derive(Clone)]
struct Brick {
    code: usize,        // Line number, used for identification.
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
    bases: Vec<usize>
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        // Checks whether – from the top down – two bricks overlap.

        // Big chunk of lazy comparison code, modified from AoC 2022 Day 4.
        if (self.x.0 >= other.x.0 && self.x.0 <= other.x.1
            || self.x.1 >= other.x.0 && self.x.1 <= other.x.1
            || other.x.0 >= self.x.0 && other.x.0 <= self.x.1
            || other.x.1 >= self.x.0 && other.x.1 <= self.x.1) &&
           (self.y.0 >= other.y.0 && self.y.0 <= other.y.1
            || self.y.1 >= other.y.0 && self.y.1 <= other.y.1
            || other.y.0 >= self.y.0 && other.y.0 <= self.y.1
            || other.y.1 >= self.y.0 && other.y.1 <= self.y.1)
        {
            return true;
        } else {
            return false;
        }
    }
}

fn build_bricks(brick_data: String) -> Vec<Brick> {
    let brick_data = brick_data.replace("~", ",");

    let mut bricks = brick_data
        .lines()
        .enumerate()
        .filter_map(|(code, data)| {
            let coord_slice = data
                .split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>();

            if coord_slice.len() == 6 {
                return Some(Brick {
                    code,
                    x: (coord_slice[0], coord_slice[3]),
                    y: (coord_slice[1], coord_slice[4]),
                    z: (coord_slice[2], coord_slice[5]),
                    bases: vec![]
                });
            } else {
                return None;
            }
        }).collect::<Vec<Brick>>();

    // Sort bricks by height.
    bricks.sort_by(|a, b| a.z.0.cmp(&b.z.0));

    // Apply downwards gravity.
    for i in 0..bricks.len() {
        let new_z0 = bricks
            .iter()
            .filter(|brick| bricks[i].overlaps(brick) && brick.z.1 < bricks[i].z.0)
            .map(|brick| brick.z.1 + 1)
            .max()
            .unwrap_or(1);

        let new_z1 = (bricks[i].z.1 - bricks[i].z.0) + new_z0;

        bricks[i].z.0 = new_z0;
        bricks[i].z.1 = new_z1;

    }

    // Figure out the base for each brick.

    for i in 0..bricks.len() {
        let self_ = bricks[i].clone();

        bricks
            .clone()
            .iter()
            .filter(|brick| self_.overlaps(brick) && brick.z.1 + 1 == self_.z.0)
            .map(|brick| brick.code)
            .for_each(|code| bricks[i].bases.push(code));
    }

    return bricks;
}

fn superfluous_bricks() -> usize {
    // Calculate the total number of bricks which aren’t the sole supporter
    // of any other brick.

    if let Some(bricks) = fs::read_to_string("data/22.input").ok() {
        let bricks = build_bricks(bricks);

        return bricks
            .iter()
            .filter(|brick| bricks
                .iter()
                .filter(|other| other.bases == vec![brick.code])
                .count() == 0)
            .count();
    } else {
        panic!("file not found")
    }
}

fn main() {
    println!("part one: {}", superfluous_bricks());
}
