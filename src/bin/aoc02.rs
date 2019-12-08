#![allow(dead_code)]

use std::fmt;

pub fn main() {
    println!("02a: {}", solve_a());
}

fn solve_a() -> usize {
    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?
    let mut ic = Intcode::from_string(&load_input());
    ic.mem[1] = 12;
    ic.mem[2] = 2;
    while ic.step() {}
    ic.mem[0]
}

fn load_input() -> String {
    std::fs::read_to_string("input/input02.txt").unwrap()
}

struct Intcode {
    /// Contents of memory.
    mem: Vec<usize>,

    /// 0-based program counter.
    pc: usize,
}

impl Intcode {
    /// Take one step, mutating memory and updating pc.
    ///
    /// Return false if the computer should stop.
    pub fn step(&mut self) -> bool {
        let op = match self.mem[self.pc] {
            99 => return false,
            1 => usize::checked_add,
            2 => usize::checked_mul,
            other => {
                println!("Invalid opcode {} at pc={}", other, self.pc);
                return false;
            }
        };
        let a = self.mem[self.mem[self.pc + 1]];
        let b = self.mem[self.mem[self.pc + 2]];
        let c = self.mem[self.pc + 3];
        dbg!(op, a, b, c, self.pc);
        self.mem[c] = op(a, b).unwrap();
        self.pc += 4;
        true
    }

    pub fn from_string(s: &str) -> Intcode {
        Intcode {
            mem: s
                .trim()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
            pc: 0,
        }
    }
}

impl fmt::Display for Intcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mem[0])?;
        for i in &self.mem[1..] {
            write!(f, ",{}", i)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut ic = Intcode::from_string("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(ic.pc, 0);
        assert_eq!(ic.mem[3], 3);

        assert_eq!(ic.step(), true);
        assert_eq!(ic.mem[3], 70);

        assert_eq!(ic.step(), true);
        assert_eq!(ic.mem[0], 3500);

        assert_eq!(ic.step(), false);
    }

    #[test]
    fn examples_part_a() {
        let mut ic = Intcode::from_string("1,0,0,0,99");
        assert_eq!(ic.step(), true);
        assert_eq!(ic.step(), false);
        assert_eq!(ic.to_string(), "2,0,0,0,99");

        let mut ic = Intcode::from_string("2,3,0,3,99");
        while ic.step() {}
        assert_eq!(ic.to_string(), "2,3,0,6,99");

        let mut ic = Intcode::from_string("2,4,4,5,99,0");
        while ic.step() {}
        assert_eq!(ic.to_string(), "2,4,4,5,99,9801");

        let mut ic = Intcode::from_string("1,1,1,4,99,5,6,0,99");
        while ic.step() {}
        assert_eq!(ic.to_string(), "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 3790689);
    }
}
