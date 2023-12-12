use std::fs;
use std::collections::VecDeque;

fn walk_record(diagram: &Vec<char>, mut goal: VecDeque<usize>, accrual: usize, cursor: usize) -> usize {
    // Walk the diagram with an accrual system.

    if cursor == diagram.len() {
        if goal.len() == 0 {
            return 1;
        } else {
            if goal.len() == 1 && accrual == goal[0] {
                return 1;
            }

            return 0;
        }
    }

    if goal.len() == 0 {
        if diagram
            .iter()
            .enumerate()
            .filter(|(idx, v)| idx >= &cursor && **v == '#')
            .count() == 0
        {
            return 1;
        } else {
            return 0;
        }
    }

    return match &diagram[cursor..] {
        ['#', ..] => {
            if accrual == goal[0] {
                return 0;
            } else {
                return walk_record(diagram, goal, accrual + 1, cursor + 1);
            }
        },
        ['.', ..] => {
            if accrual == goal[0] {
                drop(goal.pop_front());
                return walk_record(diagram, goal, 0, cursor + 1);
            } else if accrual == 0 {
                return walk_record(diagram, goal, accrual, cursor + 1);
            } else {
                return 0;
            }
        },
        ['?', ..] => {
            if accrual == goal[0] {
                // Found our goal, treat ? as .
                drop(goal.pop_front());
                return walk_record(diagram, goal, 0, cursor + 1);
            } else if accrual > 0 {
                // We need to keep walking.
                return walk_record(diagram, goal, accrual + 1, cursor + 1);
            } else {
                return
                    walk_record(diagram, goal.clone(), 0, cursor + 1) +   // .
                    walk_record(diagram, goal, accrual + 1, cursor + 1);  // #
            }
        },
        _ => 0
    }
}

fn nonogram_combinations() -> usize {
    // How many different ways could a spring record be put together.

    if let Some(input) = fs::read_to_string("data/12.input").ok() {
        return input
            .lines()
            .filter_map(|line| {
                if let Some((diagram, goal)) = line.split_once(" ") {
                    let diagram = diagram.chars().collect::<Vec<char>>();
                    let goal = goal
                        .split(",")
                        .filter_map(|c| c.parse::<usize>().ok())
                        .collect();

                    return Some(walk_record(&diagram, goal, 0, 0));
                } else {
                    return None;
                }
            })
            .sum::<usize>();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", nonogram_combinations());
}
