use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Heading {
    Up,
    Right,
    Down,
    Left
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cursor {
    heading: Heading,
    x: usize,
    y: usize
}

impl Cursor {
    fn towards_edge(&self, board: &Vec<Vec<char>>) -> bool {
        match self.heading {
            Heading::Up => self.y == 0,
            Heading::Left => self.x == 0,
            Heading::Down => self.y == board.len() - 1,
            Heading::Right => self.x == board[0].len() - 1
        }
    }

    fn progress(&mut self) {
        match self.heading {
            Heading::Up => self.y -= 1,
            Heading::Down => self.y += 1,
            Heading::Left => self.x -= 1,
            Heading::Right => self.x += 1
        }
    }

    fn mirror_bounce(&mut self, c: char) {
        match c {
            '/' => match self.heading {
                Heading::Up => self.heading = Heading::Right,
                Heading::Down => self.heading = Heading::Left,
                Heading::Left => self.heading = Heading::Down,
                Heading::Right => self.heading = Heading::Up,
            },
            '\\' => match self.heading {
                Heading::Up => self.heading = Heading::Left,
                Heading::Down => self.heading = Heading::Right,
                Heading::Left => self.heading = Heading::Up,
                Heading::Right => self.heading = Heading::Down
            },
            _ => {}
        }
    }
}

fn run_energisation(
    found: &mut HashSet<Cursor>,
    mut cursor: Cursor,
    board: &Vec<Vec<char>>)
{
    // Insert the starting value.
    found.insert(cursor.clone());

    loop {
        // Use a mirror, if you’re on one.
        cursor.mirror_bounce(board[cursor.y][cursor.x]);

        // Check to see if you’re about to crash into a wall.
        if cursor.towards_edge(board) {
            break;
        }

        // Move one step.
        cursor.progress();

        // If you’re on a splitting mirror, split.
        match board[cursor.y][cursor.x] {
            '-' if cursor.heading == Heading::Up || cursor.heading == Heading::Down => {
                found.insert(cursor.clone());

                if cursor.x != 0 {
                    run_energisation(
                        found,
                        Cursor { heading: Heading::Left, x: cursor.x, y: cursor.y },
                        board
                    );
                }

                if cursor.x != board[cursor.y].len() - 1 {
                    run_energisation(
                        found,
                        Cursor { heading: Heading::Right, x: cursor.x, y: cursor.y },
                        board
                    );
                }

                break;
            },
            '|' if cursor.heading == Heading::Left || cursor.heading == Heading::Right => {
                found.insert(cursor.clone());

                if cursor.y != 0 {
                    run_energisation(
                        found,
                        Cursor { heading: Heading::Up, x: cursor.x, y: cursor.y },
                        board
                    );
                }

                if cursor.y != board.len() - 1 {
                    run_energisation(
                        found,
                        Cursor { heading: Heading::Down, x: cursor.x, y: cursor.y },
                        board
                    );
                }

                break;
            },
            _ => {}
        }

        // Insert your current location if you’re not on a splitting mirror.
        if !found.insert(cursor.clone()) {
            break;
        }
    }
}

fn energised_tiles() -> usize {
    // Create a trail going around a mirror maze, count the number of tiles touched.

    if let Some(input) = fs::read_to_string("data/16.input").ok() {
        let cursor = Cursor { heading: Heading::Right, x: 0, y: 0 };
        let board = input.lines().map(|line| line.chars().collect()).collect();
        let mut found = HashSet::new();

        run_energisation(&mut found, cursor, &board);

        return found
            .iter()
            .map(|v| (v.x, v.y))
            .collect::<HashSet<_>>().len();
    }

    panic!("file not found")
}

fn best_energising_tile() -> usize {
    if let Some(input) = fs::read_to_string("data/16.input").ok() {
        let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut starting_choices = vec![];

        for idx in 0..board.len() {
            starting_choices.push(Cursor { heading: Heading::Right, x: 0, y: idx });
            starting_choices.push(Cursor { heading: Heading::Down,  x: idx, y: 0 });
            starting_choices.push(Cursor { heading: Heading::Left,  x: board.len() - 1, y: idx });
            starting_choices.push(Cursor { heading: Heading::Up,    x: idx, y: board.len() - 1 });
        }

        return starting_choices.into_iter().map(|cur| {
            let mut found = HashSet::new();

            run_energisation(&mut found, cur, &board);

            return found
                .iter()
                .map(|v| (v.x, v.y))
                .collect::<HashSet<_>>().len();
        }).max().unwrap_or(0);
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", energised_tiles());
    println!("part two: {}", best_energising_tile());
}
