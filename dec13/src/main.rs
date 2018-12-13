use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match self {
            Direction::LEFT  => Direction::DOWN,
            Direction::RIGHT => Direction::UP,
            Direction::UP    => Direction::LEFT,
            Direction::DOWN  => Direction::RIGHT,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::LEFT  => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
            Direction::UP    => Direction::RIGHT,
            Direction::DOWN  => Direction::LEFT,
        }
    }
}

#[derive(Debug)]
struct Cart {
    pos: (u32, u32),
    direction: Direction,
    next_turn: u32,
}

impl Cart {
    fn new(x: u32, y: u32, dir: Direction) -> Cart {
        Cart {
            pos: (x, y),
            direction: dir,
            next_turn: 0,
        }
    }

    fn move_cart(&mut self) {
        match &self.direction {
            Direction::LEFT  => self.pos.0 -= 1,
            Direction::RIGHT => self.pos.0 += 1,
            Direction::UP    => self.pos.1 -= 1,
            Direction::DOWN  => self.pos.1 += 1,
        }
    }

    fn update(&mut self, map: &HashMap<(u32, u32), char>) {
        let last_x = self.pos.0;
        let last_y = self.pos.1;
        self.move_cart();
        match map.get(&self.pos).expect("no mapposition where cart is?!") {
            '-' | '|' => (),    // Nothing to do
            '/' => if self.pos.0 == last_x {
                    if self.pos.1 < last_y {    // Coming from under
                        self.direction = Direction::RIGHT;
                    } else {                    // Coming from above
                        self.direction = Direction::LEFT;
                    }
                } else {
                    if self.pos.0 < last_x {    // Coming from right
                        self.direction = Direction::DOWN;
                    } else {                    // Coming from left
                        self.direction = Direction::UP;
                    }
                },
            '\\' => if self.pos.0 == last_x {
                    if self.pos.1 < last_y {    // Coming from under
                        self.direction = Direction::LEFT;
                    } else {                    // Coming from above
                        self.direction = Direction::RIGHT;
                    }
                } else {
                    if self.pos.0 < last_x {    // Coming from right
                        self.direction = Direction::UP;
                    } else {                    // Coming from left
                        self.direction = Direction::DOWN;
                    }
                },
            '+' => {
                match self.next_turn {
                    0 => self.direction = self.direction.rotate_left(),
                    1 => (), // forward, do nothing,
                    2 => self.direction = self.direction.rotate_right(),
                    other => panic!("Invalid next_turn direction: {}", other)
                }
                self.next_turn = (self.next_turn + 1) % 3;
            }
            otherwise => panic!("Unhandled case {} at {:?}", otherwise, self)
        }
    }
}

fn main() {
    let input = include_str!("input");
    // let input = include_str!("test");
    // let input = include_str!("test2");

    let mut carts = Vec::new();
    let mut map: HashMap<(u32, u32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '<' => { carts.push(Cart::new(x as u32, y as u32, Direction::LEFT));  map.insert((x as u32, y as u32), '-') },
                '>' => { carts.push(Cart::new(x as u32, y as u32, Direction::RIGHT)); map.insert((x as u32, y as u32), '-') },
                'v' => { carts.push(Cart::new(x as u32, y as u32, Direction::DOWN));  map.insert((x as u32, y as u32), '|') },
                '^' => { carts.push(Cart::new(x as u32, y as u32, Direction::UP));    map.insert((x as u32, y as u32), '|') },
                other => map.insert((x as u32, y as u32), other)
            };
        }
    }

    for tick in 1.. {
        carts.sort_by(|a, b| {
            if a.pos.1 == b.pos.1 {
                return a.pos.0.partial_cmp(&b.pos.0).unwrap();
            } else {
                return a.pos.1.partial_cmp(&b.pos.1).unwrap();
            }
        });

        if carts.len() <= 1 {
            println!("Only {:?} left on tick {}", carts, tick);
            break;
        }

        let mut carts_locations: HashSet<(u32, u32)> = carts.iter().map(|cart| cart.pos).collect();
        let mut crash_locations: HashSet<(u32, u32)> = HashSet::new();
        for cart in carts.iter_mut() {
            if !crash_locations.contains(&cart.pos) {   // If someone has not run into us
                carts_locations.remove(&cart.pos);
                cart.update(&map);
                if carts_locations.contains(&cart.pos) {
                    println!("Crash on tick {} at location {:?}", tick, cart.pos);
                    crash_locations.insert(cart.pos);
                } else {
                    carts_locations.insert(cart.pos);
                }
            }
        }
        carts = carts.into_iter()
            .filter(|cart| !crash_locations.contains(&cart.pos))
            .collect();
    }
}
