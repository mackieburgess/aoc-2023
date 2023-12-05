use std::fs;

#[derive(Clone)]
struct Connection {
    start: usize,
    end: usize,
    modification: isize,

}

#[derive(Clone)]
struct Map {
    from: String,
    to: String,
    map: Vec<Connection>
}

fn find_values(seeds: Vec<usize>, almanac: Vec<Map>, start: String, end: String) -> Vec<usize> {
    if start == end {
        return seeds;
    }

    if let Some(map) = almanac.clone().iter().find(|map| map.from == start) {
        let new_seeds = seeds.iter().map(|seed| {
            for connection in map.map.iter() {
                if (connection.start..connection.end).contains(seed) {
                    return ((*seed as isize) + connection.modification) as usize;
                }
            }
            return *seed;
        }).collect::<Vec<usize>>();

        return find_values(new_seeds, almanac, map.to.clone(), end);
    }

    panic!("No map from {start}");
}

fn get_smallest_seed() -> usize {
    if let Some(almanac) = fs::read_to_string("data/5.input").ok() {
        if let Some((seeds, almanac)) = almanac.split_once("\n\n") {
            let almanac = almanac
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
                                .filter_map(|v| v.parse::<usize>().ok())
                                .collect::<Vec<usize>>();
                            if line.len() == 3 {
                                let (dest, src, range) = (line[0], line[1], line[2]);

                                let modification = (dest as isize) - (src as isize);

                                return Some(Connection { start: src, end: src + range, modification});
                            } else { return None; }

                        }).collect::<Vec<Connection>>();

                    return Some(Map { from: start_code.to_string(), to: end_code.to_string(), map: connections });
                }).collect::<Vec<Map>>();

            let seeds = seeds
                .replace("seeds: ", "")
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            return *find_values(
                seeds,
                almanac,
                "seed".to_string(),
                "location".to_string()
            ).iter().min().unwrap_or(&0);
        }
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", get_smallest_seed());
}
