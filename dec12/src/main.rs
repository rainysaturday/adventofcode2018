extern crate regex;
use regex::Regex;
use std::collections::HashSet;

fn print_state(state: &HashSet<i32>) -> String {
    let min = *state.iter().min().unwrap();
    let max = *state.iter().max().unwrap();
    let mut res = String::new();
    for i in min..=max {
        res.push(if state.contains(&i) { '#' } else { '.' });
    }
    res
}

fn populate_next(next_gen: &mut HashSet<i32>, current_gen: &HashSet<i32>, rules: &Vec<(&str, bool)>) {
    let min = *current_gen.iter().min().unwrap();
    let max = *current_gen.iter().max().unwrap();
    for pot in min-5..=max+5 {
        for (rule, does_create_new) in rules {
            let mut rule_match = true;
            for (offset, pot_info) in rule.chars().enumerate() {
                match pot_info {
                    '#' => if !current_gen.contains(&(pot + offset as i32 - 2)) { rule_match = false; },
                    '.' => if current_gen.contains(&(pot + offset as i32 - 2)) { rule_match = false; },
                    _ => panic!("bad rule")
                }
            }

            if rule_match {
                if *does_create_new {
                    next_gen.insert(pot);
                }
                break;
            }
        }
    }
}

fn main() {
    let input = include_str!("input");
    // let input = include_str!("test");

    let init_state_reg = Regex::new(r"initial state: (.*)").unwrap();
    let rules_reg = Regex::new(r"(.*) => (.)").unwrap();

    let mut current_gen: HashSet<i32> = HashSet::new();
    let mut rules: Vec<(&str, bool)> = Vec::new();

    for line in input.lines() {
        if let Some(c) = init_state_reg.captures(line) {
            let state = c.get(1).unwrap().as_str();
            state.chars().enumerate().for_each(|(id, pot)| {
                if pot == '#' {
                    current_gen.insert(id as i32);
                }
            });
        } else if let Some(c) = rules_reg.captures(line) {
            rules.push((
                c.get(1).unwrap().as_str(), 
                c.get(2).unwrap().as_str().chars().nth(0).unwrap() == '#', 
                ));
        }
    }

    println!("Generation 0: {}", print_state(&current_gen));

    let mut part1sum: i64 = 0;
    let mut part2sum: i64 = 0;

    let mut last_sum = current_gen.iter().sum::<i32>();
    let mut last_diff = last_sum as i64;
    for gen_id in 1.. {
        let mut next_gen = HashSet::new();
        populate_next(&mut next_gen, &current_gen, &rules);
        let sum = next_gen.iter().sum::<i32>();
        let diff = (sum - last_sum) as i64;
        // println!("Gen {}: {}: sum {}, diff from last sum {}", gen_id, print_state(&next_gen), sum, diff);
        if gen_id == 20 {
            part1sum = next_gen.iter().sum::<i32>() as i64;
        }

        if print_state(&current_gen).eq(&print_state(&next_gen)) && last_diff == diff {
            println!("Equilibrium reached after generation {}, no more relatively unique generations will happen.", gen_id);
            part2sum = (sum as i64) + ((50_000_000_000 - gen_id as i64) * diff);
            break;
        }
        
        current_gen = next_gen;
        last_sum = sum;
        last_diff = diff;
    }

    println!("Part1: sum pots after 20 gens: {}", part1sum);
    println!("Part2: after 50_000_000_000 generations the sum will be: {}", part2sum);
}
