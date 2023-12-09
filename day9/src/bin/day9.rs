use std::fs::read_to_string;
use itertools::{Itertools};

fn main() {
    let result = read_to_string("day9/data/input.txt").map(parse_input);
    match result {
        Ok(input) => {
            let extrapolated_next_sum = input.iter().map(interpolate).sum::<i64>();
            println!("{:?}", extrapolated_next_sum);
            let reversed = input.iter().map(|v| v.iter().rev().map(|it| it.clone()).collect_vec()).collect_vec();
            let extrapolated_prev_sum = reversed.iter().map(interpolate).sum::<i64>();
            println!("{:?}", extrapolated_prev_sum);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

fn interpolate(values: &Vec<i64>) -> i64 {
    let extrapolated = if values.iter().all(|v| *v == 0) {
        0
    } else {
        let differences = values.iter().zip(values.iter().skip(1)).map(|(a, b)| b - a).collect_vec();
        let difference = interpolate(&differences);
        let last = values.last().unwrap();
        last + difference
    };

    return extrapolated;
}

// General
fn parse_input(file_content: String) -> Vec<Vec<i64>>
{
    file_content
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect_vec())
        .collect_vec()
}




