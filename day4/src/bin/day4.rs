use std::collections::HashSet;
use std::fs::read_to_string;
use itertools::{Itertools};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

fn main() {
    let result = read_to_string("day4/data/input.txt").map(parse_cards);
    match result {
        Ok(cards) => {
            let points = cards.iter().map(points).sum::<u64>();
            println!("{:?}", points);
            let played_cards = resolve(cards);
            println!("{:?}", played_cards);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

// Day 1
fn points(card: &Card) -> u64
{
    let matches = matches(&card);

    if matches == 0 {
        return 0;
    }

    return 2u64.pow(matches - 1);
}

fn matches(card: &Card) -> u32 {
    u32::try_from(card.your_numbers.intersection(&card.winning_numbers).count()).unwrap()
}


// Day 2
fn resolve(cards : Vec<Card>) -> u64
{

    let initial_deck = cards.iter().map(|_| 1u64).collect_vec();
    let initial = (initial_deck, 0u64);
    let result = cards.iter().fold(initial, |(deck, played_cards), card| {

        let instances = deck.first().unwrap();
        let matches = usize::try_from(matches(card)).unwrap();
        let updated_cards = deck.iter().skip(1)
            .take(matches)
            .map(|it| it + instances)
            .collect_vec();
        let remaining_cards = deck.iter().skip(1 + matches).map(|v| *v).collect_vec();
        let new_deck = updated_cards.iter().chain(remaining_cards.iter()).map(|v| *v).collect_vec();
        return (new_deck, played_cards + instances);
    });

    return result.1;
}

// General
fn parse_cards(file_content: String) -> Vec<Card>
{
    return file_content
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(parse_card)
        .collect();
}

fn parse_card(line: &str) -> Card
{
    let mut colon_split = line.split(":");
    let numbers = colon_split.next().expect("Line must have numbers");

    let mut numbers_split = numbers.split("|");
    let winning_numbers = parse_numbers(
        numbers_split
            .next()
            .expect("Line must have winning numbers"));

    let your_numbers = parse_numbers(
        numbers_split
            .next()
            .expect("Line must have your numbers"));

    return Card { winning_numbers, your_numbers };
}

fn parse_numbers(line: &str) -> HashSet<u32>
{
    line.split(" ")
        .filter(|num_str| !num_str.is_empty())
        .flat_map(|num_str| num_str.parse::<u32>())
        .collect::<HashSet<u32>>()
}