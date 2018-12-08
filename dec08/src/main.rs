struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>
}

impl Node {
    fn new() -> Node {
        Node {
            metadata: Vec::new(),
            children: Vec::new()
        }
    }
}

fn parse(region: &[i32], all_metadata: &mut Vec<i32>) -> (Node, usize) {
    if region.len() < 2 {
        panic!("region is too small {:?}", region);
    }

    let num_children = region[0];
    let num_metadata = region[1];
    let mut this_node = Node::new();

    let mut position: usize = 2;
    for _ in 0..num_children {
        let (child_node, parsed_offset) = parse(&region[position..], all_metadata);
        this_node.children.push(child_node);
        position += parsed_offset;
    }

    for pos in position..position+num_metadata as usize {
        let metadata = region[pos];
        all_metadata.push(metadata);
        this_node.metadata.push(metadata);
    }

    (this_node, position + num_metadata as usize)
}

fn value(node: &Node) -> i32 {
    if node.children.len() == 0 {
        return node.metadata.iter().sum::<i32>();
    }

    let mut sum = 0;
    for m in &node.metadata {
        if (*m as usize) <= node.children.len() && *m > 0 {
            sum += value(&node.children[(*m - 1) as usize]);
        }
    }

    return sum;
}

fn main() {
    let input = include_str!("input").lines().next().unwrap();
    // let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    let numbers: Vec<i32> = input.split_whitespace()
        .map(|w| w.parse::<i32>().unwrap())
        .collect();

    let mut all_metadata = Vec::new();
    let (root_node, _) = parse(&numbers, &mut all_metadata);
    println!("part1 sum metadata: {}", all_metadata.iter().sum::<i32>());
    println!("part2 vaulation: {}", value(&root_node));
}
