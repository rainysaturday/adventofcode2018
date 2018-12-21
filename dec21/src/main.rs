extern crate regex;
use regex::Regex;
use std::collections::HashMap;

type Imm = i64;
type Reg = usize;
const NUM_REG: usize = 6;

#[derive(Debug)]
enum Insn {
    Addr(Reg, Reg, Reg),
    Addi(Reg, Imm, Reg),
    Mulr(Reg, Reg, Reg),
    Muli(Reg, Imm, Reg),
    Banr(Reg, Reg, Reg),
    Bani(Reg, Imm, Reg),
    Borr(Reg, Reg, Reg),
    Bori(Reg, Imm, Reg),
    Setr(Reg, Reg, Reg),
    Seti(Imm, Imm, Reg),
    Gtir(Imm, Reg, Reg),
    Gtri(Reg, Imm, Reg),
    Gtrr(Reg, Reg, Reg),
    Eqir(Imm, Reg, Reg),
    Eqri(Reg, Imm, Reg),
    Eqrr(Reg, Reg, Reg),
}


fn execute(insn: &Insn, regs: &mut [Imm; NUM_REG]) {
    match insn {
        Insn::Addr(rn, rm, rd) => regs[*rd] = regs[*rn] + regs[*rm],
        Insn::Addi(rn, im, rd) => regs[*rd] = regs[*rn] + im,
        Insn::Mulr(rn, rm, rd) => regs[*rd] = regs[*rn] * regs[*rm],
        Insn::Muli(rn, im, rd) => regs[*rd] = regs[*rn] * im,
        Insn::Banr(rn, rm, rd) => regs[*rd] = regs[*rn] & regs[*rm],
        Insn::Bani(rn, im, rd) => regs[*rd] = regs[*rn] & im,
        Insn::Borr(rn, rm, rd) => regs[*rd] = regs[*rn] | regs[*rm],
        Insn::Bori(rn, im, rd) => regs[*rd] = regs[*rn] | im,
        Insn::Setr(rn, _, rd) => regs[*rd] = regs[*rn],
        Insn::Seti(im, _, rd) => regs[*rd] = *im,
        Insn::Gtir(im, rm, rd) => regs[*rd] = if *im > regs[*rm] { 1 } else { 0 },
        Insn::Gtri(rn, im, rd) => regs[*rd] = if regs[*rn] > *im { 1 } else { 0 },
        Insn::Gtrr(rn, rm, rd) => regs[*rd] = if regs[*rn] > regs[*rm] { 1 } else { 0 },
        Insn::Eqir(im, rm, rd) => regs[*rd] = if *im == regs[*rm] { 1 } else { 0 },
        Insn::Eqri(rn, im, rd) => regs[*rd] = if regs[*rn] == *im { 1 } else { 0 },
        Insn::Eqrr(rn, rm, rd) => regs[*rd] = if regs[*rn] == regs[*rm] { 1 } else { 0 },
        _ => panic!("Unknown instruction")
    };
}

fn parse_insn(insn_order: &str, insn: &[Imm; 3]) -> Insn {
    match insn_order {
        "addr" => Insn::Addr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        "addi" => Insn::Addi(insn[0] as Reg, insn[1] as Imm, insn[2] as Reg),
        "mulr" => Insn::Mulr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        "muli" => Insn::Muli(insn[0] as Reg, insn[1] as Imm, insn[2] as Reg),
        "banr" => Insn::Banr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        "bani" => Insn::Bani(insn[0] as Reg, insn[1] as Imm, insn[2] as Reg),
        "borr" => Insn::Borr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        "bori" => Insn::Bori(insn[0] as Reg, insn[1] as Imm, insn[2] as Reg),
        "setr" => Insn::Setr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        "seti" => Insn::Seti(insn[0] as Imm, insn[1] as Imm, insn[2] as Reg),
        "gtir" => Insn::Gtir(insn[0] as Imm, insn[1] as Reg, insn[2] as Reg),
        "gtri" => Insn::Gtri(insn[0] as Reg, insn[1] as Imm, insn[2] as Reg),
        "gtrr" => Insn::Gtrr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        "eqir" => Insn::Eqir(insn[0] as Imm, insn[1] as Reg, insn[2] as Reg),
        "eqri" => Insn::Eqri(insn[0] as Reg, insn[1] as Imm, insn[2] as Reg),
        "eqrr" => Insn::Eqrr(insn[0] as Reg, insn[1] as Reg, insn[2] as Reg),
        _ => panic!("not valid variant")
    }
}

fn run_program(program: &Vec<Insn>, ip_reg: usize, regs: &mut [Imm; NUM_REG], analyze: bool) -> (u64, bool, HashMap<Imm, Imm>) {
    let mut count = 0u64;
    let mut halted = false;
    let mut reg3s: HashMap<Imm, Imm> = HashMap::new();
    loop {
        let pc = regs[ip_reg] as usize;
        // if count % 100_000_000 == 0 {
            if pc == 28 && analyze {
                let size = reg3s.len();
                if !reg3s.contains_key(&regs[3]) {
                    reg3s.insert(regs[3], count as Imm);
                }
                if reg3s.len() == size {
                    println!("No new value detected after {}", count);
                    break;
                }
            }
        // }
        execute(&program[pc], regs);
        count += 1;

        if regs[ip_reg] >= 0 && ((regs[ip_reg] + 1i64) as usize) < program.len() {
            regs[ip_reg] += 1;
        } else {
            halted = true;
            break;
        }
    }
    println!("Executed {}", count);

    return (count, halted, reg3s);
}

fn main() {
    let input = include_str!("input");

    let ip_r = Regex::new(r"#ip (\d+)").unwrap(); 
    let instr_r = Regex::new(r"(.+) (-?\d+) (-?\d+) (-?\d+)").unwrap(); 

    let mut program: Vec<Insn> = Vec::new();
    let mut ip_reg_id = None;
    for line in input.lines() {
        if let Some(c) = instr_r.captures(line) {
            let op = c.get(1).unwrap().as_str();
            let p0 = c.get(2).unwrap().as_str().parse::<Imm>().unwrap();
            let p1 = c.get(3).unwrap().as_str().parse::<Imm>().unwrap();
            let p2 = c.get(4).unwrap().as_str().parse::<Imm>().unwrap();
            let params = [p0, p1, p2];
            let instr = parse_insn(op, &params);
            program.push(instr);
        } else if let Some(c) = ip_r.captures(line) {
            ip_reg_id = Some(c.get(1).unwrap().as_str().parse::<usize>().unwrap());
        } else {
            panic!("Unknown input: {}", line);
        }
    }

    // Looking at the input, the eqrr is seen comparing reg3 to reg0, and if they are equal the program will halt
    // Run and collect all the values we see for reg3 at this position, also store the execution count, then find min max
    println!("Analyzing instructions...");
    let mut regs: [Imm; NUM_REG] = [0, 0, 0, 0, 0, 0];
    let (_, _, reg3s) = run_program(&program, ip_reg_id.expect("should have a ip reg"), &mut regs, true);
    let lowest = reg3s.iter().min_by_key(|(zeroval, &count)| count).unwrap();
    let highest = reg3s.iter().max_by_key(|(zeroval, &count)| count).unwrap();
    println!("Part1: reg-0-value {} only executes {} instructions", lowest.0, lowest.1);
    println!("Part2: reg-0-value {} executes the most {} instructions", highest.0, highest.1);
}
