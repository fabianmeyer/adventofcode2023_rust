use std::fs::read_to_string;
use itertools::{Itertools};

#[derive(Debug, Clone)]
struct Interval {
    start: u64,
    length: u64,
}

#[derive(Debug)]
struct Mapping {
    destination_interval: Interval,
    source_interval: Interval,
}

#[derive(Debug)]
struct Almanac {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

fn main() {
    let result = read_to_string("day5/data/input.txt").map(parse_almanac);

    match result {
        Ok((seeds, almanac)) => {
            let min_location = seeds.iter().map(|seed| seed_location_num(seed, &almanac)).min().unwrap();
            println!("{:?}", min_location);
            let seed_pairs = seeds.chunks(2).map(|c| (Interval { start: c[0], length: c[1] })).collect_vec();
            let locations = seed_pairs.iter().flat_map(|seed_pair| seeds_location_num(seed_pair, &almanac)).collect_vec();
            let min_location_start = locations.iter().map(|location| location.start).min().unwrap();
            println!("{:#?}", min_location_start);
        }
        Err(err) => { println!("{:?}", err); }
    }
}


// Day 1
fn seed_location_num(seed: &u64, almanac: &Almanac) -> u64 {
    let soil = map_seed(&seed, &almanac.seed_to_soil);
    let fertilizer = map_seed(&soil, &almanac.soil_to_fertilizer);
    let water = map_seed(&fertilizer, &almanac.fertilizer_to_water);
    let light = map_seed(&water, &almanac.water_to_light);
    let temperature = map_seed(&light, &almanac.light_to_temperature);
    let humidity = map_seed(&temperature, &almanac.temperature_to_humidity);
    let location = map_seed(&humidity, &almanac.humidity_to_location);

    return location;
}

fn map_seed(it: &u64, mappings: &Vec<Mapping>) -> u64 {
    return mappings.iter().find_map(|m| {
        if *it < m.source_interval.start {
            return None;
        }

        let offset = it - m.source_interval.start;
        if offset < m.source_interval.length {
            return Some(m.destination_interval.start + offset);
        }

        None
    }).unwrap_or(*it);
}

// Day 2
fn seeds_location_num(seeds: &Interval, almanac: &Almanac) -> Vec<Interval> {
    let soils = map_seeds(&seeds, &almanac.seed_to_soil);
    let fertilizers = soils.iter().flat_map(|soil| map_seeds(&soil, &almanac.soil_to_fertilizer)).collect_vec();
    let waters = fertilizers.iter().flat_map(|fertilizer| map_seeds(&fertilizer, &almanac.fertilizer_to_water)).collect_vec();
    let lights = waters.iter().flat_map(|water| map_seeds(&water, &almanac.water_to_light)).collect_vec();
    let temperatures = lights.iter().flat_map(|light| map_seeds(&light, &almanac.light_to_temperature)).collect_vec();
    let humidities = temperatures.iter().flat_map(|temperature| map_seeds(&temperature, &almanac.temperature_to_humidity)).collect_vec();
    let locations = humidities.iter().flat_map(|humidity| map_seeds(&humidity, &almanac.humidity_to_location)).collect_vec();
    //
    return locations;
}

fn map_seeds(seeds: &Interval, mappings: &Vec<Mapping>) -> Vec<Interval> {
    let initial_mapped: Vec<Interval> = vec![];
    let initial_unmapped = vec![seeds.clone()];
    let initial = (initial_mapped, initial_unmapped);

    let (mapped, unmapped) = mappings.iter().fold(initial, |(mapped, unmapped), m| {
        let mapping_results = unmapped.iter().map(|interval| map_interval(interval, m)).collect_vec();
        let additional_mapped = mapping_results.iter().flat_map(|it| &it.0).map(|it| it.clone()).collect_vec();
        let new_unmapped = mapping_results.iter().flat_map(|it| &it.1).map(|it| it.clone()).collect_vec();

        let new_mapped = mapped.into_iter().chain(additional_mapped.into_iter()).collect_vec();

        return (new_mapped, new_unmapped);
    });

    return mapped.into_iter().chain(unmapped.into_iter()).filter(|i| i.length > 0).collect_vec();
}

fn map_interval(interval: &Interval, mapping: &Mapping) -> (Vec<Interval>, Vec<Interval>) {
    let source_start = mapping.source_interval.start;
    let source_end = source_start + mapping.source_interval.length;

    let (maybe_before, maybe_within_or_after) = split_at(interval, source_start);
    let (maybe_within, maybe_after) = maybe_within_or_after.map_or_else(
        || (None, None),
        |within_or_after| split_at(&within_or_after, source_end));

    let transposed_within = maybe_within.map(|within| transpose_interval(within, mapping));
    let mapped = transposed_within.into_iter().collect_vec();
    let unmapped = maybe_before.into_iter().chain(maybe_after.into_iter()).collect_vec();
    return (mapped, unmapped);
}

fn transpose_interval(interval: Interval, m: &Mapping) -> Interval {
    let transposed_start = interval.start - m.source_interval.start + m.destination_interval.start;
    return Interval { start: transposed_start, ..interval };
}

fn split_at(interval: &Interval, pos: u64) -> (Option<Interval>, Option<Interval>)
{
    if pos <= interval.start {
        return (None, Some(interval.clone()));
    }

    let interval_end = interval.start + interval.length;
    if interval_end < pos {
        return (Some(interval.clone()), None);
    }

    let before = Some(Interval { start: interval.start, length: pos - interval.start });
    let after = Some(Interval { start: pos, length: interval_end - pos });
    return (before, after);
}


// General
fn parse_almanac(file_content: String) -> (Vec<u64>, Almanac)
{
    let sections = file_content.split("\n\n").collect_vec();

    let seeds_section = sections[0];
    let seed_to_soil_section = sections[1];
    let soil_to_fertilizer_section = sections[2];
    let fertilizer_to_water_section = sections[3];
    let water_to_light_section = sections[4];
    let light_to_temperature_section = sections[5];
    let temperature_to_humidity_section = sections[6];
    let humidity_to_location_section = sections[7];

    let seeds = seeds_section
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|it| it.parse::<u64>().unwrap())
        .collect_vec();

    let seed_to_soil = parse_mapping(seed_to_soil_section);
    let soil_to_fertilizer = parse_mapping(soil_to_fertilizer_section);
    let fertilizer_to_water = parse_mapping(fertilizer_to_water_section);
    let water_to_light = parse_mapping(water_to_light_section);
    let light_to_temperature = parse_mapping(light_to_temperature_section);
    let temperature_to_humidity = parse_mapping(temperature_to_humidity_section);
    let humidity_to_location = parse_mapping(humidity_to_location_section);

    return (seeds, Almanac {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    });
}

fn parse_mapping(str: &str) -> Vec<Mapping>
{
    let mapping_lines = str.lines().skip(1).collect_vec();
    return mapping_lines.iter().map(|it|
        {
            let numbers = it.split(" ")
                .map(|num| num.parse::<u64>().unwrap()).collect_vec();
            let destination_interval_start = numbers[0];
            let source_interval_start = numbers[1];
            let interval_length = numbers[2];

            let destination_interval = Interval { start: destination_interval_start, length: interval_length };
            let source_interval = Interval { start: source_interval_start, length: interval_length };

            return Mapping { destination_interval, source_interval };
        }).collect_vec();
}
