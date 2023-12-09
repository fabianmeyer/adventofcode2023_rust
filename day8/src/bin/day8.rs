use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter;
use itertools::{Itertools};
use regex::{Regex};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Location(String);

fn main() {
    let result = read_to_string("day8/data/input.txt").map(parse_input);
    match result {
        Ok((directions, network)) => {
            let day1_steps = day1_steps(&Location("AAA".to_string()), &directions, &network);
            println!("{:?}", day1_steps);
            let day2_steps = day2_steps(&directions, &network);
            println!("{:?}", day2_steps);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

// Day1
fn day1_steps(start: &Location, directions: &Vec<Direction>, network: &HashMap<(Location, Direction), Location>) -> usize {
    let direction_iter = iter::repeat(directions).flatten();

    let result = direction_iter
        .scan(start.clone(), |current, dir| {
            let new_location = network.get(&(current.clone(), dir.clone()));
            *current = new_location.unwrap().clone();
            new_location
        })
        .take_while(|Location(current)| !current.ends_with('Z'))
        .count() + 1;

    result
}

// Day2
fn day2_steps(directions: &Vec<Direction>, network: &HashMap<(Location, Direction), Location>) -> i64 {
    let steps = network
        .keys()
        .map(|(loc, _)| loc)
        .unique()
        .filter(|Location(l)| l.ends_with('A'))
        .map(|l| day1_steps(l, directions, network))
        .map(|step| i64::try_from(step).unwrap())
        .collect_vec();

    return steps.iter().fold(1, |lcm, step| num::integer::lcm(lcm, *step));
}

// General
fn parse_input(file_content: String) -> (Vec<Direction>, HashMap<(Location, Direction), Location>)
{
    let direction_line = file_content.lines().next().unwrap();
    let directions = direction_line.chars().map(|c| parse_direction(c)).collect_vec();

    let network_lines = file_content.lines().skip(2).take_while(|l| !l.is_empty());
    let network_regex = Regex::new(r"^(?<source>\w\w\w) = \((?<left>\w\w\w), (?<right>\w\w\w)\)$").unwrap();

    let network = network_lines.flat_map(|line| {
        let captures = network_regex.captures(line).unwrap();
        let source = Location(captures["source"].to_string());
        let left = Location(captures["left"].to_string());
        let right = Location(captures["right"].to_string());
        return vec![((source.clone(), Direction::Left), left), ((source, Direction::Right), right)];
    }).collect::<HashMap<(Location, Direction), Location>>();

    return (directions, network);
}

fn parse_direction(c: char) -> Direction {
    match c {
        'L' => Direction::Left,
        _ => Direction::Right
    }
}



