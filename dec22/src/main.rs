use std::collections::HashMap;

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

fn main() {
    // Test
    // let depth = 510;
    // let target = (10, 10);
    // Input
    let depth = 9171;
    let target = (7, 721);


    part1(depth, target);

}
