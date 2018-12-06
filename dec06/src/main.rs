extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};

struct Coord {
    id: usize,
    x: i32,
    y: i32
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let x_dist = max(x1, x2) - min(x1, x2);
    let y_dist = max(y1, y2) - min(y1, y2);
    x_dist + y_dist
}

fn find_closest(x: i32, y: i32, coords: &Vec<Coord>) -> Option<usize> {
    let mut dist_vec: Vec<(usize, i32)> = coords.iter()
        .map(|coord| (coord.id, manhattan_distance(x, y, coord.x, coord.y)))
        .collect();

    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    if dist_vec[0].1 == dist_vec[1].1 {
        None    // There are two or more at the same close distance
    } else {
        Some(dist_vec[0].0) // Return the closest id
    }
}

fn main() {
    let input = include_str!("input");
    let limit_distance = 10000;

    // let input = include_str!("test");
    // let limit_distance = 32;

    let reg = Regex::new(r"(\d+), (\d+)").unwrap();

    let mut coords = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..lines.len() {
        let line = lines[i];
        let c = reg.captures(line).unwrap();
        coords.push(Coord {
            id: i,
            x: c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: c.get(2).unwrap().as_str().parse::<i32>().unwrap()
        });
    };

    // Find boundaries
    let left_x = coords.iter().min_by_key(|c| c.x).unwrap().x;
    let right_x = coords.iter().max_by_key(|c| c.x).unwrap().x;
    let top_y = coords.iter().min_by_key(|c| c.y).unwrap().y;
    let bottom_y = coords.iter().max_by_key(|c| c.y).unwrap().y;

    // Investigate the area
    let mut id_closest: HashMap<usize, u32> = HashMap::new();
    let mut infinite_ids = HashSet::new();
    let mut part2_all_locations_below_limit_to_all = 0;
    for y in top_y..=bottom_y {
        for x in left_x..=right_x {
            if let Some(closest) = find_closest(x, y, &coords) {
                if x == left_x || x == right_x || y == top_y || y == bottom_y {
                    infinite_ids.insert(closest);
                }
                let count = id_closest.get(&closest).unwrap_or(&0u32) + 1;
                id_closest.insert(closest, count);
            }

            // Part 2
            let sum_dist_to_all = coords.iter()
                .map(|c| manhattan_distance(x, y, c.x, c.y))
                .sum::<i32>();
            if sum_dist_to_all < limit_distance {
                part2_all_locations_below_limit_to_all += 1;
            }
        }
    }

    let biggest_finite = id_closest.iter()
        .filter(|v| !infinite_ids.contains(v.0))
        .max_by_key(|v| v.1).unwrap();

    println!("part1: Id {} has biggest region of {}", biggest_finite.0, biggest_finite.1);
    println!("part2: {}", part2_all_locations_below_limit_to_all);
}
