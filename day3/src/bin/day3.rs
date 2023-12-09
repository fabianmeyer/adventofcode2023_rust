use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Entry {
    PartNumber(u32),
    Symbol(char),
    Empty,
}

fn main() {
    let result = read_to_string("day3/data/input.txt").map(parse_engine_schematic);
    match result {
        Ok(engine_schematic) => {
            let part_number_sum = part_number_sum(&engine_schematic);
            println!("{:?}", part_number_sum);
            let gear_ratio_sum = crate::gear_ratio_sum(&engine_schematic);
            println!("{:?}", gear_ratio_sum);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

// Day 1
fn part_number_sum(engine_schematic: &Vec<Vec<Entry>>) -> u32 {
    engine_schematic
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| row_part_number_sum(engine_schematic, row_idx, row))
        .sum::<u32>()
}

fn row_part_number_sum(engine_schematic: &Vec<Vec<Entry>>, row_idx: usize, row: &Vec<Entry>) -> Vec<u32> {
    row
        .iter()
        .enumerate()
        .flat_map(|(col_idx, entry)| col_part_number_sum(engine_schematic, row_idx, col_idx, entry))
        .collect()
}

fn col_part_number_sum<'a>(engine_schematic: &'a Vec<Vec<Entry>>, row_idx: usize, col_idx: usize, entry: &'a Entry) -> Vec<u32> {
    match entry {
        Entry::Symbol(_) =>
            {
                let adjacent = adjacents(engine_schematic, row_idx, col_idx);
                return adjacent.iter().map(|it| part_number(it)).collect();
            }
        _ => vec![]
    }
}


// Day 2
fn gear_ratio_sum(engine_schematic: &Vec<Vec<Entry>>) -> u32 {
    engine_schematic
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| row_gear_ratio_sum(engine_schematic, row_idx, row))
        .sum::<u32>()
}

fn row_gear_ratio_sum(engine_schematic: &Vec<Vec<Entry>>, row_idx: usize, row: &Vec<Entry>) -> Vec<u32> {
    row
        .iter()
        .enumerate()
        .map(|(col_idx, entry)| col_gear_ratio_sum(engine_schematic, row_idx, col_idx, entry))
        .collect()
}

fn col_gear_ratio_sum<'a>(engine_schematic: &'a Vec<Vec<Entry>>, row_idx: usize, col_idx: usize, entry: &'a Entry) -> u32 {
    match entry {
        Entry::Symbol(c) if *c == '*' =>
            {
                let adjacent = adjacents(engine_schematic, row_idx, col_idx);
                let part_numbers = adjacent
                    .iter()
                    .map(|it| part_number(it))
                    .filter(|pt| *pt != 0)
                    .collect_vec();

                if part_numbers.len() != 2 {
                    return 0;
                }

                return part_numbers[0] * part_numbers[1];
            }
        _ => 0
    }
}

// General

fn adjacents(engine_schematic: &Vec<Vec<Entry>>, row_idx: usize, col_idx: usize) -> Vec<&Entry> {
    let prev_row = engine_schematic.get(row_idx - 1);
    let curr_row = engine_schematic.get(row_idx);
    let next_row = engine_schematic.get(row_idx + 1);

    let adjacent_options = [
        prev_row.and_then(|row| row.get(col_idx - 1)),
        prev_row.and_then(|row| row.get(col_idx)),
        prev_row.and_then(|row| row.get(col_idx + 1)),
        //
        curr_row.and_then(|row| row.get(col_idx - 1)),
        curr_row.and_then(|row| row.get(col_idx + 1)),
        //
        next_row.and_then(|row| row.get(col_idx - 1)),
        next_row.and_then(|row| row.get(col_idx)),
        next_row.and_then(|row| row.get(col_idx + 1)),
    ];

    let adjacent_refs = adjacent_options.iter()
        .flatten()
        .dedup()
        .map(|it| *it)
        .collect_vec();

    let adjacent = adjacent_refs.iter()
        .map(|it| *it)
        .collect_vec();

    return adjacent;
}


fn part_number(entry: &Entry) -> u32 {
    match entry {
        Entry::PartNumber(pt) => pt.clone(),
        _ => 0
    }
}

fn parse_engine_schematic(file_content: String) -> Vec<Vec<Entry>>
{
    return file_content
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(parse_engine_schematic_line)
        .collect();
}

fn parse_engine_schematic_line(line: &str) -> Vec<Entry>
{
    let digits = line.chars().take_while(|c| c.is_numeric()).collect::<String>();
    let maybe_part_number = digits.parse::<u32>().map(Entry::PartNumber);

    return match maybe_part_number {
        Ok(part_number) => {
            let mut entries = digits.chars().map(|_| part_number).collect::<Vec<Entry>>();
            let mut rest = parse_engine_schematic_line(&line[digits.len()..]);
            entries.append(&mut rest);
            return entries;
        }
        Err(_) => match line.chars().next() {
            None => vec![],
            Some(c) => {
                let mut entries = vec![match c {
                    '.' => Entry::Empty,
                    c => Entry::Symbol(c)
                }];
                let mut rest = parse_engine_schematic_line(&line[1..]);
                entries.append(&mut rest);
                return entries;
            }
        }
    };
}

