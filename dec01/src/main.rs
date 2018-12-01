use std::collections::HashSet;

fn part2(values: &Vec<i32>) -> i32 {
    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(0);
    let mut freq = 0;
    loop {
        for v in values {
            freq += v;
            if visited.contains(&freq) {
                return freq;
            }
            visited.insert(freq);            
        }
    }
}

fn main() {
    let input = include_str!("input");

    let values:Vec<i32> = input.lines()
        .map(|line| line.parse::<i32>().unwrap() )
        .collect();

    println!("part1: {}", values.iter().sum::<i32>());
    println!("part2: {}", part2(&values));
}