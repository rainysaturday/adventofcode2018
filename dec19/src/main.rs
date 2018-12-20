extern crate regex;
use regex::Regex;

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

fn run_program(program: &Vec<Insn>, ip_reg: usize, regs: &mut [Imm; NUM_REG]) {
    let mut count = 0u64;
    loop {
        let pc = regs[ip_reg] as usize;
        if count % 100_000_000 == 0 {
            println!("executed {}... pc {}, {:?}, regs: {:?}", count, pc, program[pc], regs);
        }
        execute(&program[pc], regs);
        count += 1;

        if regs[ip_reg] >= 0 && ((regs[ip_reg] + 1i64) as usize) < program.len() {
            regs[ip_reg] += 1;
        } else {
            break;
        }
    }
    println!("Executed {} instructions, resulting registers {:?}", count, regs);
}

fn main() {
    // let input = include_str!("test");
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

    println!("Part1");
    let mut regs: [Imm; NUM_REG] = [0, 0, 0, 0, 0, 0];
    run_program(&program, ip_reg_id.expect("should have a ip reg"), &mut regs);

    println!("Part2");  // Solved in pseudo file
    let target_val = 10551345;
    // let target_val = 945;
    let mut sum = 0;
    for i in 1..=target_val {
        if target_val % i == 0 {
            println!("Adding {}", i);
            sum += i; // * (target_val/i); 
        }
    }
    println!("Register 0 contains {}", sum)
}
