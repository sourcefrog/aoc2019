#![allow(dead_code)]

use mbp_aoc2019::intcode::{parse_string, Computer};

pub fn main() {
    println!("05a: {}", solve_a());
    println!("05b: {}", solve_b());
}

fn solve_a() -> isize {
    let mut computer = Computer::new(&load_input());
    computer.push_input(1);
    computer.run();
    // Output should be all 0s except the last element, which is the answer.
    let out = computer.drain_output();
    let (confirms, ans) = out.split_at(out.len() - 1);
    assert!(confirms.iter().all(|i| *i == 0));
    ans[0]
}

fn solve_b() -> isize {
    let mut c = Computer::new(&load_input());
    c.push_input(5);
    c.run();
    let out = c.drain_output();
    assert_eq!(out.len(), 1);
    out[0]
}

fn load_input() -> Vec<isize> {
    parse_string(&std::fs::read_to_string("input/input05.txt").unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_02a() {
        // Check this implementation still completely runs the 02a problem
        // to the expected solution.
        let mut mem = parse_string(&std::fs::read_to_string("input/input02.txt").unwrap());
        mem[1] = 12;
        mem[2] = 2;
        let mut computer = Computer::new(&mem);
        computer.run();
        assert_eq!(computer.borrow_mem()[0], 3_790_689);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 15_259_545);
    }

    #[test]
    fn examples_b() {
        for (code, input, output) in &[
            ("3,9,8,9,10,9,4,9,99,-1,8", 7, 0),
            ("3,9,8,9,10,9,4,9,99,-1,8", 8, 1),
            ("3,9,7,9,10,9,4,9,99,-1,8", 0, 1),
            ("3,9,7,9,10,9,4,9,99,-1,8", 7, 1),
            ("3,9,7,9,10,9,4,9,99,-1,8", 8, 0),
            ("3,9,7,9,10,9,4,9,99,-1,8", 10, 0),
            ("3,3,1108,-1,8,3,4,3,99", 8, 1),
            ("3,3,1108,-1,8,3,4,3,99", -8, 0),
            ("3,3,1107,-1,8,3,4,3,99", 0, 1),
            ("3,3,1107,-1,8,3,4,3,99", 8, 0),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 100, 1),
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0),
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -1, 1),
        ] {
            let mut c = Computer::from_string(code);
            c.push_input(*input);
            c.run();
            // assert!(c.input.is_empty());
            assert_eq!(c.drain_output(), &[*output]);
        }
    }

    #[test]
    fn example_b_large() {
        let cbase = Computer::from_string(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        );
        // The above example program uses an input instruction to ask for a
        // single number. The program will then output 999 if the input value
        // is below 8, output 1000 if the input value is equal to 8, or output
        // 1001 if the input value is greater than 8.
        for (input, output) in &[(7, 999), (8, 1000), (123, 1001)] {
            let mut c = cbase.clone();
            c.push_input(*input);
            c.run();
            assert_eq!(c.drain_output(), &[*output]);
        }
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 7_616_021)
    }
}
