extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

type Pos = (i32, i32, i32);

fn delta(a: i32, b: i32) -> u32 {
    let min_val = min(a, b);
    let max_val = max(a, b);
    if max_val < 0 {
        return ((min_val - max_val) * -1) as u32
    } else {
        if min_val < 0 {
            return max_val as u32 + (min_val * -1) as u32;
        } else {
            return max_val as u32 - min_val as u32;
        }
    }
}

fn manhattan_distance(a: &Pos, b: &Pos) -> u32 {
    delta(a.0, b.0) + delta(a.1, b.1) + delta(a.2, b.2)
}

fn num_inrange(bot: &(Pos, u32), bots: &HashMap<Pos, u32>) -> u32 {
    let mut in_range = 0;

    for b in bots {
        if manhattan_distance(&bot.0, &b.0) <= bot.1 {
            in_range += 1;
        }
    }

    return in_range;
}

#[derive(Debug)]
struct RegionState {
    upper_far_left: Pos,
    lower_close_right: Pos,
    level: u32,
    calc_contains: Option<u32>
}

impl RegionState {
    fn new(upper_far_left: Pos, lower_close_right: Pos, level: u32) -> RegionState {
        RegionState {
            upper_far_left: upper_far_left,
            lower_close_right: lower_close_right,
            level: level,
            calc_contains: None
        }
    }
}

fn oct_divide(region: &RegionState) -> Vec<RegionState> {
    let mut div = Vec::new();
    let width = delta(region.upper_far_left.0, region.lower_close_right.0) as i32;
    let height = delta(region.upper_far_left.1, region.lower_close_right.1) as i32;
    let depth = delta(region.upper_far_left.2, region.lower_close_right.2) as i32;

    let hw = width / 2;
    let hh = height / 2;
    let hd = depth / 2;

    if hw == 0 && hh == 0 && hd == 0 {
        div.push(RegionState::new(
            (region.upper_far_left.0, region.upper_far_left.1, region.upper_far_left.2),
            (region.upper_far_left.0, region.upper_far_left.1, region.upper_far_left.2),
            region.level + 1
        ));
        return div;
    }

    // Upper far left
    div.push(RegionState::new(
        (region.upper_far_left.0, region.upper_far_left.1, region.upper_far_left.2),
        (region.lower_close_right.0 - hw, region.lower_close_right.1 - hh, region.lower_close_right.2 - hd),
        region.level + 1
    ));
    // Upper far right
    div.push(RegionState::new(
        (region.upper_far_left.0 + hw, region.upper_far_left.1, region.upper_far_left.2),
        (region.lower_close_right.0, region.lower_close_right.1 - hh, region.lower_close_right.2 - hd),
        region.level + 1
    ));
    // Upper close left
    div.push(RegionState::new(
        (region.upper_far_left.0, region.upper_far_left.1, region.upper_far_left.2 + hd),
        (region.lower_close_right.0 - hw, region.lower_close_right.1 - hh, region.lower_close_right.2),
        region.level + 1
    ));
    // Upper close right
    div.push(RegionState::new(
        (region.upper_far_left.0 + hw, region.upper_far_left.1, region.upper_far_left.2 + hd),
        (region.lower_close_right.0, region.lower_close_right.1 - hh, region.lower_close_right.2),
        region.level + 1
    ));

    // Lower far left
    div.push(RegionState::new(
        (region.upper_far_left.0, region.upper_far_left.1 + hh, region.upper_far_left.2),
        (region.lower_close_right.0 - hw, region.lower_close_right.1, region.lower_close_right.2 - hd),
        region.level + 1
    ));
    // Lower far right
    div.push(RegionState::new(
        (region.upper_far_left.0 + hw, region.upper_far_left.1 + hh, region.upper_far_left.2),
        (region.lower_close_right.0, region.lower_close_right.1, region.lower_close_right.2 - hd),
        region.level + 1
    ));
    // Lower close left
    div.push(RegionState::new(
        (region.upper_far_left.0, region.upper_far_left.1 + hh, region.upper_far_left.2 + hd),
        (region.lower_close_right.0 - hw, region.lower_close_right.1, region.lower_close_right.2),
        region.level + 1
    ));
    // Lower close right
    div.push(RegionState::new(
        (region.upper_far_left.0 + hw, region.upper_far_left.1 + hh, region.upper_far_left.2 + hd),
        (region.lower_close_right.0, region.lower_close_right.1, region.lower_close_right.2),
        region.level + 1
    ));

    div
}

type Pos2D = (i32, i32);

fn closest_in_2d_plane(top_left: Pos2D, bottom_right: Pos2D, search: Pos2D) -> Pos2D {
    // If search is in the plane, just return it
    if search.0 >= top_left.0 &&
        search.1 >= top_left.1 &&
        search.0 < bottom_right.0 &&
        search.1 < bottom_right.1 {
            return search;
    }

    let mut new_pos = search;
    // Adjust so that it is in the closest position
    if new_pos.0 <= top_left.0 {
        new_pos.0 = top_left.0;
    }
    if new_pos.1 <= top_left.1 {
        new_pos.1 = top_left.1;
    }

    if new_pos.0 >= bottom_right.0 {
        new_pos.0 = bottom_right.0;
    }
    if new_pos.1 >= bottom_right.1 {
        new_pos.1 = bottom_right.1;
    }

    new_pos
}

