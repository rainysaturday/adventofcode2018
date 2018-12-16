extern crate regex;
use regex::Regex;
use std::cmp;
use std::collections::{HashSet, HashMap};

type Imm = i32;
type Reg = usize;

#[derive(Debug)]
struct Sample {
    before: [Imm; 4],
    insn: [Imm; 4],
    after: [Imm; 4],
}

impl Sample {
    fn new(before: [Imm; 4], instr: [Imm; 4], after: [Imm; 4]) -> Sample {
        Sample {
            before: before,
            insn: instr,
            after: after
        }
    }
}

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


fn execute(insn: Insn, regs: &mut [Imm; 4]) {
    match insn {
        Insn::Addr(rn, rm, rd) => regs[rd] = regs[rn] + regs[rm],
        Insn::Addi(rn, im, rd) => regs[rd] = regs[rn] + im,
        Insn::Mulr(rn, rm, rd) => regs[rd] = regs[rn] * regs[rm],
        Insn::Muli(rn, im, rd) => regs[rd] = regs[rn] * im,
        Insn::Banr(rn, rm, rd) => regs[rd] = regs[rn] & regs[rm],
        Insn::Bani(rn, im, rd) => regs[rd] = regs[rn] & im,
        Insn::Borr(rn, rm, rd) => regs[rd] = regs[rn] | regs[rm],
        Insn::Bori(rn, im, rd) => regs[rd] = regs[rn] | im,
        Insn::Setr(rn, _, rd) => regs[rd] = regs[rn],
        Insn::Seti(im, _, rd) => regs[rd] = im,
        Insn::Gtir(im, rm, rd) => regs[rd] = if im > regs[rm] { 1 } else { 0 },
        Insn::Gtri(rn, im, rd) => regs[rd] = if regs[rn] > im { 1 } else { 0 },
        Insn::Gtrr(rn, rm, rd) => regs[rd] = if regs[rn] > regs[rm] { 1 } else { 0 },
        Insn::Eqir(im, rm, rd) => regs[rd] = if im == regs[rm] { 1 } else { 0 },
        Insn::Eqri(rn, im, rd) => regs[rd] = if regs[rn] == im { 1 } else { 0 },
        Insn::Eqrr(rn, rm, rd) => regs[rd] = if regs[rn] == regs[rm] { 1 } else { 0 },
        _ => panic!("Unknown instruction")
    };
}

fn parse_insn(insn_order: Imm, insn: &[Imm; 4]) -> Insn {
    match insn_order {
        0 => Insn::Addr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        1 => Insn::Addi(insn[1] as Reg, insn[2] as Imm, insn[3] as Reg),
        2 => Insn::Mulr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        3 => Insn::Muli(insn[1] as Reg, insn[2] as Imm, insn[3] as Reg),
        4 => Insn::Banr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        5 => Insn::Bani(insn[1] as Reg, insn[2] as Imm, insn[3] as Reg),
        6 => Insn::Borr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        7 => Insn::Bori(insn[1] as Reg, insn[2] as Imm, insn[3] as Reg),
        8 => Insn::Setr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        9 => Insn::Seti(insn[1] as Imm, insn[2] as Imm, insn[3] as Reg),
        10 => Insn::Gtir(insn[1] as Imm, insn[2] as Reg, insn[3] as Reg),
        11 => Insn::Gtri(insn[1] as Reg, insn[2] as Imm, insn[3] as Reg),
        12 => Insn::Gtrr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        13 => Insn::Eqir(insn[1] as Imm, insn[2] as Reg, insn[3] as Reg),
        14 => Insn::Eqri(insn[1] as Reg, insn[2] as Imm, insn[3] as Reg),
        15 => Insn::Eqrr(insn[1] as Reg, insn[2] as Reg, insn[3] as Reg),
        _ => panic!("not valid variant")
    }
}

fn part2(solves: &mut HashMap<Imm, HashSet<Imm>>, raw_insns: &Vec<[Imm; 4]>) {
    let mut opcode_to_insn_order: HashMap<Imm, Imm> = HashMap::new();
    let mut vec_solve: Vec<(Imm, Vec<Imm>)> = solves.iter()
        .map(|(opcode, list_orders)| {
            let mut sorted: Vec<Imm> = list_orders.iter().map(|v| *v).collect();
            sorted.sort();
            (*opcode, sorted)
        }).collect();
    vec_solve.sort_by(|a, b| {
        a.0.partial_cmp(&b.0).unwrap()
    });
    for (opcode, solved_by) in &vec_solve {
        println!("Opcode {} is solved by {:?}", opcode, solved_by);
    }

    while opcode_to_insn_order.len() < 16 {
        let mut found_map_for = None;
        for (opcode, solved_by) in solves.iter() {
            if solved_by.len() == 1 {
                let order = *solved_by.iter().next().unwrap();
                found_map_for = Some(order);
                opcode_to_insn_order.insert(*opcode, order);
                break;
            }
        }
        if let Some(order) = found_map_for {
            for v in solves.values_mut() {
                v.remove(&order);
            }
        } else {
            panic!("Could not solve, current state: {:?}", solves);
        }
    }
    println!("Solution {:?}", opcode_to_insn_order);

    // Lets run all the instructions
    let mut registers: [Imm; 4] = [0, 0, 0, 0];
    for raw_insn in raw_insns {
        let insn = parse_insn(*opcode_to_insn_order.get(&raw_insn[0]).unwrap(), raw_insn);
        execute(insn, &mut registers);
    }
    println!("Part2: register 0 contains {}", registers[0]);
}

