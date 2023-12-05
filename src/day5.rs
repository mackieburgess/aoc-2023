use std::fs;

#[derive(Clone)]
struct Connection {
    start: isize,
    end: isize,
    offset: isize,

}

#[derive(Clone)]
struct Map {
    from: String,
    to: String,
    map: Vec<Connection>
}

fn get_almanac(almanac: &str) -> Vec<Map>{
    // Parsing.

    return almanac
        .replace(" map:", "")
        .split("\n\n")
        .filter_map(|mapping| {
            let (mappings, numbers) = mapping.split_once("\n").unwrap();
            let (start_code, end_code) = mappings.split_once("-to-").unwrap();

            let connections = numbers
                .lines()
                .filter_map(|line| {
                    let line = line
                        .split_whitespace()
                        .filter_map(|v| v.parse::<isize>().ok())
                        .collect::<Vec<isize>>();
                    if line.len() == 3 {
                        let (dest, src, range) = (line[0], line[1], line[2]);

                        return Some(Connection {
                            start: src,
                            end: src + range - 1,
                            offset: dest - src
                        });
                    } else { return None; }

                }).collect::<Vec<Connection>>();

            return Some(Map { from: start_code.to_string(), to: end_code.to_string(), map: connections });
        }).collect::<Vec<Map>>();
}

fn get_seeds(seeds: &str) -> Vec<isize> {
    return seeds
        .replace("seeds: ", "")
        .split_whitespace()
        .filter_map(|s| s.parse::<isize>().ok())
        .collect();
}

fn find_values(seeds: Vec<isize>, almanac: Vec<Map>, start: String, end: String) -> Vec<isize> {
    if start == end {
        return seeds;
    }

    if let Some(map) = almanac.clone().iter().find(|map| map.from == start) {
        let new_seeds = seeds.iter().map(|seed| {
            for connection in map.map.iter() {
                if (connection.start..=connection.end).contains(seed) {
                    return seed + connection.offset;
                }
            }
            return *seed;
        }).collect::<Vec<isize>>();

        return find_values(new_seeds, almanac, map.to.clone(), end);
    }

    panic!("No map from {start}");
}

fn get_smallest_seed() -> isize {
    if let Some(almanac) = fs::read_to_string("data/5.input").ok() {
        if let Some((seeds, almanac)) = almanac.split_once("\n\n") {
            let seeds = get_seeds(seeds);
            let almanac = get_almanac(almanac);

            return find_values(
                seeds,
                almanac,
                "seed".to_string(),
                "location".to_string()
            ).into_iter().min().unwrap_or(0);
        }
    }

    panic!("file not found")
}

fn get_seed_ranges(seeds: &str) -> Vec<(isize, isize)> {
    let seeds = seeds
        .replace("seeds: ", "")
        .split_whitespace()
        .filter_map(|s| s.parse::<isize>().ok())
        .collect::<Vec<isize>>();

    return seeds
        .chunks_exact(2)
        .map(|chunk| {
            let start = chunk[0];
            let end = chunk[1] + start - 1;

            return (start, end);
        }).collect();
}

fn find_ranges(seeds: Vec<(isize, isize)>, almanac: Vec<Map>, start: String, end: String) -> Vec<(isize, isize)> {
    if start == end { return seeds; }

    if let Some(map) = almanac.clone().iter().find(|map| map.from == start) {
        let new_seeds = seeds.iter().map(|seed_range| {
            // Items are only mapped once, this ensures that.
            let mut found_ranges = vec![];

            // Start with the core range.
            let mut to_check = vec![*seed_range];

            for conn in map.map.iter() {
                let mut try_next_time = vec![];

                // Hairy, *hairy* manual logic.
                // Should probably be redone using range objects.
                while let Some(seed) = to_check.pop() {
                    if conn.start <= seed.0 && seed.1 <= conn.end {
                        // Connection contains seed range, entire thing is modified.
                        found_ranges.push((
                            seed.0 + conn.offset,
                            seed.1 + conn.offset
                        ));
                    } else if seed.0 < conn.start && conn.end < seed.1 {
                        // Seed range contains connection.
                        try_next_time.push((seed.0, conn.start - 1));
                        found_ranges.push((
                            conn.start + conn.offset,
                            (conn.end) + conn.offset
                        ));
                        try_next_time.push((conn.end + 1, seed.1));
                    } else if conn.start <= seed.0 && seed.0 <= conn.end {
                        // Seed range ends outside connection.
                        found_ranges.push((
                            seed.0 + conn.offset,
                            (conn.end) + conn.offset
                        ));
                        try_next_time.push((conn.end + 1, seed.1));
                    } else if conn.start <= seed.1 && seed.1 <= conn.end {
                        // Seed range starts outside connection.
                        try_next_time.push((seed.0, conn.start - 1));
                        found_ranges.push((
                            conn.start + conn.offset,
                            seed.1 + conn.offset
                        ));
                    } else {
                        // Completely distinct.
                        try_next_time.push(seed)
                    }
                }

                // Pass anything which wasn’t mapped into the next iteration.
                to_check = try_next_time;
            }

            found_ranges.append(&mut to_check);
            return found_ranges;
        }).collect::<Vec<Vec<(isize, isize)>>>();

        let new_seeds: Vec<(isize, isize)> = new_seeds.into_iter().flat_map(|v| v).collect();

        return find_ranges(new_seeds, almanac, map.to.clone(), end);
    }

    panic!("No map from {start}");
}

fn get_smallest_seed_from_range() -> isize {
    if let Some(almanac) = fs::read_to_string("data/5.input").ok() {
        if let Some((seeds, almanac)) = almanac.split_once("\n\n") {
            let seeds = get_seed_ranges(seeds);
            let almanac = get_almanac(almanac);

            // Instead of checking individual elements, we check ranges of elements, splitting
            // ranges apart when need be.
            let ranges = find_ranges(
                seeds,
                almanac,
                "seed".to_string(),
                "location".to_string()
            );

            // Smallest start to a range.
            return ranges
                .into_iter()
                .min_by(|left, right| left.0.cmp(&right.0))
                .unwrap_or((-1, -1))
                .0;
        }
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", get_smallest_seed());
    println!("part two: {}", get_smallest_seed_from_range());
}
