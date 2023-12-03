use std::fs;

struct Number {
    value: usize,
    y: usize,
    start: usize,
    end: usize
}

struct Symbol {
    value: char,
    y: usize,
    x: usize
}

fn parse_numbers_and_symbols() -> (Vec<Symbol>, Vec<Number>) {
    // Get all numbers from the string, including their start and end coordinates.
    // Additionally, get the coordinates of all symbols.

    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    let mut value = String::new();
    let mut start = 0;

    let input = fs::read_to_string("data/3.input").unwrap();

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

                symbols.push(Symbol {
                    value: char,
                    y, x
                });
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

fn symbol_touches_number(symbol: &Symbol, number: &Number) -> bool {
    // The symbol exists on the perimeter of the number.

    return
        symbol.y.abs_diff(number.y) <= 1 &&
        symbol.x <= (number.end + 1) &&
        symbol.x >= (number.start.checked_sub(1).unwrap_or(0));
}

fn sum_of_true_parts() -> usize {
    // Sum all numbers which have a symbol on their perimeter.

    let (symbols, numbers) = parse_numbers_and_symbols();

    return numbers
        .iter()
        .filter(|number| {
            // Check whether a symbol is in any of the space surrounding a number.
            // `.checked_sub()` handles number.start being 0.

            symbols.iter().any(|symbol| symbol_touches_number(symbol, number))
        })
        .map(|number| number.value)
        .sum();

}

fn sum_of_gear_ratios() -> usize {
    // Find the sum of all gear ratios in the input.
    // A gear is a `*` symbol with exactly two numbers near it,
    // its ratio is the product of those two numbers.

    let (symbols, numbers) = parse_numbers_and_symbols();

    return symbols
        .iter()
        .filter(|symbol| symbol.value == '*')
        .filter_map(|symbol| {
            // Find each number pertinent to our gear.

            let cogs = numbers
                .iter()
                .filter(|number| symbol_touches_number(symbol, number))
                .collect::<Vec<&Number>>();

            if cogs.len() == 2 {
                return Some(cogs[0].value * cogs[1].value);
            } else {
                None
            }
        }).sum();
}

fn main() {
    println!("part one: {}", sum_of_true_parts());
    println!("part two: {}", sum_of_gear_ratios());
}
