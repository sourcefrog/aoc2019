#![allow(dead_code)]

use std::fmt;

pub fn main() {
    println!("02a: {}", solve_a());
    println!("02b: {}", solve_b());
}

fn solve_a() -> usize {
    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?
    let mut ic = Intcode::from_string(&load_input());
    ic.mem[1] = 12;
    ic.mem[2] = 2;
    ic.run_to_completion()
}

fn solve_b() -> usize {
    // Find values a and b, each between 0 and 99 inclusive, stored in
    // address 1 and 2, that lead to noun final output value of 19690720 in
    // address 0.
    //
    // NOTE: In principle we could cause an infinite loop, and so the
    // tests might need to cap processing cycles. However this doesn't seem
    // to actually happen on this input, so I won't worry.
    let orig_ic = Intcode::from_string(&load_input());
    let desired = 19_690_720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            // dbg!(noun, verb);
            let mut ic = orig_ic.clone();
            ic.mem[1] = noun;
            ic.mem[2] = verb;
            let output = ic.run_to_completion();
            if output == desired {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

fn load_input() -> String {
    std::fs::read_to_string("input/input02.txt").unwrap()
}

#[derive(Clone)]
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
        // dbg!(op, a, b, c, self.pc);
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

    pub fn run_to_completion(&mut self) -> usize {
        while self.step() {}
        self.mem[0]
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
        assert_eq!(solve_a(), 3_790_689);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 6533);
    }
}
