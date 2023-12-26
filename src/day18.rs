use std::fs;

enum Mode {
    Standard,
    Hex
}

enum Turn {
    Clockwise,
    CounterClockwise
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy)]
struct Instruction {
    // Hilarious to me that I’m storing these in a vector.
    direction: Direction,
    magnitude: isize
}

impl Direction {
    fn turn(&self, other: &Direction) -> Turn {
        return match (self, other) {
            (Direction::Up, Direction::Right) |
            (Direction::Right, Direction::Down) |
            (Direction::Down, Direction::Left) |
            (Direction::Left, Direction::Up) => Turn::Clockwise,
            (Direction::Up, Direction::Left) |
            (Direction::Right, Direction::Up) |
            (Direction::Down, Direction::Right) |
            (Direction::Left, Direction::Down) => Turn::CounterClockwise,
            _ => panic!()
        }
    }
}

fn read_instructions(instructions: String, mode: Mode) -> Vec<Instruction> {
    instructions
        .lines()
        .filter_map(|line| {
            if let Some((direction, rest)) = line.split_once(" ") {
                if let Some((magnitude, color)) = rest.split_once(" ") {
                    match mode {
                        Mode::Standard => {
                            let direction = match direction {
                                "U" => Direction::Up,
                                "R" => Direction::Right,
                                "D" => Direction::Down,
                                "L" => Direction::Left,
                                _ => panic!("bad direction")
                            };

                            if let Some(magnitude) = magnitude.parse::<isize>().ok() {
                                return Some(Instruction { direction, magnitude });
                            }
                        },
                        Mode::Hex => {
                            let mut size_code = color.replace("(#", "").replace(")", "").to_uppercase();
                            let dir_code = size_code.split_off(5);

                            let direction = match dir_code.as_str() {
                                "0" => Direction::Right,
                                "1" => Direction::Down,
                                "2" => Direction::Left,
                                "3" => Direction::Up,
                                _ => unreachable!()
                            };

                            if let Some(magnitude) = isize::from_str_radix(&size_code, 16).ok() {
                                return Some(Instruction { direction, magnitude });
                            }
                        }
                    }
                }
            }

            return None;
        }).collect()
}

fn lava_bowl_area(mode: Mode) -> isize {
    if let Some(instructions) = fs::read_to_string("data/18.input").ok() {
        let mut instructions = read_instructions(instructions, mode);

        // Copy the first movement over to the end, so that we can calculate the corners.
        instructions.push(instructions[0].clone());
        let instructions = instructions;

        let mut cursor: (isize, isize) = (0, 0);
        let mut points = vec![cursor];

        // The last direction, accounting for the repeat of the first element.
        let mut initial_direction = instructions[instructions.len() - 2].direction;

        for instruction_pair in instructions.windows(2) {

            // Increment handles drawing a line around the lava bowl wall, instead of “on it”.
            let increment: isize = match (
                initial_direction.turn(&instruction_pair[0].direction),
                instruction_pair[0].direction.turn(&instruction_pair[1].direction)
            ) {
                (Turn::Clockwise, Turn::Clockwise) => 1,
                (Turn::CounterClockwise, Turn::CounterClockwise) => -1,
                _ => 0
            };

            match instruction_pair[0].direction {
                Direction::Up    => cursor.1 -= instruction_pair[0].magnitude + increment,
                Direction::Right => cursor.0 += instruction_pair[0].magnitude + increment,
                Direction::Down  => cursor.1 += instruction_pair[0].magnitude + increment,
                Direction::Left  => cursor.0 -= instruction_pair[0].magnitude + increment
            };

            points.push(cursor);

            initial_direction = instruction_pair[0].direction;
        }

        // Shoelace formula: a maths shorthand for calculating the area of a polygon.
        points
            .windows(2)
            .fold(0, |acc, pair| {
                return acc + (pair[0].0 * pair[1].1) - (pair[0].1 * pair[1].0);
            }) / 2

    } else {
        panic!("file not found")
    }
}

fn main() {
    println!("part one: {}", lava_bowl_area(Mode::Standard));
    println!("part two: {}", lava_bowl_area(Mode::Hex));
}
