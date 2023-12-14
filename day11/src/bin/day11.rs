use std::cmp::{max, min};
use std::fs::read_to_string;
use itertools::{Itertools};
use crate::Space::Void;
use crate::Space::Galaxy;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Space {
    Void(usize),
    Galaxy,
}


fn main() {
    let result = read_to_string("day11/data/input.txt").map(parse_input);
    match result {
        Ok(input) => {
            let universe = expand(input);
            let galaxies = galaxies(&universe);
            let galaxy_pairs = pairs(galaxies);
            let distances = galaxy_pairs.into_iter().map(|(a, b)| distance(&universe, a, b)).sum::<usize>();
            println!("{:?}", distances);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

fn distance(universe: &Vec<Vec<Space>>, (a_row, a_col): (usize, usize), (b_row, b_col): (usize, usize)) -> usize
{
    let min_col = min(a_col, b_col);
    let max_col = max(a_col, b_col);
    let min_row = min(a_row, b_row);
    let max_row = max(a_row, b_row);

    let cols = &universe[min_row][min_col..max_col];
    (min_row..max_row)
        .map(|row| &universe[row][min_col]).chain(cols).map(|s| match s {
        Void(n) => *n,
        Galaxy => 1
    }).sum()
}

fn galaxies(space: &Vec<Vec<Space>>) -> Vec<(usize, usize)> {
    space.iter().enumerate().flat_map(|(row_num, row)| {
        row.iter().enumerate().flat_map(move |(col_num, s)| {
            match s {
                Void(_) => None,
                Galaxy => Some((row_num, col_num))
            }
        })
    }).collect_vec()
}

fn pairs(galaxies: Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    if galaxies.len() < 2 {
        vec![]
    } else {
        let mut iter = galaxies.into_iter();
        let head = iter.next().unwrap();
        let tail = iter.collect_vec();
        let mut result =
            tail.iter()
                .map(|t| (head.clone(), t.clone()))
                .collect_vec();
        let mut tail_pairs = pairs(tail);
        result.append(&mut tail_pairs);
        result
    }
}

fn expand(space: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let expand_col = space.first().unwrap().iter().enumerate().map(|(c, _)| space.iter().all(|row| is_void(&row[c]))).collect_vec();

    space.into_iter()
        .map(|row| {
            let expanded_row = row.into_iter().zip(expand_col.iter()).map(|(s, expand)| if *expand { Void(1000000) } else { s }).collect_vec();
            if expanded_row.iter().all(is_void) {
                expanded_row.into_iter().map(|_| Void(1000000)).collect_vec()
            } else {
                expanded_row
            }
        })
        .collect_vec()
}

fn is_void(s: &Space) -> bool {
    match s {
        Void(_) => true,
        Galaxy => false
    }
}

// General
fn parse_input(file_content: String) -> Vec<Vec<Space>>
{
    file_content
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| line.chars().map(|n| match n {
            '#' => Galaxy,
            _ => Void(1)
        }).collect_vec())
        .collect_vec()
}




