use std::fs;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct Instruction {
    // Hilarious to me that I’m storing these in a vector.
    direction: Direction,
    magnitude: usize
}

fn read_instructions(instructions: String) -> Vec<Instruction> {
    instructions
        .lines()
        .filter_map(|line| {
            if let Some((direction, rest)) = line.split_once(" ") {
                if let Some((magnitude, _color)) = rest.split_once(" ") {
                    let direction = match direction {
                        "U" => Direction::Up,
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        _ => panic!("bad direction")
                    };

                    if let Some(magnitude) = magnitude.parse::<usize>().ok() {
                        return Some(Instruction { direction, magnitude });
                    }
                }
            }

            return None;
        }).collect()
}

fn lava_bowl_area() -> usize {
    // Use a turtle to draw a lava bowl, then calculate the flood fill of the
    // shape it makes.

    if let Some(instructions) = fs::read_to_string("data/18.input").ok() {
        let instructions = read_instructions(instructions);

        let mut cursor = (0, 0);

        let mut found = HashMap::new();

        for (idx, instruction) in instructions.iter().enumerate() {
            let movement = match instruction.direction {
                Direction::Up   | Direction::Down  => '|',
                Direction::Left | Direction::Right => '-'
            };

            for _ in 0..(instruction.magnitude - 1) {
                match instruction.direction {
                    Direction::Up    => cursor.1 -= 1,
                    Direction::Right => cursor.0 += 1,
                    Direction::Down  => cursor.1 += 1,
                    Direction::Left  => cursor.0 -= 1
                }

                found.insert(cursor, movement);
            }

            // Handle the turning.

            match instruction.direction {
                Direction::Up    => cursor.1 -= 1,
                Direction::Right => cursor.0 += 1,
                Direction::Down  => cursor.1 += 1,
                Direction::Left  => cursor.0 -= 1
            }

            let next_direction = match idx < instructions.len() - 1 {
                true => instructions[idx + 1].direction,
                false => instructions[0].direction
            };

            // Parse corners into our day 10 solution.
            let final_movement = match (instruction.direction, next_direction) {
                (Direction::Up, Direction::Left)    | (Direction::Right, Direction::Down) => '7',
                (Direction::Up, Direction::Right)   | (Direction::Left, Direction::Down)  => 'F',
                (Direction::Down, Direction::Left)  | (Direction::Right, Direction::Up)   => 'J',
                (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up)    => 'L',
                _ => unreachable!()
            };

            found.insert(cursor, final_movement);
        }

        let (min_x, min_y) = (
            found.keys().map(|f| f.0).min().unwrap(),
            found.keys().map(|f| f.1).min().unwrap(),
        );

        let (max_x, max_y) = (
            found.keys().map(|f| f.0).max().unwrap(),
            found.keys().map(|f| f.1).max().unwrap(),
        );

        let mut fill = 0;

        // Reuses my day 10 solution for counting enclosed tiles.
        for y in min_y..=max_y {
            let mut enclosed = false;
            let mut unflip = '.';

            for x in min_x..=max_x {
                if let Some(wall_piece) = found.get(&(x, y)) {
                    match wall_piece {
                        '|' => enclosed = !enclosed,
                        'F' => {
                            enclosed = !enclosed;
                            unflip = '7';
                        },
                        'L' => {
                            enclosed = !enclosed;
                            unflip = 'J';
                        },
                        v if *v == unflip => {
                            enclosed = !enclosed;
                        },
                        _ => {}
                    }
                } else {
                    if enclosed { fill += 1 }
                }
            }
        }

        return found.len() + fill;
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", lava_bowl_area());
}
