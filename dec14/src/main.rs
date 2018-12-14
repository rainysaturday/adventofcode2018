
fn search_last(recipes: &Vec<u8>, digits: &Vec<u8>) -> bool {
    if recipes.len() >= digits.len() {
        for i in 0..digits.len() {
            if recipes[recipes.len() - i - 1] != digits[i] {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn val_to_digits(input: usize) -> Vec<u8> {
    let mut ans = Vec::new();
    let digits = (input as f32).log10() as usize;
    let mut val = input;
    for _ in 0..=digits {
        ans.push((val % 10) as u8);
        val /= 10;
    }
    ans
}

fn main() {
    let input = 990941;
    // let input = 59414;
    // let input = 2018;  // test;
    // let input = 2018;  // test;
    // let input = 5;  // test;

    let digits = val_to_digits(input);
    let mut recipes: Vec<u8> = Vec::new();
    recipes.push(3);
    recipes.push(7);
    let mut elf1pos = 0;
    let mut elf2pos = 1;
    let mut part1solved = false;
    let mut part2solved = false;
    while !part1solved || !part2solved {
        let a = recipes[elf1pos];
        let b = recipes[elf2pos];
        let sum = a + b;
        if sum >= 10 {
            recipes.push(sum / 10);
        }
        if !part2solved && search_last(&recipes, &digits) {
            part2solved = true;
            println!("Found {} to the left of {}", recipes.len() - digits.len(), input);
        }
        recipes.push(sum % 10);
        if !part2solved && search_last(&recipes, &digits) {
            part2solved = true;
            println!("Found {} to the left of {}", recipes.len() - digits.len(), input);
        }

        if !part1solved && recipes.len() >= input + 10 {
            part1solved = true;
            print!("Last 10 recipes after {}: ", input);
            for i in input..input+10 {
                print!("{}", recipes[i]);
            }
            println!();
        }

        elf1pos = (elf1pos + 1 + a as usize) % recipes.len();
        elf2pos = (elf2pos + 1 + b as usize) % recipes.len();
    }
}
