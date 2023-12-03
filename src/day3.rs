use std::fs;

struct Number {
    value: usize,
    y: usize,
    start: usize,
    end: usize
}

fn parse_numbers_and_symbols(input: String) -> (Vec<(usize, usize)>, Vec<Number>) {
    // Get all numbers from the string, including their start and end coordinates.
    // Additionally, get the coordinates of all symbols.

    let mut symbols: Vec<(usize, usize)> = vec![];
    let mut numbers: Vec<Number> = vec![];

    let mut value = String::new();
    let mut start = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if !char.is_ascii_digit() && !value.is_empty() {
                // New number found, add it to our list.

                numbers.push(Number {
                    value: value.parse::<usize>().unwrap(),
                    y,
                    start,
                    end: x - 1
                });

                value.clear();
            }

            if !char.is_ascii_digit() && char != '.' {
                // Symbol found, add to the map.

                symbols.push((y, x));
            }

            if char.is_ascii_digit() {
                // part of a number found.

                if value.is_empty() {
                    start = x;
                }

                value.push(char);
            }
        });

        // Account for numbers at the end of a line.
        if !value.is_empty() {
            numbers.push(Number {
                value: value.parse::<usize>().unwrap(),
                y,
                start,
                end: line.len() - 1
            });

            value.clear();
        }
    });

    return (symbols, numbers);
}

fn sum_of_true_parts() -> usize {
    // Sum all numbers which have a symbol on their perimeter.

    if let Some(input) = fs::read_to_string("data/3.input").ok() {
        let (symbols, numbers) = parse_numbers_and_symbols(input);

        return numbers
            .iter()
            .filter(|number| {
                // Check whether a symbol is in any of the space surrounding a number.
                // `.checked_sub()` handles number.start being 0.

                symbols.iter().any(|symbol| {
                    symbol.0.abs_diff(number.y) <= 1 &&
                    symbol.1 <= (number.end + 1) &&
                    symbol.1 >= (number.start.checked_sub(1).unwrap_or(0))
                })
            })
            .map(|number| number.value)
            .sum();
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", sum_of_true_parts());
}
