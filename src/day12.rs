use std::fs;
use std::collections::HashMap;
use std::iter::repeat;

fn walk_record(
    diagram: &Vec<char>,
    goal: &Vec<usize>,
    accrual: usize,
    cursor: usize,
    goal_cur: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>
) -> usize {
    // Walk the diagram with an accrual system.

    if let Some(cache_entry) = cache.get(&(accrual, cursor, goal_cur)) {
        return *cache_entry;
    }

    if cursor == diagram.len() {
        if goal_cur == goal.len() {
            return 1;
        } else {
            if goal_cur == goal.len() - 1 && accrual == goal[goal_cur] {
                return 1;
            }

            return 0;
        }
    }

    if goal_cur == goal.len() {
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

    return match diagram[cursor..] {
        ['#', ..] => {
            if accrual == goal[goal_cur] {
                return 0;
            } else {
                let result = walk_record(diagram, goal, accrual + 1, cursor + 1, goal_cur, cache);

                return result;
            }
        },
        ['.', ..] => {
            if accrual == goal[goal_cur] {
                let result = walk_record(diagram, goal, 0, cursor + 1, goal_cur + 1, cache);

                return result;
            } else if accrual == 0 {
                let result = walk_record(diagram, goal, accrual, cursor + 1, goal_cur, cache);

                return result;
            } else {
                return 0;
            }
        },
        ['?', ..] => {
            if accrual == goal[goal_cur] {
                // Found our goal, treat ? as .
                // drop(goal.pop_front());
                let result = walk_record(diagram, goal, 0, cursor + 1, goal_cur + 1, cache);

                return result;
            } else if accrual > 0 {
                // We need to keep walking.
                let result = walk_record(diagram, goal, accrual + 1, cursor + 1, goal_cur, cache);

                return result;
            } else {
                // Test each option.
                let result =
                    walk_record(diagram, goal, 0, cursor + 1, goal_cur, cache) +           // .
                    walk_record(diagram, goal, accrual + 1, cursor + 1, goal_cur, cache);  // #

                cache.insert((accrual, cursor, goal_cur), result);

                return result;
            }
        },
        _ => 0
    }
}

fn n_nonogram_combinations(n: usize) -> usize {
    // How many different ways could a spring record be put together, when repeated N times?

    if let Some(input) = fs::read_to_string("data/12.input").ok() {
        return input
            .lines()
            .filter_map(|line| {
                if let Some((diagram, goal)) = line.split_once(" ") {
                    let mut diagram = repeat(
                            diagram.chars().chain("?".chars())
                        ).take(n).flatten().collect::<Vec<char>>();
                    drop(diagram.pop());

                    let goal = repeat(
                            goal.split(",").filter_map(|c| c.parse::<usize>().ok())
                        ).take(n).flatten().collect();


                    // Using the cache speeds up the splitting operation by an unbelievable margin.
                    let mut cache = HashMap::new();

                    return Some(walk_record(&diagram, &goal, 0, 0, 0, &mut cache));
                } else {
                    return None;
                }
            })
            .sum::<usize>();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", n_nonogram_combinations(1));
    println!("part two: {}", n_nonogram_combinations(5));
}
