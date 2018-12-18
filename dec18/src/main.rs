use std::collections::{HashSet, HashMap};

fn look_around(pos: &(usize, usize), map: &HashMap<(usize, usize), char>, open_adj: &mut u32, trees_adj: &mut u32, lumber_adj: &mut u32) {
    let y_start = if pos.1 == 0 { 0 } else { pos.1 - 1 }; 
    let x_start = if pos.0 == 0 { 0 } else { pos.0 - 1 }; 
    for y in y_start..=pos.1+1 {
        for x in x_start..=pos.0+1 {
            if x == pos.0 && y == pos.1 {
                continue;
            } else if let Some(acre) = map.get(&(x, y)) {
                match acre {
                    '.' => *open_adj += 1,
                    '|' => *trees_adj += 1,
                    '#' => *lumber_adj += 1,
                    other => panic!("Not recognized {}", other),
                };
            }
        }
    }
}

fn print_forest(map: &HashMap<(usize, usize), char>) -> String {
    let left_x = map.keys().min_by_key(|pos| pos.0).unwrap().0;
    let right_x = map.keys().max_by_key(|pos| pos.0).unwrap().0;
    let top_y = map.keys().min_by_key(|pos| pos.1).unwrap().1;
    let bottom_y = map.keys().max_by_key(|pos| pos.1).unwrap().1;
    let mut forest = String::new();
    for y in top_y..=bottom_y {
        for x in left_x..=right_x {
            if let Some(acre) = map.get(&(x, y)) {
                forest.push(*acre);
            } else {
                forest.push(' ');
            }
        }
        forest.push('\n');
    }
    return forest;
}

fn do_forest_things(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    let mut next: HashMap<(usize, usize), char> = HashMap::new();
    for (pos, acre) in map {
        let mut trees_adj = 0;
        let mut open_adj = 0;
        let mut lumber_adj = 0;
        look_around(pos, map, &mut open_adj, &mut trees_adj, &mut lumber_adj);
        match acre {
            '.' => if trees_adj >= 3 {
                next.insert(*pos, '|');
            } else {
                next.insert(*pos, *acre);
            },
            '|' => if lumber_adj >= 3 {
                next.insert(*pos, '#');
            } else {
                next.insert(*pos, *acre);
            },
            '#' => if trees_adj >= 1 && lumber_adj >= 1 {
                next.insert(*pos, *acre);
            } else {
                next.insert(*pos, '.');
            },
            _ => panic!("Hum"),
        }
    }

    return next;
}

fn main() {
    // let input = include_str!("test");
    let input = include_str!("input");


    let mut map: HashMap<(usize, usize), char> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, acre) in line.chars().enumerate() {
            map.insert((x, y), acre);
        }
    }
    let mut forests: Vec<(String, u64)> = Vec::new();
    let mut seen_forests: HashSet<String> = HashSet::new();
    for minute in 1.. {
        let forest = print_forest(&map);
        if seen_forests.contains(&forest) {
            println!("Seen this forest before after {} minutes", minute);
            break;
        }

        map = do_forest_things(&map);
        let trees = map.values().filter(|acre| **acre == '|').count() as u64;
        let lumber = map.values().filter(|acre| **acre == '#').count() as u64;
        let res_value = trees*lumber;
        if minute == 10  {
            println!("After {} minutes we have {} trees and {} lumberyards, giving resource value of {}", minute, trees, lumber, res_value);
        }

        seen_forests.insert(forest.clone());
        forests.push((forest, res_value));
    }

    // Figure out how big loop is
    let mut loop_start = 0;
    for i in 2..forests.len() - 1 {
        if forests[forests.len() - i].1 == forests[forests.len() - 1].1 {
            loop_start = forests.len() - i;
            break;
        }
    }
    println!("Loop start at {}", loop_start);
    let last_forest = forests.len() - 1;
    // println!("Forest {} with value {}\n{}", loop_start, forests[loop_start].1, forests[loop_start].0);
    // println!("Forest {} with value {}\n{}", last_forest, forests[last_forest].1, forests[last_forest].0);

    // Value of 1_000_000_000 would be
    let loop_size = last_forest - loop_start;
    let onebill_rel_pos = (1_000_000_000 - loop_start - 1) % loop_size;
    println!("value of forest on pos 1_000_000_000 = {}", forests[loop_start + onebill_rel_pos].1);
}
