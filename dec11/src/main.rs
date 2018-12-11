use std::collections::HashMap;

fn power_level(x: u32, y: u32, grid_serial: u32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += grid_serial;
    power *= rack_id;
    ((power as i32 / 100) % 10) - 5
}

fn largest_area_sum(grid_serial: u32, size_range: std::ops::RangeInclusive<u32>) -> (u32, u32, u32, i32) {
    let mut area_sum_cache: HashMap<(u32, u32), i32> = HashMap::new();
    let mut largest_size = 1;
    let mut largest_pos = (0, 0);
    let mut largest_power = 0;
    for current_size in size_range {
        for y in 1..=300 - (current_size - 1) {
            for x in 1..=300 - (current_size - 1) {
                let mut power_sum = 0i32;
                if let Some(cached_sum) = area_sum_cache.get(&(x, y)) {
                    // Only need to run the right and bottom sides
                    power_sum = *cached_sum;
                    for sub_x in x..x+current_size - 1 {
                        power_sum += power_level(sub_x, y+current_size-1, grid_serial);
                    }
                    for sub_y in y..y+current_size - 1 {
                        power_sum += power_level(x+current_size-1, sub_y, grid_serial);
                    }

                    // Add corner
                    power_sum += power_level(x+current_size-1, y+current_size-1, grid_serial);
                } else {
                    for sub_x in x..x+current_size {
                        for sub_y in y..y+current_size {
                            power_sum += power_level(sub_x, sub_y, grid_serial);
                        }
                    }
                }
                
                area_sum_cache.insert((x, y), power_sum);

                // Insert top left position with sum
                if power_sum > largest_power {
                    largest_power = power_sum;
                    largest_size = current_size;
                    largest_pos = (x, y)
                }
            }
        }
    }
    (largest_pos.0, largest_pos.1, largest_size, largest_power)
}

fn main() {
    let input = 8444;
    // let input = 18; // Test

    let part1 = largest_area_sum(input, 3..=3);
    println!("Part1: area ({},{},{}) has largest power of {}", part1.0, part1.1, part1.2, part1.3);

    // Part 2
    let part2 = largest_area_sum(input, 1..=300);
    println!("Part2: area ({},{},{}) has largest power of {}", part2.0, part2.1, part2.2, part2.3);
}
