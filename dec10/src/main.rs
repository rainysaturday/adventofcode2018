extern crate regex;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32
}

impl Point {
    fn new(x: i32, y: i32, vel_x: i32, vel_y: i32) -> Point {
        Point {
            x: x,
            y: y,
            vel_x: vel_x,
            vel_y: vel_y
        }
    }

    fn update(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

fn is_cohesive(map: &HashSet<(i32, i32)>) -> bool {
    // Message is cohesive is no single point is without connections
    for loc in map {
        if !map.contains(&(loc.0 - 1, loc.1)) &&
           !map.contains(&(loc.0 + 1, loc.1)) &&
           !map.contains(&(loc.0 - 1, loc.1 - 1)) &&
           !map.contains(&(loc.0 + 1, loc.1 + 1)) &&
           !map.contains(&(loc.0, loc.1 - 1)) &&
           !map.contains(&(loc.0, loc.1 + 1)) &&
           !map.contains(&(loc.0 - 1, loc.1 + 1)) &&
           !map.contains(&(loc.0 + 1, loc.1 - 1)) {
               return false;    // Not cohesive since this point is by itself
           }
    }

    true
}

fn render(map: &HashSet<(i32, i32)>) {
    let left_x = map.iter().min_by_key(|val| val.0).unwrap().0;
    let right_x = map.iter().max_by_key(|val| val.0).unwrap().0;
    let top_y = map.iter().min_by_key(|val| val.1).unwrap().1;
    let bottom_y = map.iter().max_by_key(|val| val.1).unwrap().1;

    for y in top_y..=bottom_y {
        for x in left_x..=right_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input");
    // let input = include_str!("test");

    // position=< 9,  1> velocity=< 0,  2>
    let reg = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();

    let mut points = Vec::new();
    for line in input.lines() {
        let c = reg.captures(line).unwrap();
        let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vel_x = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vel_y = c.get(4).unwrap().as_str().parse::<i32>().unwrap();
        points.push(Point::new(x, y, vel_x, vel_y));
    }

    for second in 1.. {
        points.iter_mut().for_each(|p| p.update());
        let map: HashSet<(i32, i32)> = points.iter()
                    .map(|p| (p.x, p.y))
                    .collect();
        if is_cohesive(&map) {
            let new_height = points.iter().max_by_key(|p| p.y).unwrap().y - points.iter().min_by_key(|p| p.y).unwrap().y;
            println!("seconds: {}, message is {} high", second, new_height);
            render(&map);
            break;
        }
    }
}
