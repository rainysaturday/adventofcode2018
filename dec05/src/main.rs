
fn react_pols(pols: &Vec<char>) -> Vec<&char> {
    let mut stack = Vec::new();
    for pol in pols {
        if stack.len() == 0 {
            stack.push(pol);
            continue;
        }
        let a = stack[stack.len() - 1];
        let b = pol;
        if b != a && b.eq_ignore_ascii_case(&a) {
            stack.pop();
        } else {
            stack.push(pol);
        }
    }
    stack
}

fn main() {
    let input = include_str!("input").lines().next().unwrap();
    // let input = "dabAcCaCBAcCcaDA"; // test

    let pols: Vec<char> = input.chars().collect();
    println!("Part1: {}", react_pols(&pols).len());

    // Part2
    let mut best = 1000000;
    for c in 0..'z' as u8 - 'a' as u8 {
        let char_lower = (c + 'a' as u8) as char;
        let char_upper = (c + 'A' as u8) as char;
        let filtered_pols: Vec<char> = input.chars()
            .filter(|x| *x != char_lower && *x != char_upper)
            .collect();
        let reacted = react_pols(&filtered_pols);
        if reacted.len() < best {
            best = reacted.len();
        }
    }
    println!("Part2: {}", best);
}
