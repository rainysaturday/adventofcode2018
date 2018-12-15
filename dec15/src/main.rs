use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, BinaryHeap};

fn reading_order(a: &Pos, b: &Pos) -> std::cmp::Ordering {
    if a.y == b.y {
        return a.x.partial_cmp(&b.x).unwrap();
    } else {
        return a.y.partial_cmp(&b.y).unwrap();
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Pos {
    x: usize, 
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x,
            y: y
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Unit {
    pos: Pos,
    hp: i32,
    team: char,
    acted: bool
}

impl Unit {
    fn new(x: usize, y: usize, team: char) -> Unit {
        Unit {
            pos: Pos::new(x, y),
            hp: 200,
            team: team,
            acted: false
        }
    }
}

fn print_map(walls: &HashSet<Pos>, all_units: &HashMap<Pos, Unit>) {
    let left_x = walls.iter().min_by_key(|v| v.x).unwrap().x;
    let right_x = walls.iter().max_by_key(|v| v.x).unwrap().x;
    let top_y = walls.iter().min_by_key(|v| v.y).unwrap().y;
    let bottom_y = walls.iter().max_by_key(|v| v.y).unwrap().y;
    for y in top_y..=bottom_y {
        print!("{:02} ", y);
        let mut ghp = Vec::new();
        let mut ehp = Vec::new();
        for x in left_x..=right_x {
            if walls.contains(&Pos::new(x, y)) {
                print!("#");
            } else if let Some(unit) = all_units.get(&Pos::new(x, y)) {
                print!("{}", unit.team);
                if unit.team == 'G' {
                    ghp.push(unit.hp);
                } else {
                    ehp.push(unit.hp);
                }
            } else {
                print!(" ");
            }
        }
        if ghp.len() > 0 {
            print!(" Goblins {:?}", ghp);
        }
        if ehp.len() > 0 {
            print!(" Elfs {:?}", ehp);
        }
        println!();
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DistState {
    x: usize,
    y: usize,
    cost: u32
}

impl Ord for DistState {
    fn cmp(&self, other: &DistState) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| {
                if self.y == other.y {
                    other.x.cmp(&self.x)
                } else {
                    other.y.cmp(&self.y)
                }
            })
    }
}

impl PartialOrd for DistState {
    fn partial_cmp(&self, other: &DistState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(from: &Pos, to: &Pos, walls: &HashSet<Pos>, other_units: &HashMap<Pos, Unit>) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(DistState {x: from.x, y: from.y, cost: 0u32});

    while let Some(state) = to_visit.pop() {
        let curr_pos = Pos::new(state.x, state.y);
        if visited.contains(&curr_pos) {
            continue;
        } else {
            visited.insert(curr_pos.clone());
        }
        if walls.contains(&curr_pos) || other_units.contains_key(&curr_pos) {
            continue;
        }

        if &curr_pos == to {
            return Some(state.cost);
        }

        // Add candidates
        to_visit.push(DistState { x: curr_pos.x - 1, y: curr_pos.y, cost: state.cost + 1 });
        to_visit.push(DistState { x: curr_pos.x + 1, y: curr_pos.y, cost: state.cost + 1 });
        to_visit.push(DistState { x: curr_pos.x, y: curr_pos.y - 1, cost: state.cost + 1 });
        to_visit.push(DistState { x: curr_pos.x, y: curr_pos.y + 1, cost: state.cost + 1 });
    }

    return None;
}

fn bad_guy_in_range(good_guy: &Unit, all_units: &HashMap<Pos, Unit>) -> Option<Unit> {
    // Manually sorted in reading order
    let adjacent = vec![
        Pos::new(good_guy.pos.x, good_guy.pos.y - 1),
        Pos::new(good_guy.pos.x - 1, good_guy.pos.y),
        Pos::new(good_guy.pos.x + 1, good_guy.pos.y),
        Pos::new(good_guy.pos.x, good_guy.pos.y + 1),
        ];
    let mut lowest_bad_guy = None;
    let mut lowest_hp = 201;
    for adj in adjacent {
        if let Some(unit) = all_units.get(&adj) {
            if unit.team != good_guy.team {
                if unit.hp < lowest_hp {
                    lowest_hp = unit.hp;
                    lowest_bad_guy = Some(unit.clone());
                }
            }
        }
    }
    lowest_bad_guy
}

fn try_attack(attacker: &Unit, all_units: &mut HashMap<Pos, Unit>, elf_power: i32) -> Option<Pos> {
    if let Some(mut bad_guy) = bad_guy_in_range(&attacker, &all_units) {
        bad_guy.hp -= if attacker.team == 'E' { elf_power } else { 3 };   // Do damage
        // println!("{:?}:{} attacked {:?}:{}", attacker, attacker.hp, bad_guy.pos, bad_guy.hp);
        if bad_guy.hp <= 0 {
            // println!("Bad guy at {:?} died", bad_guy.pos);
            all_units.remove(&bad_guy.pos);
        } else {
            all_units.insert(bad_guy.pos.clone(), bad_guy.clone());
        }
        return Some(bad_guy.pos.clone());
    }
    None
}

fn perform_unit(good_guy_pos: &Pos, 
    all_units: &mut HashMap<Pos, Unit>, 
    walls: &HashSet<Pos>,
    elf_power: i32) -> Option<Pos> {
    if !all_units.contains_key(&good_guy_pos) {
        // println!("Pos {:?} does no longer exist in {:?}, they likely already died", good_guy_pos, all_units);
        return None;
    }
    let mut good_guy = all_units.get(&good_guy_pos).expect("unit should exist").clone();
    all_units.remove(&good_guy.pos);

    // Manually sorted adjacent in reading order
    let adjacent = vec![
        Pos::new(good_guy.pos.x, good_guy.pos.y - 1),
        Pos::new(good_guy.pos.x - 1, good_guy.pos.y),
        Pos::new(good_guy.pos.x + 1, good_guy.pos.y),
        Pos::new(good_guy.pos.x, good_guy.pos.y + 1),
        ];

    // Populate target positions
    let mut target_pos = Vec::new();
    all_units.iter()
        .map(|(pos, unit)| unit)
        .filter(|unit| unit.team != good_guy.team)
        .for_each(|bad_guy| {
            target_pos.push(Pos::new(bad_guy.pos.x - 1, bad_guy.pos.y));
            target_pos.push(Pos::new(bad_guy.pos.x + 1, bad_guy.pos.y));
            target_pos.push(Pos::new(bad_guy.pos.x, bad_guy.pos.y - 1));
            target_pos.push(Pos::new(bad_guy.pos.x, bad_guy.pos.y + 1));
        });
    target_pos = target_pos.into_iter()
        .filter(|p| !walls.contains(&p) && !all_units.contains_key(&p))
        .collect();

    all_units.remove(&good_guy.pos);

    // Can we attack someone?
    let mut attacked = try_attack(&good_guy, all_units, elf_power);
    if attacked.is_none() && target_pos.len() > 0 {
        // Try to move instead
        let mut moves: Vec<(&Pos, &Pos, u32)> = adjacent.iter()
            .map(|adj| {
                let mut targets: Vec<(&Pos, u32)> = target_pos.iter().map(|p| (p, distance(&adj, &p, &walls, &all_units)))
                    .filter(|(p, dist)| dist.is_some())
                    .map(|(p, dist)| (p, dist.unwrap()))
                    .collect();
                targets.sort_by(|a, b| {
                    // Sort targets by distance, then on reading order
                    if a.1 != b.1 {
                        a.1.partial_cmp(&b.1).unwrap()
                    } else {
                        reading_order(a.0, b.0)
                    }
                });
                // println!("targets for {:?} = {:?}", adj, targets);
                
                if targets.len() > 0 {
                    return (adj, Some(targets[0].0), Some(targets[0].1));
                }
                return (adj, None, None);
            })
            .filter(|(_adj, _targ, dist)| dist.is_some())
            .map(|(adj, targ, dist)| (adj, targ.unwrap(), dist.unwrap()))
            .collect();

        // println!("MOVES unsorted: {:?}", moves);
        moves.sort_by(|a, b| {
            // First on distance, then on reading_order of targets
            if a.2 != b.2 {
                a.2.partial_cmp(&b.2).unwrap()
            } else {
                reading_order(a.1, b.1)
            }
        });
        // println!("MOVES sorted: {:?}", moves);


        // Move to the best position if there is one
        if moves.len() > 0 {
            // println!("Moving from {:?} to {:?}", good_guy.pos, moves);
            good_guy.pos = moves[0].0.clone();
            attacked = try_attack(&good_guy, all_units, elf_power);   // Attack directly after moving
        }
    }
    good_guy.acted = true;
    all_units.insert(good_guy.pos.clone(), good_guy.clone());
    return attacked;
}

fn simulate(input: &str, elf_power: i32, stop_on_elf_dead: bool) -> Option<i32> {
    let mut walls = HashSet::new();
    let mut all_units = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert(Pos::new(x, y));
            } else if c == 'G' || c == 'E' {
                all_units.insert(Pos::new(x, y), Unit::new(x, y, c));
            }
        }
    }

    let elf_count = all_units.iter().filter(|(_, unit)| unit.team == 'E').count();

    for turn in 0.. {
        // println!("Turn: {}", turn);
        // print_map(&walls, &all_units);
        let mut todo_units: Vec<Pos> = all_units.iter().map(|(p, _)| p.clone()).collect();
        all_units.values_mut().for_each(|unit| unit.acted = false);
        todo_units.sort_by(reading_order);
        for i in 0..todo_units.len() {
            let unit_pos = &todo_units[i];
            if let Some(unit) = all_units.get(&unit_pos) {
                if unit.acted {
                    continue;
                }
            }

            if let Some(attacked) = perform_unit(unit_pos, &mut all_units, &walls, elf_power) {
                if stop_on_elf_dead {
                    let current_elfs_alive = all_units.iter().filter(|(_, unit)| unit.team == 'E').count();
                    if elf_count != current_elfs_alive {
                        println!("Elf died, don't contine with {}", elf_power);
                        return None;
                    }
                }

                let team = all_units.values().next().unwrap().team;
                if all_units.values().all(|unit| unit.team == team) {
                    let completed_this = if i == todo_units.len() - 1 { 1 } else { 0 };
                    println!("Game ended after {} turns: \n{:?}", turn, all_units);
                    let turns = (turn + completed_this);
                    let total_health = all_units.values().map(|unit| unit.hp).sum::<i32>();
                    let answer = turns * total_health;
                    println!("Outcome: {} * {} = {}", turns, total_health, answer);
                    return Some(answer);
                }
            }
        }
    }
    return None;
}

fn main() {
    // assert!(simulate(include_str!("test"), 3, false) == 27730);
    // assert!(simulate(include_str!("test2"), 3, false) == 36334);
    // assert!(simulate(include_str!("test3"), 3, false) == 39514);
    // assert!(simulate(include_str!("test4"), 3, false) == 27755);
    // assert!(simulate(include_str!("test5"), 3, false) == 28944);
    // assert!(simulate(include_str!("test6"), 3, false) == 18740);
    assert!(simulate(include_str!("input"), 3, false).unwrap() == 225096);

    for elf_power in 4.. {
        if let Some(outcome) = simulate(include_str!("input"), elf_power, true) {
            println!("Part2 Elf power {} lets us have the outcome {} without a single Elf death", elf_power, outcome);
            assert!(outcome == 35354);
            break;
        }
    }
}
