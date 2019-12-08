#![allow(dead_code)]

use std::collections::VecDeque;
use std::convert::TryInto;

pub fn main() {
    println!("05a: {}", solve_a());
}

fn solve_a() -> isize {
    let mut computer = Computer::new(load_input());
    computer.push_input(&[1]);
    computer.run();
    // Output should be all 0s except the last element, which is the answer.
    let out = computer.output;
    let (confirms, ans) = out.split_at(out.len() - 1);
    assert!(confirms.iter().all(|i| *i == 0));
    ans[0]
}

fn load_input() -> Vec<isize> {
    parse_string(&std::fs::read_to_string("input/input05.txt").unwrap())
}

fn parse_string(s: &str) -> Vec<isize> {
    s.trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Param {
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
enum Insn {
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

struct Computer {
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

    /// Make these values available for input instructions.
    pub fn push_input(&mut self, input: &[isize]) {
        self.input.extend(input)
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
    fn run(&mut self) {
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
    fn solution_02a() {
        // Check this implementation still completely runs the 02a problem
        // to the expected solution.
        let mut mem = parse_string(&std::fs::read_to_string("input/input02.txt").unwrap());
        mem[1] = 12;
        mem[2] = 2;
        let mut computer = Computer::new(mem);
        computer.run();
        assert_eq!(computer.mem[0], 3_790_689);
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
        computer.push_input(&[8888]);
        computer.run();
        assert_eq!(computer.mem, parse_string("3,3,99,8888"));
        assert!(computer.input.is_empty());
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 15_259_545);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn examples_b() {
        // 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let mut c = Computer::from_string("3,9,8,9,10,9,4,9,99,-1,8");
        c.push_input(&[7]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[0]);

        let mut c = Computer::from_string("3,9,8,9,10,9,4,9,99,-1,8");
        c.push_input(&[8]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[1]);

        // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let mut c = Computer::from_string("3,9,7,9,10,9,4,9,99,-1,8");
        c.push_input(&[9999]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[0]);

        for below_val in &[-1234, 0, 7] {
            let mut c = Computer::from_string("3,9,7,9,10,9,4,9,99,-1,8");
            c.push_input(&[*below_val]);
            c.run();
            assert!(c.input.is_empty());
            assert_eq!(c.output, &[1]);
        }

        // 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let mut c = Computer::from_string("3,3,1108,-1,8,3,4,3,99");
        c.push_input(&[9999]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[0]);

        let mut c = Computer::from_string("3,3,1108,-1,8,3,4,3,99");
        c.push_input(&[8]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[1]);

        // 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let mut c = Computer::from_string("3,3,1107,-1,8,3,4,3,99");
        c.push_input(&[8]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[0]);

        let mut c = Computer::from_string("3,3,1107,-1,8,3,4,3,99");
        c.push_input(&[-8]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[1]);

        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        // 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
        let mut c = Computer::from_string("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        c.push_input(&[-8]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[1]);

        let mut c = Computer::from_string("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        c.push_input(&[0]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[0]);

        // 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
        let mut c = Computer::from_string("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        c.push_input(&[-8]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[1]);

        let mut c = Computer::from_string("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        c.push_input(&[0]);
        c.run();
        assert!(c.input.is_empty());
        assert_eq!(c.output, &[0]);
    }
}
