use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

#[derive(Debug, Clone)]
struct Result {
    final_distance: u64,
}


fn main() {
    let result = read_to_string("day6/data/input.txt").map(parse_races_day1);
    match result {
        Ok(races) => {
            let single_wins = races.iter().map(wins).map(|wins| u64::try_from(wins.iter().count()).unwrap()).product::<u64>();
            println!("{:#?}", single_wins);
            let joint_race_time = races.iter().map(|r| r.time.to_string()).collect::<String>().parse::<u64>().unwrap();
            let joint_race_distance = races.iter().map(|r| r.distance.to_string()).collect::<String>().parse::<u64>().unwrap();
            let joint_race = Race { time: joint_race_time, distance: joint_race_distance };
            let joint_wins = wins(&joint_race).iter().count();
            println!("{:#?}", joint_wins);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

fn wins(race: &Race) -> Vec<Result> {
    let min_press_duration = race.distance / race.time;
    let max_press_duration = race.time - 1;
    let press_durations = min_press_duration..max_press_duration;
    return press_durations
        .map(|press_duration| {
            let remaining_time = race.time - press_duration;
            let final_distance = press_duration * remaining_time;
            return Result { final_distance };
        })
        .filter(|result| result.final_distance > race.distance)
        .collect_vec();
}

// General
fn parse_races_day1(file_content: String) -> Vec<Race>
{
    let lines = file_content.lines().collect_vec();
    let time_line = lines[0];
    let distance_line = lines[1];

    let times = time_line.split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .flat_map(|it| it.parse::<u64>())
        .collect_vec();

    let distances = distance_line
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .flat_map(|it| it.parse::<u64>())
        .collect_vec();

    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect_vec();

    return races;
}
