use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
    Position(usize),
    Immediate(isize),
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
            99 => Stop,
            other => panic!("invalid opcode {}", other),
        };
        (insn, param_num + 1)
    }
}

#[derive(Clone, Debug)]
pub struct Computer {
    mem: Vec<isize>,
    pc: usize,
    input: VecDeque<isize>,
    output: Vec<isize>,
}

impl Computer {
    /// Construct a new computer, given an array of memory and a (possibly empty)
    /// stream of inputs made available to Input instructions.
    pub fn new(mem: Vec<isize>) -> Computer {
        Computer {
            mem,
            pc: 0,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }

    pub fn from_string(s: &str) -> Computer {
        Computer::new(parse_string(s))
    }

    /// Make a value available for input instructions.
    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input)
    }

    pub fn poke_at(&mut self, addr: usize, v: isize) {
        self.mem[addr] = v;
    }

    pub fn borrow_output(&self) -> &[isize] {
        &self.output
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
        match &insn {
            Stop => return false,
            Add(p1, p2, p3) => self.poke(&p3, self.peek(&p1).checked_add(self.peek(&p2)).unwrap()),
            Mul(p1, p2, p3) => self.poke(&p3, self.peek(&p1).checked_mul(self.peek(&p2)).unwrap()),
            Input(a) => {
                let v = self.input.pop_front().unwrap();
                self.poke(&a, v);
            }
            Output(a) => self.output.push(self.peek(&a)),
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
        }
        self.pc = newpc;
        true
    }

    /// Run until reaching a Stop instruction.
    pub fn run(&mut self) {
        while self.step() {}
    }

    fn peek(&self, p: &Param) -> isize {
        match p {
            Param::Immediate(i) => *i,
            Param::Position(p) => self.mem[*p],
        }
    }

    fn poke(&mut self, p: &Param, x: isize) {
        match p {
            Param::Immediate(_i) => panic!("can't write to immediate parameter"),
            Param::Position(p) => self.mem[*p] = x,
        }
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
        let mut computer = Computer::new(mem);
        assert_eq!(computer.step(), true);
        assert_eq!(computer.mem[4], 99);
        assert_eq!(computer.pc, 4);
    }

    #[test]
    fn examples_from_02() {
        // https://adventofcode.com/2019/day/2
        let mut computer = Computer::from_string("2,4,4,5,99,0");
        computer.run();
        assert_eq!(computer.mem, parse_string("2,4,4,5,99,9801"));

        let mut computer = Computer::from_string("1,1,1,4,99,5,6,0,99");
        computer.run();
        assert_eq!(computer.mem, parse_string("30,1,1,4,2,5,6,0,99"));
    }

    #[test]
    fn output() {
        let mut computer = Computer::from_string("104,1234,99");
        computer.run();
        assert_eq!(computer.output, &[1234]);
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
        assert_eq!(ic.mem, parse_string("2,0,0,0,99"));

        let mut ic = Computer::from_string("2,3,0,3,99");
        ic.run();
        assert_eq!(ic.mem, parse_string("2,3,0,6,99"));

        let mut ic = Computer::from_string("2,4,4,5,99,0");
        ic.run();
        assert_eq!(ic.mem, parse_string("2,4,4,5,99,9801"));

        let mut ic = Computer::from_string("1,1,1,4,99,5,6,0,99");
        ic.run();
        assert_eq!(ic.mem, parse_string("30,1,1,4,2,5,6,0,99"));
    }
}
