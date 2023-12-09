use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::iter::once;
use itertools::{Itertools};
use crate::Card::{Ace, Jack, King, Num, Queen};

#[derive(Debug, Clone, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Num(u32),
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Ace => 'A',
            King => 'K',
            Queen => 'Q',
            Jack => 'J',
            Num(10) => 'T',
            Num(n) => n.to_string().chars().next().unwrap()
        };
        write!(f, "{}", c)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        score_card(self).cmp(&score_card(other))
    }
}

#[derive(Debug, Clone, Eq)]
enum Pattern {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        score_pattern(self).cmp(&score_pattern(other))
    }
}

#[derive(Debug, Clone, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cards.iter().map(|c| c.to_string()).collect::<String>())
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let pattern_ord = pattern(self).cmp(&pattern(other));
        if pattern_ord != Ordering::Equal {
            return pattern_ord;
        }

        let cards_ord = self.cards.iter()
            .zip(other.cards.iter())
            .map(|(s, o)| s.cmp(o))
            .find(|c| *c != Ordering::Equal)
            .unwrap_or(Ordering::Equal);

        return cards_ord;
    }
}


fn main() {
    let result = read_to_string("day7/data/input.txt").map(parse_hands);
    match result {
        Ok(hands) => {
            let ranked_hands = ranked_hands(hands);

            for (rank, hand, bid) in ranked_hands.iter() {
                println!("{:?}\t{}\t{:?}\t{:?}", rank, hand, bid, pattern(&hand));
            };

            let total = total_score(ranked_hands);
            println!("{:?}", total);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

fn total_score(ranked_hands: Vec<(u64, Hand, u64)>) -> u64 {
    ranked_hands.iter().map(|(rank, _, bid)| rank * bid).sum::<u64>()
}

fn ranked_hands(hands: Vec<(Hand, u32)>) -> Vec<(u64, Hand, u64)> {
    hands.into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .enumerate()
        .map(|(rank, hand)| {
            (u64::try_from(rank + 1).unwrap(), hand.0, u64::from(hand.1))
        })
        .collect_vec()
}


fn score_card(card: &Card) -> u32 {
    match card {
        Ace => 14,
        King => 13,
        Queen => 12,
        Jack => 1,
        Num(n) => *n
    }
}

fn score_pattern(pattern: &Pattern) -> u32 {
    match pattern {
        Pattern::FiveOfAKind => 20,
        Pattern::FourOfAKind => 19,
        Pattern::FullHouse => 18,
        Pattern::ThreeOfAKind => 17,
        Pattern::TwoPair => 16,
        Pattern::OnePair => 15,
        Pattern::HighCard => 14
    }
}

fn pattern(hand: &Hand) -> Pattern {
    let hands = hands(hand);

    hands.iter().map(|hand| {
        let counts = hand.cards.iter().counts()
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1)).collect_vec();

        let first = *counts.first().unwrap();
        let second = counts.get(1);
        match first.1 {
            5 => Pattern::FiveOfAKind,
            4 => Pattern::FourOfAKind,
            3 => match second.unwrap().1 {
                2 => Pattern::FullHouse,
                _ => Pattern::ThreeOfAKind
            },
            2 => match second.unwrap().1 {
                2 => Pattern::TwoPair,
                _ => Pattern::OnePair
            },
            _ => Pattern::HighCard
        }
    }).max().unwrap()
}

fn hands(hand: &Hand) -> Vec<Hand> {
    let maybe_joker = hand.cards.iter().position(|c| *c == Jack);

    maybe_joker.map_or_else(|| vec![hand.clone()], |pos| {
        let distinct_non_jokers =
            hand.cards.iter()
                .filter(|c| **c != Jack)
                .chain(once(&Ace))
                .unique()
                .collect_vec();

        let prefix = hand
            .cards
            .iter()
            .take(pos)
            .chain(hand.cards.iter().skip(pos + 1))
            .map(|it| it.clone())
            .collect_vec();

        let possible_hands = distinct_non_jokers.iter().map(|c| {
            Hand { cards: prefix.iter().chain(once(*c)).map(|it| it.clone()).collect_vec() }
        }).collect_vec();

        return possible_hands.iter().flat_map(hands).collect_vec();
    })
}


// General
fn parse_hands(file_content: String) -> Vec<(Hand, u32)>
{
    return file_content.lines().filter(|l| !l.is_empty())
        .map(|l| parse_hand(l))
        .collect_vec();
}

fn parse_hand(line: &str) -> (Hand, u32)
{
    let sections = line.split(" ").collect_vec();
    let cards = sections[0].chars().map(|c| parse_card(c)).collect_vec();
    let bid = sections[1].parse::<u32>().unwrap();

    return (Hand { cards }, bid);
}

fn parse_card(c: char) -> Card {
    match c {
        'A' => Ace,
        'K' => King,
        'Q' => Queen,
        'J' => Jack,
        'T' => Num(10),
        _ => Num(c.to_digit(10).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
".to_string();

        let result = parse_hands(input);
        let ranked_hands = ranked_hands(result);

        assert_eq!(1, ranked_hands[0].0);
        assert_eq!(2, ranked_hands[1].0);
        assert_eq!(3, ranked_hands[2].0);
        assert_eq!(4, ranked_hands[3].0);
        assert_eq!(5, ranked_hands[4].0);

        assert_eq!(parse_hand("32T3K 765").0, ranked_hands[0].1);
        assert_eq!(parse_hand("KTJJT 220").0, ranked_hands[1].1);
        assert_eq!(parse_hand("KK677 28").0, ranked_hands[2].1);
        assert_eq!(parse_hand("T55J5 684").0, ranked_hands[3].1);
        assert_eq!(parse_hand("QQQJA 483").0, ranked_hands[4].1);

        assert_eq!(765, ranked_hands[0].2);
        assert_eq!(220, ranked_hands[1].2);
        assert_eq!(28, ranked_hands[2].2);
        assert_eq!(684, ranked_hands[3].2);
        assert_eq!(483, ranked_hands[4].2);

        assert_eq!(6440, total_score(ranked_hands));
    }
}