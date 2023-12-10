use std::fs;
use std::str;

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

/// (red, green, blue)
type Round = (u32, u32, u32);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn is_impossible(round: &Round) -> bool {
    return round.0 > RED_COUNT || round.1 > GREEN_COUNT || round.2 > BLUE_COUNT;
}

fn get_file_content() -> String {
    let file_path = "assets/input";
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn get_games(file_content: &String) -> Vec<Game> {
    let mut games = vec![];
    for line in file_content.split("\n") {
        dbg!(&line);
        let mut temp = line.split(":");
        let mut game = Game {
            id: temp
                .next()
                .unwrap_or("NOPE")
                .replace("Game ", "")
                .parse()
                .unwrap_or(0),
            rounds: vec![],
        };
        for round_as_string in temp.next().unwrap_or("NOPE NOPE").split(";") {
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
    return games;
}

fn main() {
    let games = get_games(&get_file_content());

    let mut game_id_sum = 0;

    for game in games.iter() {
        let mut should_add_game = true;
        for round in game.rounds.iter() {
            if is_impossible(round) {
                println!("Game {} MUST NOT BE ADDED", game.id);
                should_add_game = false;
                break;
            }
        }
        if should_add_game {
            println!("Game {} is added", game.id);
            game_id_sum += game.id;
        }
    }

    println!("Game IDs sum: {}", game_id_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_possible_01() {
        assert_eq!(is_impossible(&(4, 3, 0)), true);
    }

    #[test]
    fn is_possible_02() {
        assert_eq!(is_impossible(&(8, 6, 20)), false);
    }

    #[test]
    fn is_possible_03() {
        assert_eq!(is_impossible(&(14, 3, 15)), false);
    }
}
