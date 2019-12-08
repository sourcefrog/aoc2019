#![allow(dead_code)]

use std::collections::VecDeque;
use std::convert::TryFrom;

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
    fn examples_b() {
        // 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        // let mut c = Computer::from_string("3,9,8,9,10,9,4,9,99,-1,8");
        // c.push_input(&[7]);
        // c.run();
        // assert!(c.input.is_empty());
        // assert_eq!(c.output, &[0]);

        // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        // 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        // 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

        // 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
        // 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
        // Here's a larger example:
    }
}