fn calculate_contains(state: &RegionState, bots: &HashMap<Pos, u32>) -> u32 {
    let mut count = 0;

    for (bot_pos, bot_range) in bots {
        // if the bot position is within the box, then add it
        if bot_pos.0 >= state.upper_far_left.0 &&
            bot_pos.1 >= state.upper_far_left.1 &&
            bot_pos.2 >= state.upper_far_left.2 &&
            bot_pos.0 <= state.lower_close_right.0 &&
            bot_pos.1 <= state.lower_close_right.1 &&
            bot_pos.2 <= state.lower_close_right.2 {
            
            count += 1;
        } else {
            // Position is not in the box, check if the range is enough to get some part of it inside
            let mut is_within = false;

            // Check left and right
            {
                let closest2d = closest_in_2d_plane(
                    (state.upper_far_left.2, state.upper_far_left.1), 
                    (state.lower_close_right.2, state.lower_close_right.1),
                    (bot_pos.2, bot_pos.1)
                    );
                let closest3dleft = (state.upper_far_left.0, closest2d.1, closest2d.0);
                let closest3dright = (state.lower_close_right.0, closest2d.1, closest2d.0);
                if manhattan_distance(bot_pos, &closest3dleft) <= *bot_range ||
                   manhattan_distance(bot_pos, &closest3dright) <= *bot_range {
                    is_within = true;
                }
            }

            // Check top and bottom
            if !is_within {
                let closest2d = closest_in_2d_plane(
                    (state.upper_far_left.0, state.upper_far_left.2), 
                    (state.lower_close_right.0, state.lower_close_right.2),
                    (bot_pos.0, bot_pos.2)
                    );
                let closest3dtop = (closest2d.0, state.upper_far_left.1, closest2d.1);
                let closest3dbottom = (closest2d.0, state.lower_close_right.1, closest2d.1);
                if manhattan_distance(bot_pos, &closest3dtop) <= *bot_range ||
                   manhattan_distance(bot_pos, &closest3dbottom) <= *bot_range {
                    is_within = true;
                }
            }

            // Check back and front
            if !is_within {
                let closest2d = closest_in_2d_plane(
                    (state.upper_far_left.0, state.upper_far_left.1), 
                    (state.lower_close_right.0, state.lower_close_right.1),
                    (bot_pos.0, bot_pos.1)
                    );
                let closest3dback = (closest2d.0, closest2d.1, state.upper_far_left.2);
                let closest3dfront = (closest2d.0, closest2d.1, state.lower_close_right.2);
                if manhattan_distance(bot_pos, &closest3dback) <= *bot_range ||
                   manhattan_distance(bot_pos, &closest3dfront) <= *bot_range {
                    is_within = true;
                }
            }

            if is_within {
                count += 1;
            }
        }
    }

    count
}

fn hotspot(bots: &HashMap<Pos, u32>) -> Option<Pos> {
    let mut to_visit: Vec<RegionState> = Vec::new();
    to_visit.push(RegionState::new(
        (-200_000_000, -200_000_000, -200_000_000),
        ( 200_000_000,  200_000_000,  200_000_000),
        0
    ));
    // to_visit.push(RegionState::new(
    //     (0, 0, 0),
    //     ( 20,  20,  20),
    //     0,
    // ));

    while let Some(state) = to_visit.pop() {
        // let size = manhattan_distance(&state.upper_far, &state.lower_close);
        // println!("Visiting region of size {}, {:?}", size, state);
        if state.upper_far_left == state.lower_close_right {
            println!("Found position on level {}", state.level);
            return Some(state.upper_far_left);
        }

        // Subdivide
        to_visit.extend(oct_divide(&state));

        // Calculate contains
        for sub_state in to_visit.iter_mut() {
            if sub_state.calc_contains.is_none() {
                sub_state.calc_contains = Some(calculate_contains(&sub_state, bots));
            }
        }

        // Sort
        // println!("Presort: {:?}", to_visit);
        to_visit.sort_by(|a, b| {
            let a_contains = a.calc_contains.expect("Should be calculated");
            let b_contains = b.calc_contains.expect("Should be calculated");
            let a_size = manhattan_distance(&a.upper_far_left, &a.lower_close_right);
            let b_size = manhattan_distance(&b.upper_far_left, &b.lower_close_right);
            if a_contains != b_contains {
                return a_contains.cmp(&b_contains);
            } else {
                return b_size.cmp(&a_size);
            }
        });
        // println!("Postsort: {:?}", to_visit);
    }

    None
}

fn main() {
//    let input = include_str!("test");
    let input = include_str!("input");
//    let input = include_str!("test2");

    let bot_r = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();

    let mut bots: HashMap<Pos, u32> = HashMap::new();
    for line in input.lines() {
        let c = bot_r.captures(line).expect("Should match");
        let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let z = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let r = c.get(4).unwrap().as_str().parse::<u32>().unwrap();
        bots.insert((x, y, z), r);
    }

    let largest = bots.iter().max_by_key(|v| v.1).unwrap();
    println!("Bot with largest range: {:?}", largest);
    println!("Part1: num inrange: {}", num_inrange(&(*largest.0, *largest.1), &bots));

    let mut posses = HashSet::new();
    for (pos, _) in &bots {
        posses.insert(pos);
    }
    println!("{} bots, and {} unique positions", bots.len(), posses.len());

    let hot = hotspot(&bots).expect("should have found something");
    println!("Part2: hotspot pos {:?}, dist from 0,0,0 is {}", hot, manhattan_distance(&(0, 0, 0), &hot));
}
