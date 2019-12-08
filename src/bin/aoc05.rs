#![allow(dead_code)]

use std::collections::VecDeque;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::iter::IntoIterator;

pub fn main() {}

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
        let mode =
            m[0] / (match i {
                0 => 100,
                1 => 1_000,
                2 => 10_000,
                _ => panic!(),
            }) % 10;
        let val = m[i + 1];
        match mode {
            0 => Param::Position(usize::try_from(val).unwrap()),
            1 => Param::Immediate(val),
            x => panic!("bad mode {:?} in {:?}", x, m[0]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Insn {
    Stop,
    Input { a: Param },
    Output { a: Param },
    Add { p: [Param; 3] },
    Mul { p: [Param; 3] },
}
use Insn::*;

impl Insn {
    /// Number of memory cells for the encoded form including the opcode.
    fn encoded_len(&self) -> usize {
        match self {
            Stop => 1,
            Input { .. } | Output { .. } => 2,
            Add { .. } | Mul { .. } => 4,
        }
    }

    /// Decode the instruction at the start of `m`.
    fn decode(m: &[isize]) -> Insn {
        match m[0] % 100 {
            1 => Add {
                p: [
                    Param::decode(m, 0),
                    Param::decode(m, 1),
                    Param::decode(m, 2),
                ],
            },
            2 => Mul {
                p: [
                    Param::decode(m, 0),
                    Param::decode(m, 1),
                    Param::decode(m, 2),
                ],
            },
            3 => Input {
                a: Param::decode(m, 0),
            },
            4 => Output {
                a: Param::decode(m, 0),
            },
            99 => Stop,
            other => panic!("invalid opcode {}", other),
        }
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
    fn new<I>(mem: Vec<isize>, input: I) -> Computer
    where
        I: IntoIterator<Item = isize>,
    {
        Computer {
            mem,
            pc: 0,
            input: VecDeque::from_iter(input.into_iter()),
            output: Vec::new(),
        }
    }

    /// Evaluate the next instruction.
    ///
    /// Return false if the computer stopped.
    fn step(&mut self) -> bool {
        let insn = Insn::decode(&self.mem[self.pc..]);
        match &insn {
            Stop => return false,
            Add { p } => self.poke(
                &p[2],
                self.peek(&p[0]).checked_add(self.peek(&p[1])).unwrap(),
            ),
            Mul { p } => self.poke(
                &p[2],
                self.peek(&p[0]).checked_mul(self.peek(&p[1])).unwrap(),
            ),
            Input { a } => {
                let v = self.input.pop_front().unwrap();
                self.poke(&a, v);
            }
            Output { a } => self.output.push(self.peek(&a)),
        }
        self.pc += insn.encoded_len();
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
    use std::iter::empty;

    #[test]
    fn decode_example() {
        let mem = parse_string("1002,4,3,4,33");
        let insn = Insn::decode(&mem);
        // dbg!(&insn);
        assert_eq!(
            insn,
            Mul {
                p: [Param::Position(4), Param::Immediate(3), Param::Position(4),],
            }
        );
    }

    #[test]
    fn example_negative() {
        let mem = parse_string("1101,100,-1,4,0");
        let mut computer = Computer::new(mem, empty());
        assert_eq!(computer.step(), true);
        assert_eq!(computer.mem[4], 99);
        assert_eq!(computer.pc, 4);
    }

    #[test]
    fn examples_from_02() {
        // https://adventofcode.com/2019/day/2
        let mut computer = Computer::new(parse_string("2,4,4,5,99,0"), empty());
        computer.run();
        assert_eq!(computer.mem, parse_string("2,4,4,5,99,9801"));

        let mut computer = Computer::new(parse_string("1,1,1,4,99,5,6,0,99"), empty());
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
        let mut computer = Computer::new(mem, empty());
        computer.run();
        assert_eq!(computer.mem[0], 3_790_689);
    }

    #[test]
    fn output() {
        let mut computer = Computer::new(parse_string("104,1234,99"), empty());
        computer.run();
        assert_eq!(computer.output, &[1234]);
    }
}
