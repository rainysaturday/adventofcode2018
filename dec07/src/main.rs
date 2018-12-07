extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Node {
    depends_on: HashSet<char>,
    satisfies: HashSet<char>,
}

impl Node {
    fn new() -> Node {
        Node {
            depends_on: HashSet::new(),
            satisfies: HashSet::new()
        }
    }
}

fn is_satisfied(node_char: &char, nodes: &HashMap<char, Node>, performed: &HashSet<char>) -> bool {
    let node = nodes.get(node_char).unwrap();
    node.depends_on.iter().all(|dependency| performed.contains(dependency))
}

fn find_satisfied(nodes: &HashMap<char, Node>, performed: &HashSet<char>) -> HashSet<char> {
    nodes.keys()
        .filter(|node| is_satisfied(&node, &nodes, performed))
        .map(|node| *node)
        .collect()
}


fn perform(nodes: &HashMap<char, Node>) {
    let mut performed: HashSet<char> = HashSet::new();
    let mut order_performed = Vec::new();
    loop {
        let mut candidates: HashSet<char> = find_satisfied(&nodes, &performed).into_iter()
            .filter(|sat| !performed.contains(sat))
            .collect();

        if candidates.len() == 0 {
            println!("part1 performed in order: {}", order_performed.iter().collect::<String>());
            return;
        }
        let cand = *candidates.iter().min().unwrap();
        performed.insert(cand);
        order_performed.push(cand);
        candidates.remove(&cand);
    }
}

fn perform2(nodes: &HashMap<char, Node>) {
    let limit_worker = 5;
    let mut performed: HashSet<char> = HashSet::new();
    let mut ongoing: HashMap<char, u32> = HashMap::new();
    let mut order_performed = Vec::new();
    let mut current_time = 0;
    loop {
        // Find candidates for work nodes
        let mut candidates: HashSet<char> = find_satisfied(&nodes, &performed).into_iter()
            .filter(|sat| !performed.contains(sat) && !ongoing.contains_key(sat))
            .collect();

        // If no new work available, and no ongoing work, then we are done
        if candidates.len() == 0 && ongoing.len() == 0 {
            println!("part2 performed in order: {}, completed at {}", order_performed.iter().collect::<String>(), current_time);
            return;
        }

        // Hand out work
        let workers_available = limit_worker - ongoing.len();
        for _ in 0..workers_available {
            if candidates.len() > 0 {
                let cand = *candidates.iter().min().unwrap();
                let completes_at = current_time + cand as u32 - 'A' as u32 + 61;
                if !ongoing.contains_key(&cand) {
                    ongoing.insert(cand, completes_at);
                }
                candidates.remove(&cand);
            }
        }

        // Perform the works that are closest to finish
        let mut earliest_ids = Vec::new();
        let mut earliest_deadline = 100000000;
        for work in ongoing.iter() {
            if *work.1 < earliest_deadline {
                earliest_ids.clear();
                earliest_ids.push(*work.0);
                earliest_deadline = *work.1;
            } else if *work.1 == earliest_deadline {
                earliest_ids.push(*work.0);
            }
        }
        earliest_ids.sort();
        for work in earliest_ids.iter() {
            ongoing.remove(work);
            performed.insert(*work);
            order_performed.push(*work);
            current_time = earliest_deadline;
        }
    }
}

fn main() {
    let input = include_str!("input");
    // let input = include_str!("test");

    // Step C must be finished before step A can begin.
    let reg = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();

    // Parse nodes
    let mut nodes: HashMap<char, Node> = HashMap::new();
    for line in input.lines() {
        let c = reg.captures(line).unwrap();
        let node = c.get(2).unwrap().as_str().chars().next().unwrap();
        let dependency = c.get(1).unwrap().as_str().chars().next().unwrap();
        if nodes.contains_key(&node) {
            let mut found_node = nodes.get_mut(&node).unwrap();
            found_node.depends_on.insert(dependency);
        } else {
            let mut new_node = Node::new();
            new_node.depends_on.insert(dependency);
            nodes.insert(node, new_node);
        }

        if nodes.contains_key(&dependency) {
            let mut found_node = nodes.get_mut(&dependency).unwrap();
            found_node.satisfies.insert(node);
        } else {
            let mut new_node = Node::new();
            new_node.satisfies.insert(node);
            nodes.insert(dependency, new_node);
        }
    }

    perform(&nodes);
    perform2(&nodes);
}
