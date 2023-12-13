use std::fs;

struct Mirror {
    horizontal: Vec<Vec<bool>>,
    vertical: Vec<Vec<bool>>
}

fn get_mirrors(mirrors: String) -> Vec<Mirror> {
    mirrors.split("\n\n").map(|mirror| {
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
    }).collect::<Vec<Mirror>>()
}

fn search_for_seam(plane: &Vec<Vec<bool>>) -> Option<usize> {
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

fn off_by_one(left: &Vec<bool>, right: &Vec<bool>) -> bool {
    left.iter().zip(right).filter(|(l, r)| l != r).count() == 1
}

fn search_for_smudge(plane: &Vec<Vec<bool>>) -> Option<usize> {
    // Find the first idx which can reduce into a reflection, including a smudge.
    // A smudge is an off-by-one error, where a single value is wrong.

    for idx in 0..(plane.len() - 1) {
        let mut left = idx;
        let mut right = left + 1;
        let mut found = false;
        let mut smudge_used = false;

        loop {
            if plane[left] != plane[right] {
                if !smudge_used && off_by_one(&plane[left], &plane[right]) {
                    // Bypass the break if smudge hasn't been used.
                    smudge_used = true;
                } else {
                    break;
                }
            }

            if left == 0 || right == plane.len() - 1 {
                found = true;
                break;
            } else {
                left -= 1;
                right += 1;
            }
        }

        // Ensure we used a smudge.
        if found && smudge_used {
            return Some(idx);
        }
    }

    return None;
}

fn locate_mirrors() -> usize {
    if let Some(mirrors) = fs::read_to_string("data/13.input").ok() {
        let mirrors = get_mirrors(mirrors);

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

fn locate_smudged_mirrors() -> usize {
    // Instead of finding the reflective point of a mirror, find the first off-by-one error.

    if let Some(mirrors) = fs::read_to_string("data/13.input").ok() {
        let mirrors = get_mirrors(mirrors);

        return mirrors.iter().map(|mirror| {
            if let Some(seam) = search_for_smudge(&mirror.horizontal) {
                // 100 * idx for horizontal mirrors.
                return (seam + 1) * 100;
            } else {
                if let Some(seam) = search_for_smudge(&mirror.vertical) {
                    // 1 * idx for vertical mirrors.
                    return seam + 1;
                }
            }

            return 0;
        }).sum::<usize>()
    }

    panic!("file not found");

}

fn main() {
    println!("part one: {}", locate_mirrors());
    println!("part two: {}", locate_smudged_mirrors());
}
