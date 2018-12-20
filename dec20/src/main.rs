use std::collections::{HashMap, VecDeque};
use std::cmp::min;

fn find_closing(offset: usize, input: &str) -> Option<usize> {
    let mut open = 0;
    for (pos, c) in input.chars().enumerate().skip(offset) {
        match c {
            '(' => open += 1,
            ')' => {
                open -= 1;
                if open == 0 {
                    return Some(pos);
                }
            },
            _ => ()
        };
    }
    None
}

fn find_on_same_level(ch: char, input: &str) -> Vec<usize> {
    let mut option_level = 0;
    let mut res = Vec::new();
    for (pos, c) in input.chars().enumerate() {
        if c == ch && option_level == 0 { 
            res.push(pos); 
        }
        match c {
            '(' => option_level += 1,
            ')' => option_level -= 1,
            _ => (),
        }
    }
    res
}

fn visit_str(steps: &str, start_pos: (i32, i32), map: &mut HashMap<(i32, i32), char>) -> (i32, i32) {
    // println!("Visiting {:?}", start_pos);
    let mut x = start_pos.0;
    let mut y = start_pos.1;
    for dir in steps.chars() {
        match dir {
            'N' => { map.insert((x, y - 1), '|'); y -= 2; },
            'S' => { map.insert((x, y + 1), '|'); y += 2; },
            'W' => { map.insert((x - 1, y), '|'); x -= 2; },
            'E' => { map.insert((x + 1, y), '|'); x += 2; },
            _ => panic!("dir {} is not valid", dir)
        };
        map.insert((x, y), ' ');
    }
    (x, y)
}

fn expand(desc: &str, in_option: bool, pos: (i32, i32), map: &mut HashMap<(i32, i32), char>) {
    // println!("Visiting {}, starting from {:?}", desc, pos);
    // print_map(map, pos);
    if in_option {
        // If in option, always parse branches first
        let mut start = 0;
        let pipes = find_on_same_level('|', desc);
        if pipes.len() > 0 {
            for b_pos in &pipes {
                expand(&desc[start..*b_pos], false, pos, map);
                start = b_pos + 1;
            }
            // Push last
            expand(&desc[start..desc.len()], false, pos, map);
        } else {
            panic!("String {} is in option but does not contain any branches", desc);
        }
    } else {
        let start_options = find_on_same_level('(', desc);
        let mut curr_pos = pos;
        if start_options.len() > 0 {
            let mut after_last_close = 0;
            for option_start in start_options {
                if option_start > after_last_close {
                    // Visit things until option start
                    curr_pos = visit_str(&desc[after_last_close..option_start], curr_pos, map);
                }

                let close_pos = find_closing(option_start, desc).expect("Should have closing");
                after_last_close = close_pos + 1;

                // Expand option branch
                expand(&desc[option_start+1..close_pos], true, curr_pos, map);
            }

            // Finally visit last part if any
            if after_last_close < desc.len() {
                visit_str(&desc[after_last_close..desc.len()], curr_pos, map);
            }
        } else {
            // No option and not in option, just push the directions, visit
            visit_str(&desc[0..desc.len()], pos, map);
        }
    }
}

fn shortest_path_furthest_away(map: &HashMap<(i32, i32), char>) -> HashMap<(i32, i32), usize> {
    let mut rooms_cost: HashMap<(i32, i32), usize> = HashMap::new();

    let mut to_visit: VecDeque<(i32, i32, usize)> = VecDeque::new();
    to_visit.push_back((0, 0, 0));
    while let Some(state) = to_visit.pop_back() {
        let pos = (state.0, state.1);
        let cost = state.2;
        if !map.contains_key(&pos) {
            continue;
        }

        let mut already_visited = false;
        if rooms_cost.contains_key(&pos) {
            already_visited = true;
        }
        let existing = *rooms_cost.get(&pos).unwrap_or(&10000000);
        rooms_cost.insert(pos, min(existing, cost));

        if !already_visited || cost < existing {
            if map.contains_key(&(pos.0 + 1, pos.1)) { to_visit.push_back((state.0 + 2, state.1, state.2 + 1)) };
            if map.contains_key(&(pos.0 - 1, pos.1)) { to_visit.push_back((state.0 - 2, state.1, state.2 + 1)) };
            if map.contains_key(&(pos.0, pos.1 + 1)) { to_visit.push_back((state.0, state.1 + 2, state.2 + 1)) };
            if map.contains_key(&(pos.0, pos.1 - 1)) { to_visit.push_back((state.0, state.1 - 2, state.2 + 1)) };
        }
    }

    return rooms_cost;
}

fn print_map(map: &HashMap<(i32, i32), char>, current_pos: (i32, i32)) {
    let left_x = map.keys().min_by_key(|(x, _)| x).unwrap().0 - 1;
    let right_x = map.keys().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let top_y = map.keys().min_by_key(|(_, y)| y).unwrap().1 - 1;
    let bottom_y = map.keys().max_by_key(|(_, y)| y).unwrap().1 + 1;
    for y in top_y..=bottom_y {
        print!("{:03} ", y);
        for x in left_x..=right_x {
            if let Some(_c) = map.get(&(x, y)) {
                if x == current_pos.0  && y == current_pos.1 {
                    print!("X");
                } else {
                    // print!("{}", _c);
                    print!(" ");
                }
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn main() {
    // let input = include_str!("mytest").lines().next().unwrap();    // Borde ge 18
    // let input = include_str!("test").lines().next().unwrap();    // Borde ge 18
    // let input = include_str!("test2").lines().next().unwrap();   // Borde ge 23
    // let input = include_str!("test3").lines().next().unwrap();   // Borde ge 31
    let input = include_str!("input").lines().next().unwrap();

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    map.insert((0, 0), '.');
    expand(&input[1..input.len()-1], false, (0, 0), &mut map);

    // print_map(&map, (0, 0));
    let rooms_cost = shortest_path_furthest_away(&map);
    println!("Part1: Shortest path to room furthest away {}", rooms_cost.values().max().unwrap());
    println!("Part2: Num shortest paths to rooms at least 1000 doors away {}", rooms_cost.values().filter(|cost| *cost >= &1000).count());

}
