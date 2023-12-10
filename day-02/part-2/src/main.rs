use std::fs;
use std::str;

/// (red, green, blue)
type Round = (u32, u32, u32);

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

fn get_power(round: &Round) -> u32 {
    return round.0 * round.1 * round.2;
}

fn get_file_content() -> String {
    let file_path = "assets/input";
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn get_games(file_content: &String) -> Vec<Game> {
    let mut games = vec![];
    for line in file_content.split("\n") {
        if line != "" {
            dbg!(&line);
            let temp = line.split(":");
            let mut game = Game { rounds: vec![] };
            for round_as_string in temp.last().unwrap_or("NOPE NOPE").split(";") {
                let mut round: Round = (0, 0, 0);
                for ball_count_for_one_color in round_as_string.split(",") {
                    let k = ball_count_for_one_color.split(" ").collect::<Vec<&str>>();
                    let index = if k[0] == "" { 2 } else { 1 };
                    if k[index] == "red" {
                        round.0 = k[index - 1].parse().unwrap_or(0);
                    }
                    if k[index] == "green" {
                        round.1 = k[index - 1].parse().unwrap_or(0);
                    }
                    if k[index] == "blue" {
                        round.2 = k[index - 1].parse().unwrap_or(0);
                    }
                }
                game.rounds.push(round);
            }
            dbg!(&game);
            games.push(game);
        }
    }
    return games;
}

fn main() {
    let games = get_games(&get_file_content());

    let mut game_powers_sum = 0;

    for game in games.iter() {
        let mut minimum_round: Round = (0, 0, 0);
        for round in game.rounds.iter() {
            if round.0 > minimum_round.0 {
                minimum_round.0 = round.0;
            }
            if round.1 > minimum_round.1 {
                minimum_round.1 = round.1;
            }
            if round.2 > minimum_round.2 {
                minimum_round.2 = round.2;
            }
        }
        game_powers_sum += get_power(&minimum_round);
    }

    println!("Game Powers sum: {}", game_powers_sum);
}

#[cfg(test)]
mod tests {}
