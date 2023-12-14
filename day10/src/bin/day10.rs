use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::read_to_string;
use std::io::Read;
use std::iter::{once, Once};
use itertools::{Itertools, unfold};
use std::time::Instant;
use termion::{color, style};
use termion::color::{Green, Red};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    row: i16,
    col: i16,
}

impl Point {
    fn new(row: i16, col: i16) -> Point {
        Point {
            row,
            col,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Joint {
    nw: Option<Point>,
    ne: Option<Point>,
    sw: Option<Point>,
    se: Option<Point>,
}

impl Joint {
    fn new(nw: Option<Point>, ne: Option<Point>, sw: Option<Point>, se: Option<Point>) -> Joint {
        Joint { nw, ne, sw, se }
    }
}

fn main() {
    let maybe_file_content = read_to_string("day10/data/input.txt");

    match maybe_file_content {
        Ok(file_content) => {
            let start = file_content.lines().enumerate().flat_map(|(row, l)| l.find('S').map(|col| Point::new(i16::try_from(row).unwrap(), i16::try_from(col).unwrap()))).next().unwrap();
            let input = parse_input(&file_content);
            let bidirectional = filter_bidirectional(input);

            let now = Instant::now();
            let loops = reachable_loops(&bidirectional, &start);
            let path = loops.first().unwrap();
            println!("{}", now.elapsed().as_millis());

            let last = path.back().unwrap();
            let loop_start = path.iter().find_position(|p| *p == last).unwrap().0;
            let farthest = (path.len() - loop_start) / 2 + loop_start;
            println!("{}", farthest);

            let now = Instant::now();
            let flooded_joints = fill(&bidirectional);
            let flooded_points = flooded_joints.iter().flat_map(|j| [j.nw.clone(), j.ne.clone(), j.sw.clone(), j.se.clone()]).flatten().collect::<HashSet<_>>();

            let loop_points = path.iter().collect::<HashSet<_>>();
            println!("{}", now.elapsed().as_millis());

            let mut enclosed = bidirectional.keys().cloned().collect::<HashSet<_>>();
            loop_points.iter().for_each(|p| { enclosed.remove(*p); });
            flooded_points.iter().for_each(|p| { enclosed.remove(p); });
            println!("{}", enclosed.len());

            print(&bidirectional, flooded_points, loop_points);
        }
        Err(err) => { println!("{:?}", err); }
    }
}

fn print(map: &HashMap<Point, Vec<Point>>, flooded_points: HashSet<Point>, loop_points: HashSet<&Point>) {
    let rows = map.keys().map(|k| k.row).max().unwrap() + 1;
    let cols = map.keys().map(|k| k.col).max().unwrap() + 1;

    for row in 0..rows {
        for col in 0..cols {
            let pos = Point::new(row, col);
            let north = Point::new(row - 1, col);
            let south = Point::new(row + 1, col);
            let west = Point::new(row, col - 1);
            let east = Point::new(row, col + 1);
            let none = vec![];
            let edges = map.get(&pos).unwrap_or(&none);
            let x =
                if edges.contains(&north) && edges.contains(&south) { '┃' } else if edges.contains(&east) && edges.contains(&west) { '━' } else if edges.contains(&north) && edges.contains(&east) { '┗' } else if edges.contains(&north) && edges.contains(&west) { '┛' } else if edges.contains(&south) && edges.contains(&west) { '┓' } else if edges.contains(&south) && edges.contains(&east) { '┏' } else { ' ' };

            if loop_points.contains(&pos) {
                print!("{}{}", color::Bg(Red), x);
            } else if flooded_points.contains(&pos) {
                print!("{}{}", color::Bg(Green), x);
            } else {
                print!("{}{}", style::Reset, x);
            }
        }
        print!("{}{}", style::Reset, '\n');
    }

    print!("\n");
}


fn reachable_loops(map: &HashMap<Point, Vec<Point>>, start: &Point) -> Vec<LinkedList<Point>> {
    let start_path = once(start.clone()).collect::<LinkedList<_>>();
    let loops = unfold(vec![start_path], |paths| {
        if paths.is_empty() {
            return None;
        }

        let (new_loops, mut new_paths): (Vec<_>, Vec<_>) =
            paths
                .drain(..)
                .flat_map(|path| next_paths(map, path))
                .partition(is_loop);

        paths.append(&mut new_paths);

        Some(new_loops)
    }).flatten().collect_vec();

    return loops;
}

fn fill(map: &HashMap<Point, Vec<Point>>) -> HashSet<Joint> {
    let initial_visited = HashSet::<Joint>::new();
    let start = Joint::new(Some(Point::new(0, 0)), Some(Point::new(0, 1)), Some(Point::new(1, 0)), Some(Point::new(1, 1)));
    let initial_current = once(start).collect::<HashSet<_>>();

    let joints = unfold((initial_visited, initial_current), |(visited, current)| {
        if current.is_empty() {
            return None;
        }

        let mut next = current.iter()
            .flat_map(|c| reachable(map, c))
            .filter(|j| !(visited.contains(j) || current.contains(j)))
            .collect::<HashSet<_>>();

        let result = current.iter().cloned().collect::<HashSet<_>>();

        current.drain().for_each(|j| { visited.insert(j); });
        next.drain().for_each(|j| { current.insert(j); });

        Some(result)
    })
        .flatten()
        .collect::<HashSet<Joint>>();

    joints
}

fn reachable(map: &HashMap<Point, Vec<Point>>, current: &Joint) -> Vec<Joint> {
    let nnw = current.nw.as_ref().map(|nw| Point::new(nw.row - 1, nw.col)).filter(|p| map.contains_key(p));
    let nne = current.ne.as_ref().map(|ne| Point::new(ne.row - 1, ne.col)).filter(|p| map.contains_key(p));
    let n = if passable(map, &current.nw, &current.ne) {
        Some(Joint::new(nnw, nne, (&current.nw).clone(), (&current.ne).clone()))
    } else { None };


    let ssw = current.sw.as_ref().map(|sw| Point::new(sw.row + 1, sw.col)).filter(|p| map.contains_key(p));
    let sse = current.se.as_ref().map(|se| Point::new(se.row + 1, se.col)).filter(|p| map.contains_key(p));
    let s = if passable(map, &current.sw, &current.se) {
        Some(Joint::new((&current.sw).clone(), (&current.se).clone(), ssw, sse))
    } else { None };

    let ene = current.ne.as_ref().map(|ne| Point::new(ne.row, ne.col + 1)).filter(|p| map.contains_key(p));
    let ese = current.se.as_ref().map(|se| Point::new(se.row, se.col + 1)).filter(|p| map.contains_key(p));
    let e = if passable(map, &current.ne, &current.se) {
        Some(Joint::new((&current.ne).clone(), ene, (&current.se).clone(), ese))
    } else { None };

    let wnw = current.nw.as_ref().map(|nw| Point::new(nw.row, nw.col - 1)).filter(|p| map.contains_key(p));
    let wsw = current.sw.as_ref().map(|sw| Point::new(sw.row, sw.col - 1)).filter(|p| map.contains_key(p));
    let w = if passable(map, &current.nw, &current.sw) {
        Some(Joint::new(wnw, (&current.nw).clone(), wsw, (&current.sw).clone()))
    } else { None };

    return [n, s, e, w].into_iter().flatten().collect_vec();
}

fn passable(map: &HashMap<Point, Vec<Point>>, maybe_a: &Option<Point>, maybe_b: &Option<Point>) -> bool {
    match maybe_a {
        Some(a) => {
            match maybe_b {
                Some(b) => map.get(a).map_or_else(|| false, |it| !it.contains(b)),
                None => true
            }
        }
        None => true
    }
}


fn next_paths(map: &HashMap<Point, Vec<Point>>, mut path: LinkedList<Point>) -> Vec<LinkedList<Point>> {
    let position = path.back().unwrap();

    let prev_position = if path.len() > 1 { path.iter().nth_back(1) } else { None };
    let maybe_nexts = map.get(&position);
    match maybe_nexts {
        None => vec![],
        Some(nexts) => {
            let valid_nexts = nexts.iter().filter(|next| match prev_position {
                None => true,
                Some(prev) => *next != prev
            }).map(|n| n.clone()).collect_vec();

            if valid_nexts.len() == 1 {
                let head = valid_nexts.first().unwrap();
                path.push_back(head.clone());
                vec![path]
            } else {
                valid_nexts.iter().map(|next| {
                    let mut new_path = path.clone();
                    new_path.push_back(next.clone());
                    new_path
                }).collect_vec()
            }
        }
    }
}

fn is_loop(path: &LinkedList<Point>) -> bool {
    let last = path.back().unwrap();
    let pos = path.iter().find_position(|p| *p == last).unwrap().0;
    return pos != path.len() - 1;
}

fn filter_bidirectional(map: HashMap<Point, Vec<Point>>) -> HashMap<Point, Vec<Point>>
{
    map.iter()
        .map(|(loc, edges)| {
            let bidirectional_edges = edges.iter().filter_map(|e|
                if map.get(e).map_or_else(|| false, |ed| ed.contains(loc)) {
                    Some(e.clone())
                } else {
                    None
                }
            ).collect_vec();
            (loc.clone(), bidirectional_edges)
        })
        .collect::<HashMap<Point, Vec<Point>>>()
}

// General
fn parse_input(file_content: &String) -> HashMap<Point, Vec<Point>>
{
    file_content
        .lines()
        .enumerate()
        .take_while(|(_, l)| !l.is_empty())
        .flat_map(|(row, line)| {
            let i16_row = i16::try_from(row).unwrap();
            line.chars().enumerate().map(move |(col, c)| {
                let i16_col = i16::try_from(col).unwrap();
                let location = Point::new(i16_row, i16_col);
                let north = Point::new(i16_row - 1, i16_col);
                let south = Point::new(i16_row + 1, i16_col);
                let west = Point::new(i16_row, i16_col - 1);
                let east = Point::new(i16_row, i16_col + 1);

                (location, match c {
                    '|' => vec![north, south],
                    '-' => vec![east, west],
                    'L' => vec![north, east],
                    'J' => vec![north, west],
                    '7' => vec![south, west],
                    'F' => vec![south, east],
                    'S' => vec![north, south, west, east],
                    _ => vec![]
                })
            })
        })
        .collect::<HashMap<Point, Vec<Point>>>()
}




