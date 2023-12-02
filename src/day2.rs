use std::fs;

struct Game {
    id: usize,
    data: Vec<Reveal>
}

struct Reveal {
    red: usize,
    green: usize,
    blue: usize
}


fn parse_round(round: String) -> Reveal {
    // Read through a round, modifying the colour counts as we go.

    let (mut red, mut green, mut blue) = (0, 0, 0);

    round
        .split(", ")
        .for_each(|subset| {
            if let Some((count, colour)) = subset.split_once(" ") {
                match colour {
                    "red" => red =     count.parse::<usize>().unwrap_or(0),
                    "green" => green = count.parse::<usize>().unwrap_or(0),
                    "blue" => blue =   count.parse::<usize>().unwrap_or(0),
                    _ => ()
                }
            }
        });

    Reveal { red, green, blue }
}

fn parse_game(schema: String) -> Option<Game> {
    // Parse out the game string into a Game object.

    let schema = schema.replace("Game ", "");

    if let Some((id, games)) = schema.split_once(": ") {
        let parsed_games: Vec<Reveal> = games
            .split("; ")
            .map(|round| parse_round(round.to_string()))
            .collect();

        if let Some(u_id) = id.parse::<usize>().ok() {
            return Some(Game { id: u_id, data: parsed_games });
        }
    }

    None
}

fn possible_game(game: Game) -> Option<usize> {
    // Return the id of the game if it’s possible.

    fn challenge(reveal: &Reveal) -> bool {
        reveal.red <= 12 &&
        reveal.green <= 13 &&
        reveal.blue <= 14
    }

    if game.data.iter().all(|reveal| challenge(reveal)) {
        return Some(game.id);
    } else {
        return None;
    }
}

fn possible_game_count() -> usize {
    // Return the sum of game ids, for all possible games.

    if let Some(game_lines) = fs::read_to_string("data/2.input").ok() {
        let games: usize = game_lines
            .lines()
            .filter_map(|line| parse_game(line.to_string()))
            .filter_map(|game| possible_game(game))
            .sum();

        return games;
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", possible_game_count());
}
