use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
    Position(usize),
    Immediate(isize),
    Relative(isize),
}

impl Param {
    /// Decode parameter i for the opcode at the start of m.
    fn decode(m: &[isize], i: usize) -> Param {
        let expo = match i {
            0 => 100,
            1 => 1_000,
            2 => 10_000,
            _ => panic!("bad parameter number {} in {}", i, m[0]),
        };
        let val = m[i + 1];
        match (m[0] / expo) % 10 {
            0 => Param::Position(val.try_into().unwrap()),
            1 => Param::Immediate(val),
            2 => Param::Relative(val),
            x => panic!("bad mode {:?} in {:?}", x, m[0]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Insn {
    Stop,
    Input(Param),
    Output(Param),
    Add(Param, Param, Param),
    Mul(Param, Param, Param),

    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),

    AdjRelBase(Param),
}
use Insn::*;

impl Insn {
    /// Decode the instruction at the start of `m`.
    ///
    /// Returns the instruction and the encoded length.
    fn decode(m: &[isize]) -> (Insn, usize) {
        let mut param_num = 0;
        let mut p = || {
            let p = Param::decode(m, param_num);
            param_num += 1;
            p
        };
        let insn = match m[0] % 100 {
            1 => Add(p(), p(), p()),
            2 => Mul(p(), p(), p()),
            3 => Input(p()),
            4 => Output(p()),
            5 => JumpIfTrue(p(), p()),
            6 => JumpIfFalse(p(), p()),
            7 => LessThan(p(), p(), p()),
            8 => Equals(p(), p(), p()),
            9 => AdjRelBase(p()),
            99 => Stop,
            other => panic!("invalid opcode {}", other),
        };
        (insn, param_num + 1)
    }
}

#[derive(Clone)]
pub struct Computer {
    mem: Vec<isize>,
    pc: usize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    relbase: isize,
    wants_input: bool,
    halt: bool,
}

impl Computer {
    /// Construct a new computer, given an array of memory and a (possibly empty)
    /// stream of inputs made available to Input instructions.
    pub fn new(prog: &[isize]) -> Computer {
        Computer {
            mem: prog.to_vec(),
            pc: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relbase: 0,
            wants_input: false,
            halt: false,
        }
    }

    pub fn from_string(s: &str) -> Computer {
        Computer::new(&parse_string(s))
    }

    pub fn from_file(path: &str) -> Computer {
        Computer::from_string(&std::fs::read_to_string(path).unwrap())
    }

    pub fn wants_input(&self) -> bool {
        self.wants_input
    }

    pub fn is_halted(&self) -> bool {
        self.halt
    }

    /// Make a value available for input instructions.
    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input)
    }

    pub fn clear_input(&mut self) {
        self.input.clear()
    }

    pub fn input_len(&self) -> usize {
        self.input.len()
    }
    /// Push all the characters from the string into the input buffer.
    pub fn push_input_string(&mut self, s: &str) {
        for c in s.chars().map(|c| (c as u32) as isize) {
            self.push_input(c)
        }
    }

    pub fn poke_at(&mut self, addr: usize, v: isize) {
        self.mem[addr] = v;
    }

    pub fn pop_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    pub fn drain_output(&mut self) -> Vec<isize> {
        self.output.drain(..).collect()
    }

    // Length of pending output
    pub fn output_len(&self) -> usize {
        self.output.len()
    }

    pub fn drain_output_to_string_and_score(&mut self) -> (String, Option<isize>) {
        let mut s = String::with_capacity(self.output_len());
        let mut score = None;
        for c in self.drain_output() {
            match std::char::from_u32(c.try_into().unwrap()) {
                Some(ch) => s.push(ch),
                None => score = Some(c),
            }
        }
        (s, score)
    }

    pub fn borrow_mem(&self) -> &[isize] {
        &self.mem
    }

    /// Evaluate the next instruction.
    ///
    /// Return false if the computer stopped.
    fn step(&mut self) -> bool {
        let (insn, insn_len) = Insn::decode(&self.mem[self.pc..]);
        // By default, next pc will be after this instruction, but the
        // instruction might jump elsewhere, in which case this is
        // ignored.
        let mut newpc = self.pc + insn_len;
        self.wants_input = false;
        // println!("{:?}", &insn);
        match &insn {
            Stop => {
                self.halt = true;
                return false;
            }
            Add(p1, p2, p3) => self.poke(&p3, self.peek(&p1).checked_add(self.peek(&p2)).unwrap()),
            Mul(p1, p2, p3) => self.poke(&p3, self.peek(&p1).checked_mul(self.peek(&p2)).unwrap()),
            Input(a) => {
                if let Some(v) = self.input.pop_front() {
                    self.poke(&a, v);
                } else {
                    self.wants_input = true;
                    // Return without updating PC, so this will be tried again, hopefully
                    // after there's input.
                    return false;
                }
            }
            Output(a) => self.output.push_back(self.peek(&a)),
            JumpIfTrue(p1, p2) => {
                if self.peek(&p1) != 0 {
                    newpc = self.peek(&p2).try_into().unwrap()
                }
            }
            JumpIfFalse(p1, p2) => {
                if self.peek(&p1) == 0 {
                    newpc = self.peek(&p2).try_into().unwrap()
                }
            }
            LessThan(p1, p2, p3) => {
                let v: isize = (self.peek(&p1) < self.peek(&p2)).into();
                self.poke(&p3, v);
            }
            Equals(p1, p2, p3) => {
                let v: isize = (self.peek(&p1) == self.peek(&p2)).into();
                self.poke(&p3, v);
            }
            AdjRelBase(p) => {
                self.relbase = self.relbase.checked_add(self.peek(&p)).unwrap();
            }
        }
        self.pc = newpc;
        true
    }

    /// Run until reaching a Stop instruction, or lacking input.
    pub fn run(&mut self) {
        while self.step() {}
    }

    /// Run until either stopped, or output is available.
    ///
    /// Returns Some(output) if there's output, or None if the
    /// computer stopped.
    pub fn run_until_output(&mut self) -> Option<isize> {
        loop {
            if self.step() {
                if let Some(o) = self.output.pop_front() {
                    return Some(o);
                }
            } else {
                return None;
            }
        }
    }

    pub fn interact(&mut self) -> Option<isize> {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut in_lines = stdin.lock().lines();
        let mut out = stdout.lock();
        let mut score: Option<isize> = None;
        loop {
            self.run();
            let (text, new_score) = self.drain_output_to_string_and_score();
            score = score.or(new_score);
            out.write_all(&text.as_bytes()).unwrap();
            if self.is_halted() {
                return score;
            } else if self.wants_input() {
                let mut l: String = in_lines.next().unwrap().unwrap();
                l.push('\n');
                self.push_input_string(&l);
            }
        }
    }

    fn peek(&self, p: &Param) -> isize {
        let addr = match p {
            Param::Immediate(i) => return *i,
            Param::Position(p) => *p,
            Param::Relative(p) => usize::try_from(self.relbase.checked_add(*p).unwrap()).unwrap(),
        };
        if addr >= self.mem.len() {
            0
        } else {
            self.mem[addr]
        }
    }

    fn poke(&mut self, p: &Param, x: isize) {
        let addr = match p {
            Param::Immediate(_i) => panic!("can't write to immediate parameter"),
            Param::Position(p) => *p,
            Param::Relative(p) => self.relbase.checked_add(*p).unwrap().try_into().unwrap(),
        };
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0)
        }
        self.mem[addr] = x
    }

    #[cfg(test)]
    fn assert_mem_starts_with(&self, b: &[isize]) {
        assert_eq!(b, &self.mem[..b.len()]);
    }
}

