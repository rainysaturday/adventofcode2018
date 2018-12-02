use std::collections::HashMap;

fn part2(lines: std::str::Lines) {
    let l: Vec<&str> = lines.collect();
    for i in 0..l.len() {
        for u in 0..l.len() {
            let a = l[i].as_bytes();
            let b = l[u].as_bytes();
            let mut diffs = 0;
            let mut last_diff_pos = 0;
            for pos in 0..a.len() {
                if a[pos] != b[pos] {
                    diffs += 1;
                    last_diff_pos = pos;
                }
            }
            if diffs == 1 {
                let mut diff = l[i].to_string();
                diff.remove(last_diff_pos);
                println!("part2: {} and {} diffs to {}", l[i], l[u], diff);
            }
        }
    }
}

fn count_chars(line: &str) -> HashMap<u8, u32> {
    let mut map: HashMap<u8, u32> = HashMap::new();
    for c in line.to_ascii_lowercase().as_bytes() {
        let count = map.get(c).unwrap_or(&0u32) + 1;
        map.insert(*c, count);
    }
    map
}

fn part1(lines: std::str::Lines) {
    let mut twos = 0;
    let mut threes = 0;
    for line in lines {
        let map = count_chars(line);
        twos += if map.values().any(|v| *v == 2) { 1 } else { 0 };
        threes += if map.values().any(|v| *v == 3) { 1 } else { 0 }; 
    }
    println!("part1: {}", threes * twos);
}

fn main() {
    let input = include_str!("input");

    part1(input.lines());
    part2(input.lines());
}
