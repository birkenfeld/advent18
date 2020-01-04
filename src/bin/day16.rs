use advtools::prelude::{Itertools, HashMap, HashSet};
use advtools::itertools::put_back_n;
use advtools::input::{iter_input, parse_parts, parse_parts_trim};
use strum::IntoEnumIterator;
use strum_macros::{EnumString, EnumIter};

type Word = [u32; 4];

#[derive(EnumString, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
}

struct VM {
    regs: Word,
}

impl VM {
    fn new(regs: Word) -> Self { Self { regs } }

    fn run(&mut self, prog: Vec<Word>, table: &HashMap<u32, Op>) -> Word {
        for word in prog {
            self.exec_insn(table[&word[0]], word);
        }
        self.regs
    }

    fn exec_insn(&mut self, op: Op, data: Word) -> Word {
        match op {
            Op::Addr => self.exec_raw(data, false, false, |a, b| a + b),
            Op::Addi => self.exec_raw(data, false, true,  |a, b| a + b),
            Op::Mulr => self.exec_raw(data, false, false, |a, b| a * b),
            Op::Muli => self.exec_raw(data, false, true,  |a, b| a * b),
            Op::Banr => self.exec_raw(data, false, false, |a, b| a & b),
            Op::Bani => self.exec_raw(data, false, true,  |a, b| a & b),
            Op::Borr => self.exec_raw(data, false, false, |a, b| a | b),
            Op::Bori => self.exec_raw(data, false, true,  |a, b| a | b),
            Op::Setr => self.exec_raw(data, false, false, |a, _| a),
            Op::Seti => self.exec_raw(data, true,  false, |a, _| a),
            Op::Gtir => self.exec_raw(data, true,  false, |a, b| (a > b) as u32),
            Op::Gtri => self.exec_raw(data, false, true,  |a, b| (a > b) as u32),
            Op::Gtrr => self.exec_raw(data, false, false, |a, b| (a > b) as u32),
            Op::Eqir => self.exec_raw(data, true,  false, |a, b| (a == b) as u32),
            Op::Eqri => self.exec_raw(data, false, true,  |a, b| (a == b) as u32),
            Op::Eqrr => self.exec_raw(data, false, false, |a, b| (a == b) as u32),
        }
        self.regs
    }

    fn exec_raw(&mut self, data: Word, ia: bool, ib: bool, f: impl Fn(u32, u32) -> u32) {
        self.regs[data[3] as usize] = f(if ia { data[1] } else { self.regs[data[1] as usize] },
                                        if ib { data[2] } else { self.regs[data[2] as usize] });
    }
}

fn main() {
    let mut line_iter = put_back_n(iter_input::<String>());

    let mut traces: Vec<[Word; 3]> = vec![];
    while let Some((reg1, insn, reg2)) = line_iter.next_tuple() {
        if !reg1.starts_with("Before:") {
            line_iter.put_back(reg2);
            line_iter.put_back(insn);
            line_iter.put_back(reg1);
            break;
        }
        traces.push([parse_parts_trim(&reg1, [1, 2, 3, 4], "[,]"),
                     parse_parts(&insn, [0, 1, 2, 3]),
                     parse_parts_trim(&reg2, [1, 2, 3, 4], "[,]")]);
    }
    let program: Vec<Word> = line_iter.map(|line| parse_parts(&line, [0, 1, 2, 3])).collect_vec();

    let mut opcode_candidates = HashMap::<_, HashSet<_>>::new();
    let ambiguous = traces.into_iter().filter(|trace| {
        Op::iter().filter(|&op| {
            if VM::new(trace[0]).exec_insn(op, trace[1]) == trace[2] {
                opcode_candidates.entry(op).or_default().insert(trace[1][0]);
                true
            } else {
                false
            }
        }).count() >= 3
    }).count();
    advtools::verify("Ambiguous opcodes", ambiguous, 640);

    let mut opcode_table = HashMap::<u32, Op>::new();
    while !opcode_candidates.is_empty() {
        opcode_candidates.retain(|&op, cands| {
            cands.retain(|v| !opcode_table.contains_key(v));
            if cands.len() == 1 {
                opcode_table.insert(*cands.iter().next().unwrap(), op);
                false
            } else {
                true
            }
        });
    }

    let regs = VM::new([0; 4]).run(program, &opcode_table);
    advtools::verify("Register 0", regs[0], 472);
}