fn part1(samples: &Vec<Sample>) -> HashMap<Imm, HashSet<Imm>> {
    let mut three_or_more_same_behaviour = 0;
    let mut solves: HashMap<Imm, HashSet<Imm>> = HashMap::new();
    for sample in samples {
        let mut ok_count = 0;
        for variant in 0..16 {
            let mut regs = sample.before;
            let insn = parse_insn(variant, &sample.insn);
            execute(insn, &mut regs);
            
            let mut all_ok = true;
            for i in 0..regs.len() {
                if regs[i] != sample.after[i] {
                    all_ok = false;
                    break;
                }
            }
            if all_ok {
                ok_count += 1;
                if solves.contains_key(&sample.insn[0]) {
                    solves.get_mut(&sample.insn[0]).unwrap().insert(variant);
                } else {
                    let mut compat = HashSet::new();
                    compat.insert(variant);
                    solves.insert(sample.insn[0], compat);
                }
            }
        }

        if ok_count >= 3 {
            three_or_more_same_behaviour += 1;
        }
    }

    println!("Part1: num samples that show the same behaviour for three or more instructions: {}", three_or_more_same_behaviour);
    return solves;
}

fn main() {
    let input = include_str!("input");

    let before_samp_r = Regex::new(r"Before: *\[(-?\d+), (-?\d+), (-?\d+), (-?\d+)\]").unwrap(); 
    let instr_r = Regex::new(r"(-?\d+) (-?\d+) (-?\d+) (-?\d+)").unwrap(); 
    let after_samp_r = Regex::new(r"After: *\[(-?\d+), (-?\d+), (-?\d+), (-?\d+)\]").unwrap();

    let mut raw_insns: Vec<[Imm; 4]> = Vec::new();
    let mut samples: Vec<Sample> = Vec::new();
    let mut curr_before = None;
    let mut curr_instr = None;
    let mut blank_count = 0;
    let mut parse_program = false;
    for line in input.lines() {
        if line.len() == 0 {
            blank_count += 1;
            if blank_count >= 3 {
                parse_program = true;
            }
        } else {
            blank_count = 0;
        }

        if let Some(c) = before_samp_r.captures(line) {
            if parse_program {
                panic!("Got 'before' when parsing program.");
            }
            let r0 = c.get(1).unwrap().as_str().parse::<Imm>().unwrap();
            let r1 = c.get(2).unwrap().as_str().parse::<Imm>().unwrap();
            let r2 = c.get(3).unwrap().as_str().parse::<Imm>().unwrap();
            let r3 = c.get(4).unwrap().as_str().parse::<Imm>().unwrap();
            let regs = [r0, r1, r2, r3];
            curr_before = Some(regs);
        } else if let Some(c) = after_samp_r.captures(line) {
            if parse_program {
                panic!("Got 'after' when parsing program.");
            }
            let r0 = c.get(1).unwrap().as_str().parse::<Imm>().unwrap();
            let r1 = c.get(2).unwrap().as_str().parse::<Imm>().unwrap();
            let r2 = c.get(3).unwrap().as_str().parse::<Imm>().unwrap();
            let r3 = c.get(4).unwrap().as_str().parse::<Imm>().unwrap();
            let regs = [r0, r1, r2, r3];
            if curr_before.is_none() || curr_instr.is_none() {
                panic!("Received after, when before or instr has not been seen on line: {}", line);
            }
            samples.push(Sample::new(curr_before.unwrap(), curr_instr.unwrap(), regs));
            curr_before = None;
            curr_instr = None;
        } else if let Some(c) = instr_r.captures(line) {
            let op = c.get(1).unwrap().as_str().parse::<Imm>().unwrap();
            let p0 = c.get(2).unwrap().as_str().parse::<Imm>().unwrap();
            let p1 = c.get(3).unwrap().as_str().parse::<Imm>().unwrap();
            let p2 = c.get(4).unwrap().as_str().parse::<Imm>().unwrap();
            let instr = [op, p0, p1, p2];
            curr_instr = Some(instr);
            if parse_program {
                raw_insns.push(instr);
            }
        }
    }

    let mut solves = part1(&samples);
    part2(&mut solves, &raw_insns);
}
