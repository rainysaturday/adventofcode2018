extern crate regex;
use regex::Regex;
use std::collections::{HashSet, BinaryHeap};

fn print(left_x: u32, right_x: u32, top_y: u32, bottom_y: u32, clay: &HashSet<(u32, u32)>, water: &HashSet<(u32, u32)>) {
    for y in top_y..=bottom_y {
        print!("{:04} ", y);
        for x in left_x..=right_x {
            if clay.contains(&(x, y)) {
                print!("#");
            } else if water.contains(&(x, y)) {
                print!("~");
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

fn add_water(water: &mut HashSet<(u32, u32)>, clay: &HashSet<(u32, u32)>, left_x: u32, right_x: u32, top_y: u32, bottom_y: u32) {
    let mut falling: Vec<(u32, u32)> = Vec::new();
    falling.push((500, 0));
    loop {
        // print(left_x, right_x, top_y, bottom_y, clay, &water);
        // println!("Falling: {:?}", falling);
        falling.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        if let Some(curr) = falling.pop() {
            println!("Visiting {:?}", curr);
            if curr.1 <= bottom_y {
                if !clay.contains(&curr) {
                    water.insert(curr.clone());

                    if water.contains(&(curr.0, curr.1 + 1)) {  // Water
                        println!("Visiting water");
                        // Check if pool of water below
                        let mut water_to_clay = 0;
                        for offset in 0.. {
                            let pos = (curr.0 - offset, curr.1 + 1);
                            if !water.contains(&pos) {
                                if clay.contains(&pos) {
                                    water_to_clay += 1;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                        for offset in 0.. {
                            let pos = (curr.0 + offset, curr.1 + 1);
                            if !water.contains(&pos) {
                                if clay.contains(&pos) {
                                    water_to_clay += 1;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }

                        if water_to_clay == 2 {
                            // Then we should expand on top of water
                            if !clay.contains(&(curr.0 - 1, curr.1)) && !water.contains(&(curr.0 - 1, curr.1)) {
                                falling.push((curr.0 - 1, curr.1));
                            }
                            if !clay.contains(&(curr.0 + 1, curr.1)) && !water.contains(&(curr.0 + 1, curr.1)) {
                                falling.push((curr.0 + 1, curr.1));
                            }
                        } else {
                            // Do nothing since water is streaming over some edge
                        }
                    } else if clay.contains(&(curr.0, curr.1 + 1)) {   // Clay below
                        // expand to sides if empty
                        if !clay.contains(&(curr.0 - 1, curr.1)) && !water.contains(&(curr.0 - 1, curr.1)) {
                            falling.push((curr.0 - 1, curr.1));
                        }
                        if !clay.contains(&(curr.0 + 1, curr.1)) && !water.contains(&(curr.0 + 1, curr.1)) {
                            falling.push((curr.0 + 1, curr.1));
                        }
                    } else if !water.contains(&(curr.0 , curr.1 + 1)) {    // If no water below, keep falling
                        if curr.1 < bottom_y {
                            // keep falling
                            falling.push((curr.0 , curr.1));
                            falling.push((curr.0 , curr.1 + 1));
                        }
                    }
                }
            }
        } else {
           break;
        }
    }
}

fn part2(clay: &HashSet<(u32, u32)>, water: &HashSet<(u32, u32)>) {
    let mut still_water = HashSet::new();
    let mut still_size = 0;
    let mut moving_size = 0;
    loop {
        // println!("Found water at {:?}", still_water);
        for water_pos in water {
            if still_water.contains(&water_pos) {
                continue;
            }

            let mut is_still = true;

            let below_pos = (water_pos.0, water_pos.1 + 1);
            if !(clay.contains(&below_pos) || still_water.contains(&below_pos)) {
                is_still = false;
            }

            if is_still {
                // Check holds water to the right
                for offset in 1.. {
                    let next_pos = (water_pos.0 + offset, water_pos.1);
                    let below_next_pos = (water_pos.0 + offset, water_pos.1 + 1);
                    if !water.contains(&next_pos) {
                        if clay.contains(&next_pos) {
                            // Found one edge, possibly still
                            break;
                        } else {
                            is_still = false;
                            // println!("Pos {:?} is moving since no wall", next_pos);
                            break;
                        }
                    } else {
                        // Should be clay or still water below since we have water
                        if clay.contains(&below_next_pos) || still_water.contains(&below_next_pos) {
                            // Continue, since it might be still
                        } else {
                            is_still = false;
                            // println!("Pos {:?} is moving since no ground at {:?}", next_pos, below_next_pos);
                            break;
                        }
                    }
                }
            }
            // println!("For pos {:?} is_still {} to the right", water_pos, is_still);
            if is_still {
                // Check holds water to the left
                for offset in 1.. {
                    let next_pos = (water_pos.0 - offset, water_pos.1);
                    let below_next_pos = (water_pos.0 - offset, water_pos.1 + 1);
                    if !water.contains(&next_pos) {
                        if clay.contains(&next_pos) {
                            // Found one edge, possibly still
                            break;
                        } else {
                            is_still = false;
                            break;
                        }
                    } else {
                        // Should be clay or still water below since we have water
                        if clay.contains(&below_next_pos) || still_water.contains(&below_next_pos) {
                            // Continue, since it might be still
                        } else {
                            is_still = false;
                            break;
                        }
                    }
                }
            }
            if is_still {
                still_water.insert(water_pos);
            }
        }

        if still_size == still_water.len() {
            println!("No more water to go through");
            break;
        }
        still_size = still_water.len();
    }
    println!("Part2: still water: {}", still_water.len());
}

fn part1(clay: &HashSet<(u32, u32)>) {
    let left_x = clay.iter().min_by_key(|(x, y)| x).unwrap().0;
    let right_x = clay.iter().max_by_key(|(x, y)| x).unwrap().0;
    let top_y = clay.iter().min_by_key(|(x, y)| y).unwrap().1;
    let bottom_y = clay.iter().max_by_key(|(x, y)| y).unwrap().1;

    let mut water: HashSet<(u32, u32)> = HashSet::new();
    // print(left_x, right_x, top_y, bottom_y, clay, &water);
    loop {
        let water_size = water.len();
        add_water(&mut water, &clay, left_x - 1, right_x + 1, 0, bottom_y);
        if water_size == water.len() {
            break;
        }
    }
    print(left_x, right_x, top_y, bottom_y, clay, &water);
    println!("Part1: water can reach : {}", water.len() - top_y as usize);
    part2(clay, &water);
}

fn main() {
    let input = include_str!("input");
    // let input = include_str!("test");

    let xy_r = Regex::new(r"x=(\d+).*y=(.*)").unwrap();
    let yx_r = Regex::new(r"y=(\d+).*x=(.*)").unwrap();

    let mut clay: HashSet<(u32, u32)> = HashSet::new();
    for line in input.lines() {
        let mut from_x = 0;
        let mut and_to_x = 0;
        let mut from_y = 0;
        let mut and_to_y = 0;

        if let Some(xyc) = xy_r.captures(line) {
            from_x = xyc.get(1).unwrap().as_str().parse::<u32>().unwrap();
            and_to_x = from_x;
            let yr = xyc.get(2).unwrap().as_str();
            if yr.contains("..") {
                let mut parts = yr.split("..");
                from_y = parts.next().unwrap().parse::<u32>().unwrap();
                and_to_y = parts.next().unwrap().parse::<u32>().unwrap();
            } else {
                from_y = yr.parse::<u32>().unwrap();
                and_to_y = from_y;
            }
        } else if let Some(yxc) = yx_r.captures(line) {
            from_y = yxc.get(1).unwrap().as_str().parse::<u32>().unwrap();
            and_to_y = from_y;
            let xr = yxc.get(2).unwrap().as_str();
            if xr.contains("..") {
                let mut parts = xr.split("..");
                from_x = parts.next().unwrap().parse::<u32>().unwrap();
                and_to_x = parts.next().unwrap().parse::<u32>().unwrap();
            } else {
                from_x = xr.parse::<u32>().unwrap();
                and_to_x = from_x;
            }
        } else {
            panic!("Unmatched line: {}", line);
        }

        for x in from_x..=and_to_x {
            for y in from_y..=and_to_y {
                clay.insert((x, y));
            }
        }
    }

    part1(&clay);
}
