use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    prev: u32,
    next: u32,
}

impl Node {
    fn clone(&self) -> Node {
        Node {
            prev: self.prev,
            next: self.next
        }
    }
}

struct HashList {
    start_value: u32,
    nodes: HashMap<u32, Node>
}

impl HashList {
    fn new(start_value: u32) -> HashList {
        let mut hash_list = HashList {
            start_value: start_value,
            nodes: HashMap::new()
        };
        hash_list.nodes.insert(start_value, Node {prev: start_value, next: start_value});

        hash_list
    }

    fn remove_value(&mut self, value_to_remove: u32) {
        let middle = self.nodes.get(&value_to_remove).unwrap().clone();
        let mut before = self.nodes.get(&middle.prev).unwrap().clone();
        let mut after = self.nodes.get(&middle.next).unwrap().clone();

        if value_to_remove == self.start_value {
            self.start_value = middle.next;
        }

        self.nodes.remove(&value_to_remove);

        // Fix up links
        before.next = middle.next;
        after.prev = middle.prev;

        self.nodes.insert(middle.prev, before);
        self.nodes.insert(middle.next, after);
    }

    fn insert_before(&mut self, value_to_insert: u32, before_this_value: u32) {
        let mut after = self.nodes.get(&before_this_value).unwrap().clone();
        let mut before = self.nodes.get(&after.prev).unwrap().clone();

        // Insert new value
        let middle = Node {
            prev: after.prev,
            next: before.next
        };

        // Fix up links
        if self.nodes.len() == 1 {
            self.nodes.insert(before_this_value, Node {
                prev: value_to_insert,
                next: value_to_insert
            });
        } else {
            before.next = value_to_insert;
            after.prev = value_to_insert;
            self.nodes.insert(middle.prev, before);
            self.nodes.insert(middle.next, after);
        }
        self.nodes.insert(value_to_insert, middle);
    }

    fn find(&self, starting_point: u32, num_steps: i32) -> u32 {
        if num_steps == 0 {
            return starting_point;
        }

        if num_steps < 0 {
            return self.find(self.nodes.get(&starting_point).unwrap().prev, num_steps + 1);
        } else {
            return self.find(self.nodes.get(&starting_point).unwrap().next, num_steps - 1);
        }
    }

    fn validate(&self) {
        let mut current_next = self.start_value;
        let mut current_prev = self.start_value;
        let mut next_hashes = HashSet::new();
        let mut prev_hashes = HashSet::new();

        for _ in 0..self.nodes.len() {
            next_hashes.insert(current_next);
            prev_hashes.insert(current_prev);
            // println!("current_prev: {}, current_next {}", current_prev, current_next);
            current_next = self.nodes.get(&current_next).expect("expecting next").next;
            current_prev = self.nodes.get(&current_prev).expect("expecting prev").prev;
        }

        if current_prev != self.start_value ||
            current_next != self.start_value ||
            next_hashes.len() != self.nodes.len() || 
            prev_hashes.len() != self.nodes.len() {
            panic!("Not valid links anymore: nodes: {:?}\nnext_hashes {} -> {:?}\nprev_hashes {} {:?}",
                self.nodes, 
                next_hashes.len(), next_hashes, 
                prev_hashes.len(), prev_hashes);
        }
    }
}

fn play_marbles(limit_players: usize, limit_marbles: u32) {
    let mut player_score = Vec::new();
    let mut placed_marbles = HashList::new(0u32);
    for _ in 0..limit_players {
        player_score.push(0u32);
    }

    let mut current_player = 0;
    let mut last_placed_marble = 0;

    for marble_to_place in 1..=limit_marbles {
        if marble_to_place % 23 == 0 {
            let remove_marble = placed_marbles.find(last_placed_marble, -7);
            last_placed_marble = placed_marbles.find(remove_marble, 1);
            placed_marbles.remove_value(remove_marble);

            player_score[current_player] += remove_marble + marble_to_place;
        } else {
            let insert_before_marble = placed_marbles.find(last_placed_marble, 2);
            placed_marbles.insert_before(marble_to_place, insert_before_marble);
            last_placed_marble = marble_to_place;
        }

        current_player = (current_player + 1) % limit_players;
    }

    let (winner, score) = player_score.iter().enumerate().max_by_key(|x| x.1).unwrap();
    println!("Player {} has highest score of {}", winner + 1, score);
}

fn main() {

    play_marbles(9, 25);     // test
    play_marbles(13, 7999);     // test
    play_marbles(491, 71058);     // Part1
    play_marbles(491, 7105800);     // Part2
}
