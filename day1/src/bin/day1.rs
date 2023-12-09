use std::fs::read_to_string;
use regex::{Regex};

fn main() {
    let result = read_to_string("day1/data/input.txt").map(parse_calibration_values);
    match result {
        Ok(calibration_values) => {
            println!("{}", calibration_values.iter().sum::<u32>());
        }
        Err(err) => { println!("{}", err); }
    }
}

fn parse_calibration_values(file_content: String) -> Vec<u32>
{
    let lines = file_content.lines();
    let from_front = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let from_back = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let calibration_values = lines.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {

            let reversed_line = line.chars().rev().collect::<String>();
            let maybe_first_match = from_front.find(line);
            let maybe_last_match = from_back.find(reversed_line.as_str());

            let result = maybe_first_match
                .and_then(|first_match| {
                    let first = to_digit(first_match.as_str());
                    return maybe_last_match.map(|last_match| {
                        let last = to_digit(last_match.as_str().chars().rev().collect::<String>().as_str());
                        return first * 10 + last;
                    });

                });

            println!("'{}' '{}' '{}' '{}'", line, maybe_first_match.map(|m| m.as_str()).unwrap_or_default(), maybe_last_match.map(|m| m.as_str()).unwrap_or_default(), result.unwrap_or_default());

            return result.expect("Invalid line");
        })
        .collect();

    return calibration_values;
}


fn to_digit(str: &str) -> u32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str.parse().expect("Not a number")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen".to_string();

        let result = parse_calibration_values(input);

        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], result);
        assert_eq!(281, result.iter().sum::<u32>());
    }

    #[test]
    fn test_edge() {
        let input = "s8twoned".to_string();

        let result = parse_calibration_values(input);

        assert_eq!(vec![81], result);
    }


}