pub fn parse_string(s: &str) -> Vec<isize> {
    s.split(',')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_example() {
        let mem = parse_string("1002,4,3,4,33");
        let (insn, len) = Insn::decode(&mem);
        // dbg!(&insn);
        assert_eq!(
            insn,
            Mul(Param::Position(4), Param::Immediate(3), Param::Position(4),)
        );
        assert_eq!(len, 4);
    }

    #[test]
    fn example_negative() {
        let mem = parse_string("1101,100,-1,4,0");
        let mut computer = Computer::new(&mem);
        assert_eq!(computer.step(), true);
        assert_eq!(computer.mem[4], 99);
        assert_eq!(computer.pc, 4);
    }

    #[test]
    fn examples_from_02() {
        // https://adventofcode.com/2019/day/2
        let mut computer = Computer::from_string("2,4,4,5,99,0");
        computer.run();
        computer.assert_mem_starts_with(&parse_string("2,4,4,5,99,9801"));

        let mut computer = Computer::from_string("1,1,1,4,99,5,6,0,99");
        computer.run();
        computer.assert_mem_starts_with(&parse_string("30,1,1,4,2,5,6,0,99"));
    }

    #[test]
    fn output() {
        let mut computer = Computer::from_string("104,1234,99");
        computer.run();
        assert_eq!(computer.pop_output(), Some(1234));
        assert_eq!(computer.pop_output(), None);
    }

    #[test]
    fn input() {
        let mut computer = Computer::from_string("3,3,99,9999");
        computer.push_input(8888);
        computer.run();
        assert_eq!(computer.mem, parse_string("3,3,99,8888"));
        assert!(computer.input.is_empty());
    }

    #[test]
    fn example_02_1() {
        let mut ic = Computer::from_string("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(ic.pc, 0);
        assert_eq!(ic.mem[3], 3);

        assert_eq!(ic.step(), true);
        assert_eq!(ic.mem[3], 70);

        assert_eq!(ic.step(), true);
        assert_eq!(ic.mem[0], 3500);

        assert_eq!(ic.step(), false);
    }

    #[test]
    fn examples_02_a() {
        let mut ic = Computer::from_string("1,0,0,0,99");
        assert_eq!(ic.step(), true);
        assert_eq!(ic.step(), false);
        ic.assert_mem_starts_with(&parse_string("2,0,0,0,99"));

        let mut ic = Computer::from_string("2,3,0,3,99");
        ic.run();
        ic.assert_mem_starts_with(&parse_string("2,3,0,6,99"));

        let mut ic = Computer::from_string("2,4,4,5,99,0");
        ic.run();
        ic.assert_mem_starts_with(&parse_string("2,4,4,5,99,9801"));

        let mut ic = Computer::from_string("1,1,1,4,99,5,6,0,99");
        ic.run();
        ic.assert_mem_starts_with(&parse_string("30,1,1,4,2,5,6,0,99"));
    }

    #[test]
    fn examples_from_09() {
        // Produces a copy of itself.
        let prog = parse_string("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut c = Computer::new(&prog);
        c.run();
        assert_eq!(&c.drain_output(), &prog);

        // 1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number.
        let mut c = Computer::from_string("1102,34915192,34915192,7,4,7,99,0");
        c.run();
        assert_eq!(c.drain_output(), &[1219070632396864]);

        let mut c = Computer::from_string("104,1125899906842624,99");
        c.run();
        assert_eq!(c.drain_output(), &[1125899906842624]);
    }
}
