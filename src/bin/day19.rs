use advtools::input::{iter_input, parse_parts, to_usize};
use strum_macros::EnumString;

#[derive(EnumString)]
#[strum(serialize_all="snake_case")]
enum Op {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
}

struct Insn {
    op: Op,
    data: [u32; 3],
}

struct VM<'i> {
    ip: usize,
    prog: &'i [Insn],
    regs: [u32; 6],
    break_at: u32,
}

impl VM<'_> {
    fn new(prog: &Vec<Insn>, ip: usize, break_at: u32, regs: [u32; 6]) -> VM {
        VM { prog, ip, break_at, regs }
    }

    fn reg(&self, n: u32) -> u32 {
        self.regs[n as usize]
    }

    fn run(&mut self) -> [u32; 6] {
        while self.regs[self.ip] < self.prog.len() as u32 {
            let insn = &self.prog[self.regs[self.ip] as usize];
            let [ia, ib, it] = insn.data;
            self.regs[it as usize] = match insn.op {
                Op::Addr => self.reg(ia) + self.reg(ib),
                Op::Addi => self.reg(ia) + ib,
                Op::Mulr => self.reg(ia) * self.reg(ib),
                Op::Muli => self.reg(ia) * ib,
                Op::Banr => self.reg(ia) & self.reg(ib),
                Op::Bani => self.reg(ia) & ib,
                Op::Borr => self.reg(ia) | self.reg(ib),
                Op::Bori => self.reg(ia) | ib,
                Op::Setr => self.reg(ia),
                Op::Seti => ia,
                Op::Gtir => (ia > self.reg(ib)) as u32,
                Op::Gtri => (self.reg(ia) > ib) as u32,
                Op::Gtrr => (self.reg(ia) > self.reg(ib)) as u32,
                Op::Eqir => (ia == self.reg(ib)) as u32,
                Op::Eqri => (self.reg(ia) == ib) as u32,
                Op::Eqrr => (self.reg(ia) == self.reg(ib)) as u32,
            };
            self.regs[self.ip] += 1;
            if self.regs[self.ip] == self.break_at {
                return self.regs;
            }
        }
        self.regs
    }
}

fn main() {
    let mut input_iter = iter_input::<String>();
    let ip_index = to_usize(input_iter.next().unwrap().split_whitespace().nth(1).unwrap());
    let prog = input_iter.map(|line| {
        let (opstr, data): (String, _) = parse_parts(&line, [0, 1, 2, 3]);
        Insn { op: opstr.parse().unwrap(), data }
    }).collect();

    // Part 1: run the VM normally to find the result.
    let regs = VM::new(&prog, ip_index, 0, [0; 6]).run();
    advtools::verify("Starting with reg0 = 0", regs[0], 968);

    // Part 2: the result is the product of all prime factors + 1 of r5 after
    // jumping to instruction 2.  Since the algorithm is quadratic, just rewrite
    // a proper version here instead of trying to optimize the VM instructions.
    let regs = VM::new(&prog, ip_index, 1, [1, 0, 0, 0, 0, 0]).run();
    let mut input = regs[5];
    let mut output = 1;
    while input > 1 {
        let factor = (2..).find(|&n| input % n == 0).unwrap();
        input /= factor;
        output *= factor + 1;
    }
    advtools::verify("Starting with reg0 = 1", output, 10557936);
}
