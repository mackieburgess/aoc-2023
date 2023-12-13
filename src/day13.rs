use std::fs;

struct Mirror {
    horizontal: Vec<Vec<bool>>,
    vertical: Vec<Vec<bool>>
}

fn search_for_seam<T: PartialEq>(plane: &Vec<T>) -> Option<usize> {
    // Find the first idx which can reduce into a reflection.

    for idx in 0..(plane.len() - 1) {
        let mut left = idx;
        let mut right = left + 1;
        let mut found = false;

        loop {
            if plane[left] != plane[right] {
                break;
            } else {
                if left == 0 || right == plane.len() - 1 {
                    found = true;
                    break;
                } else {
                    left -= 1;
                    right += 1;
                }
            }
        }

        if found {
            return Some(idx);
        }
    }

    return None;
}

fn locate_mirror() -> usize {
    if let Some(mirrors) = fs::read_to_string("data/13.input").ok() {
        let mirrors = mirrors.split("\n\n").map(|mirror| {
            // Generate the horizontal mirror, alongside its transposition.

            let horizontal = mirror.lines().map(|line| {
                return line.chars().map(|c| c == '#').collect();
            }).collect::<Vec<Vec<bool>>>();

            let mut vertical = vec![];
            for x in 0..horizontal[0].len() {
                vertical.push(vec![]);
                for y in 0..horizontal.len() {
                    vertical[x].push(horizontal[y][x]);
                }
            }

            return Mirror { horizontal, vertical };
        }).collect::<Vec<Mirror>>();

        return mirrors.iter().map(|mirror| {
            if let Some(seam) = search_for_seam(&mirror.horizontal) {
                // 100 * idx for horizontal mirrors.
                return (seam + 1) * 100;
            } else {
                if let Some(seam) = search_for_seam(&mirror.vertical) {
                    // 1 * idx for vertical mirrors.
                    return seam + 1;
                }
            }

            return 0;
        }).sum::<usize>()
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", locate_mirror());
}
