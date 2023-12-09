use std::fs::read_to_string;
use regex::{Regex};

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct GameStat {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn main() {
    let result = read_to_string("day2/data/input.txt").map(parse_games);
    match result {
        Ok(games) => {
            let game_stats = games.iter().map(|game| game_stat(game)).collect::<Vec<GameStat>>();
            let possible_game_stats = possible(&game_stats, 12, 13, 14);
            let possible_games_sum = possible_game_stats.iter().map(|stat| stat.id).sum::<u32>();
            println!("{:?}", possible_games_sum);
            let sum_of_power = game_stats.iter().map(|stat| stat.max_red * stat.max_green * stat.max_blue).sum::<u32>();
            println!("{:?}", sum_of_power);
        }
        Err(err) => { println!("{}", err); }
    }
}

fn possible(stats: &Vec<GameStat>, red: u32, green: u32, blue: u32) -> Vec<&GameStat> {
    return stats
        .iter()
        .filter(|stat| stat.max_red <= red && stat.max_green <= green && stat.max_blue <= blue)
        .collect::<Vec<&GameStat>>();
}


fn game_stat(game: &Game) -> GameStat {
    return GameStat {
        id: game.id,
        max_red: game.reveals.iter().map(|reveal| reveal.red).max().unwrap_or_default(),
        max_green: game.reveals.iter().map(|reveal| reveal.green).max().unwrap_or_default(),
        max_blue: game.reveals.iter().map(|reveal| reveal.blue).max().unwrap_or_default()
    };
}

fn parse_games(file_content: String) -> Vec<Game>
{
    return file_content
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(parse_game)
        .collect();
}

fn parse_game(line: &str) -> Game
{
    let game_regex = Regex::new(r"^Game (?<id>\d+): (?<reveals>.+)$").unwrap();
    // Game 1: 2 blue, 4 green; 7 blue, 1 red, 14 green; 5 blue, 13 green, 1 red; 1 red, 7 blue, 11 green
    return game_regex.captures(line).map(|captures|
        {
            let id = captures["id"].parse::<u32>().expect("Game has no id");
            let reveal_match = captures["reveals"].to_string();
            let reveals = parse_reveals(reveal_match.as_str());
            return Game { id, reveals };
        }).expect("Failed to parse game");
}

fn parse_reveals(line: &str) -> Vec<Reveal>
{
    let reveals = line.split("; ").collect::<Vec<&str>>();
    return reveals.iter().map(|r| parse_reveal(r)).collect();
}

fn parse_reveal(line: &str) -> Reveal
{
    let red_regex = Regex::new(r"(?<count>\d+) red").unwrap();
    let blue_regex = Regex::new(r"(?<count>\d+) blue").unwrap();
    let green_regex = Regex::new(r"(?<count>\d+) green").unwrap();

    let reveal = Reveal {
        blue: blue_regex.captures(line).and_then(|cap| cap["count"].parse::<u32>().ok()).unwrap_or_default(),
        green: green_regex.captures(line).and_then(|cap| cap["count"].parse::<u32>().ok()).unwrap_or_default(),
        red: red_regex.captures(line).and_then(|cap| cap["count"].parse::<u32>().ok()).unwrap_or_default(),
    };

    return reveal;
}