use std::fs;

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

// Each workflow has an entry point, an ordered array of tests, and a fallback if all tests fail.
struct Workflow {
    code: String,
    tests: Vec<String>,
    fallback: String
}

fn apply_problems(problems: &Vec<String>, part: &Part) -> Option<String> {
    for problem in problems {
        if let Some((test, outcome)) = problem.split_once(':') {
            let op = match test.chars().any(|c| c == '>') {
                true => usize::gt,
                false => usize::lt
            };

            if let Some((value, number)) = test.split_once(&['>', '<']) {
                let number = number.parse::<usize>().unwrap();

                let has_passed = match value {
                    "x" => op(&part.x, &number),
                    "m" => op(&part.m, &number),
                    "a" => op(&part.a, &number),
                    "s" => op(&part.s, &number),
                    _ => unreachable!()
                };

                if has_passed {
                    return Some(outcome.to_string());
                }
            }
        } else {
            panic!("bad problem")
        }
    }

    return None;
}

fn build_workflows(workflows: &str) -> Vec<Workflow> {
    workflows
        .lines()
        .filter_map(|line| {
            let line = line.replace("}", "");

            if let Some((code, tests)) = line.split_once('{') {
                let code = code.to_string();

                let mut tests = tests
                    .split(',')
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>();

                if tests.iter().all(|t| t.chars().last() == Some('A')) {
                    // All things lead to success.
                    tests = vec!["A".to_string()];
                } else if tests.iter().all(|t| t.chars().last() == Some('R')) {
                    // All things lead to failure.
                    tests = vec!["R".to_string()];
                }

                // Last element is the fallback.
                if let Some(fallback) = tests.pop() {
                    return Some(Workflow { code, tests, fallback })
                }
            }

            None
        }).collect()
}

fn build_parts(parts: &str) -> Vec<Part> {
    parts
        .lines()
        .filter_map(|line| {
            // Kinda ridiculous data cleaning.
            let line = line
                .replace("{", "")
                .replace("}", "")
                .replace("x", "")
                .replace("m", "")
                .replace("a", "")
                .replace("s", "")
                .replace("=", "");

            let line = line
                .split(',')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<_>>();

            if line.len() == 4 {
                return Some(Part {
                    x: line[0],
                    m: line[1],
                    a: line[2],
                    s: line[3],
                });
            } else {
                return None;
            }
        }).collect()
}

fn passes(part: &Part, workflows: &Vec<Workflow>, work_code: String) -> bool {
    if work_code == "A".to_string() {
        return true;
    }

    if work_code == "R".to_string() {
        return false;
    }

    if let Some(workflow) = workflows.iter().find(|w| w.code == work_code) {
        if let Some(result) = apply_problems(&workflow.tests, part) {
            return passes(part, workflows, result);
        } else {
            return passes(part, workflows, workflow.fallback.clone());
        }
    }

    unreachable!()
}

fn accepted_parts() -> usize {
    // For each part which gets accepted by the workflow, sum up it’s characteristics.

    if let Some(input) = fs::read_to_string("data/19.input").ok() {
        if let Some((workflows, parts)) = input.split_once("\n\n") {
            let workflows = build_workflows(workflows);
            let parts = build_parts(parts);

            return parts
                .iter()
                .filter(|part| passes(*part, &workflows, "in".to_string()))
                .map(|part| part.x + part.m + part.a + part.s)
                .sum();
        }
    }

    panic!("file not found")
}

fn acceptable_combinations(
    workflows: &Vec<Workflow>,
    code: String,
    idx: usize,
    ranges: Vec<(usize, usize)>
) -> usize {
    // No combinations left.
    if ranges[0].0 > ranges[0].1 ||
       ranges[1].0 > ranges[1].1 ||
       ranges[2].0 > ranges[2].1 ||
       ranges[3].0 > ranges[3].1
    {
        return 0;
    }

    // Accepted range of combinations.
    if &code == "A" {
        return
            (ranges[0].1 - ranges[0].0 + 1) *
            (ranges[1].1 - ranges[1].0 + 1) *
            (ranges[2].1 - ranges[2].0 + 1) *
            (ranges[3].1 - ranges[3].0 + 1);
    }

    // Rejected range of combinations.
    if &code == "R" {
        return 0;
    }

    // Lots and lots of parsing down.
    if let Some(workflow) = workflows.iter().find(|w| w.code == code) {
        if workflow.tests.len() == idx {
            return acceptable_combinations(
                workflows,
                workflow.fallback.clone(),
                0,
                ranges
            );
        }

        // Get the part of ranges which needs modification.
        let to_modify = match workflow.tests[idx].chars().nth(0) {
            Some('x') => 0,
            Some('m') => 1,
            Some('a') => 2,
            Some('s') => 3,
            _ => unreachable!()
        };

        // Get the operation.
        let operation = workflow.tests[idx].chars().nth(1).unwrap();

        if let Some((condition, new_code)) = workflow.tests[idx].split_once(':') {
            if let Some((_, number)) = condition.split_once(operation) {
                // Get the condition number.
                let number = number.parse::<usize>().unwrap();

                let mut true_ranges  = ranges.clone();
                let mut false_ranges = ranges.clone();

                // Split into the accepted and unaccepted parts.

                true_ranges[to_modify] = match operation {
                    '>' => (number + 1, ranges[to_modify].1),
                    '<' => (ranges[to_modify].0, number - 1),
                    _ => unreachable!()
                };

                false_ranges[to_modify] = match operation {
                    '>' => (ranges[to_modify].0, number),
                    '<' => (number, ranges[to_modify].1),
                    _ => unreachable!()
                };

                // Sum up the combinations of the two sub-ranges.
                return acceptable_combinations(workflows, new_code.to_string(), 0, true_ranges) +
                    acceptable_combinations(workflows, code, idx+1, false_ranges);
            }
        }
    }

    unreachable!()
}

fn all_acceptable_combinations() -> usize {
    // Calculate how many combinations exists which would be accepted.
    // Each value can be between 1 and 4000 for each.

    if let Some(input) = fs::read_to_string("data/19.input").ok() {
        if let Some((workflows, _)) = input.split_once("\n\n") {
            let workflows = build_workflows(workflows);

            let ranges = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];

            return acceptable_combinations(&workflows, "in".to_string(), 0, ranges);
        }
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", accepted_parts());
    println!("part two: {}", all_acceptable_combinations());
}
