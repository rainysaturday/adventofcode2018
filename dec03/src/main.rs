extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    // #1 @ 661,227: 29x11
    let reg = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    
    let mut claim_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut non_overlapping: HashSet<u32> = HashSet::new();

    for line in input.lines() {
        let caps = reg.captures(line).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let locx = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let locy = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let width = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let height = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

        let mut overlaps = false;
        for x in locx..locx+width {
            for y in locy..locy+height {
                let pos: u32 = (y * 1000) + x;
                if claim_map.contains_key(&pos) {
                    let mut claim_vec = claim_map.get_mut(&pos).unwrap();
                    claim_vec.push(id);
                    overlaps = true;
                    for claim in claim_vec {
                        non_overlapping.remove(claim);
                    }
                } else {
                    let claim_vec = vec![id];
                    claim_map.insert(pos, claim_vec);
                }
            }
        }
        if !overlaps {
            non_overlapping.insert(id);
        }
    }

    println!("part1, # overlapping squares: {}", claim_map.values().filter(|claims| claims.len() > 1).count());
    println!("part2, non-overlapping claims: {:?}", non_overlapping);
}
