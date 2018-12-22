use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Equipment {
    Torch,
    Climbing,
    Neither
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchState {
    pos: (u32, u32),
    time_spent: u32,
    equipment_switches: u32,
    current_equiped: Equipment,
}

impl SearchState {
    fn new(pos: (u32, u32), time_spent: u32, equipment_switches: u32, current_equiped: Equipment) -> SearchState {
        SearchState {
            pos: pos,
            time_spent: time_spent,
            equipment_switches: equipment_switches,
            current_equiped: current_equiped,
        }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        other.time_spent.cmp(&self.time_spent)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn next_search(from: &SearchState, x_offset: i32, y_offset: i32, erosion_map: &HashMap<(u32, u32), u32>) -> Vec<SearchState> {
    let new_x = from.pos.0 as i32 + x_offset;
    let new_y = from.pos.1 as i32 + y_offset;
    let mut states = Vec::new();
    if new_x < 0 || new_y < 0 {
        return states
    }

    let new_pos = (new_x as u32, new_y as u32);
    let new_time_spent = from.time_spent + 1;  // Moved

    // Need to switch gear?
    if let Some(erosion_level) = erosion_map.get(&new_pos) {
        match erosion_level % 3 {
            0 => {  // Rocky, cannot use neither
                match from.current_equiped {
                    Equipment::Climbing => { 
                        states.push(SearchState::new(new_pos, new_time_spent, from.equipment_switches, Equipment::Climbing));
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Torch));
                    },
                    Equipment::Torch => { 
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Climbing));
                        states.push(SearchState::new(new_pos, new_time_spent, from.equipment_switches, Equipment::Torch));
                    },
                    Equipment::Neither => { 
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Climbing));
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Torch));
                    },
                };
            },
            1 => { // Wet, cannot use torch
                match from.current_equiped {
                    Equipment::Climbing => { 
                        states.push(SearchState::new(new_pos, new_time_spent, from.equipment_switches, Equipment::Climbing));
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Neither));
                    },
                    Equipment::Torch => { 
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Climbing));
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Neither));
                    },
                    Equipment::Neither => { 
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Climbing));
                        states.push(SearchState::new(new_pos, new_time_spent, from.equipment_switches, Equipment::Neither));
                    },
                };
            },
            2 => { // Narrow, cannot use climbing
                match from.current_equiped {
                    Equipment::Climbing => { 
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Torch));
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Neither));
                    },
                    Equipment::Torch => { 
                        states.push(SearchState::new(new_pos, new_time_spent, from.equipment_switches, Equipment::Torch));
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Neither));
                    },
                    Equipment::Neither => { 
                        states.push(SearchState::new(new_pos, new_time_spent + 7, from.equipment_switches + 1, Equipment::Torch));
                        states.push(SearchState::new(new_pos, new_time_spent, from.equipment_switches, Equipment::Neither));
                    },
                };
            },
            other => panic!("Unknown type: {}", other)
        };
    }

    states
}


fn find_fastest(erosion_map: &HashMap<(u32, u32), u32>, target: (u32, u32)) -> Option<SearchState> {
    let mut to_search: BinaryHeap<SearchState> = BinaryHeap::new();
    let mut cost_to_pos: HashMap<(u32, u32), u32> = HashMap::new();

    to_search.push(SearchState::new((0, 0), 0, 0, Equipment::Torch));
    let mut investigated = 0;
    while let Some(search) = to_search.pop() {
        // println!("Investigated {}, state: {:?}", investigated, search.pos);
        investigated += 1;
        // println!("searching {:?}\nto search: {:?}", search, to_search);

        if let Some(cost_to_here) = cost_to_pos.get(&search.pos) {
            if cost_to_here <= &search.time_spent {
                continue;
            }
        }
        cost_to_pos.insert(search.pos.clone(), search.time_spent);
        
        if search.pos == target {
            println!("Reached target with search state {:?}", search);
            // return Some(search);
            continue;
        }

        // Search more
        let mut states = next_search(&search, -1, 0, erosion_map);
        states.extend(next_search(&search, 0, -1, erosion_map));
        states.extend(next_search(&search, 1, 0, erosion_map));
        states.extend(next_search(&search, 0, 1, erosion_map));
        for state in states {
            to_search.push(state);
        }
    }


    // Print cost_map
    for y in 0..=target.1 {
        print!("{:03}", y);
        for x in 0..=target.0 {
            print!(" {:02}", cost_to_pos.get(&(x, y)).expect("calculated pos"));
            match erosion_map.get(&(x, y)).expect("erosion map") % 3 {
                0 => print!("."),
                1 => print!("~"),
                2 => print!("|"),
                other => panic!("Unknown {}", other),
            };
        }
        println!();
    }

    None
}


fn geo_index(x: u32, y: u32, target: (u32, u32), erosions: &HashMap<(u32, u32), u32>) -> u32 {
    if (x == 0 && y == 0) ||
        (x == target.0 && y == target.1) {
        return 0;
    }
    else if y == 0 {
        return x * 16807;
    } else if x == 0 {
        return y * 48271;
    } else {
        return erosions.get(&(x - 1, y)).unwrap() * erosions.get(&(x, y - 1)).unwrap();
    }
}

fn erosion_level(depth: u32, geo_index: u32) -> u32 {
    (depth + geo_index) % 20183
}

fn part1(depth: u32, target: (u32, u32)) {
    let mut risk_level = 0;
    let mut erosions = HashMap::new();
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            let geo_index = geo_index(x, y, target, &erosions);
            let erosion = erosion_level(depth, geo_index);
            risk_level += erosion % 3;
            erosions.insert((x, y), erosion);
        }
    }
    println!("Part1: Risklevel {}", risk_level);
}

fn part2(depth: u32, target: (u32, u32)) {
    let mut risk_level = 0;
    let mut erosions = HashMap::new();
    for y in 0..=target.1 + 100 {
        for x in 0..=target.0 + 100 {       // 100 extra should be enough to find cheap way
            let geo_index = geo_index(x, y, target, &erosions);
            let erosion = erosion_level(depth, geo_index);
            risk_level += erosion % 3;
            erosions.insert((x, y), erosion);
        }
    }

    let fastest = find_fastest(&erosions, target);

    println!("Part2: Risklevel {}, fastest {:?}", risk_level, fastest);
}

fn main() {
    // Test
    let depth = 510;
    let target = (10, 10);
    // Input
    // let depth = 9171;
    // let target = (7, 721);


    part1(depth, target);
    part2(depth, target);
}
