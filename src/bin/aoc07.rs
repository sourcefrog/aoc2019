use std::convert::TryInto;

use mbp_aoc2019::intcode::{parse_string, Computer};
use mbp_aoc2019::permute::permutations;

pub fn main() {
    println!("07a: {:?}", solve_a().0);
}

fn solve_a() -> (isize, Vec<usize>) {
    solve_type_a(&load_input())
}

fn solve_type_a(prog: &str) -> (isize, Vec<usize>) {
    let prog = parse_string(prog);
    let mut best_pow = 0;
    let mut best_phases = Vec::new();

    for phases in permutations(5) {
        let pow = run_pipeline(&phases, &prog);
        // println!("{:?} => {}", &phases, pow);
        if pow > best_pow {
            best_pow = pow;
            best_phases = phases.clone();
        }
    }
    (best_pow, best_phases)
}

fn load_input() -> String {
    std::fs::read_to_string("input/input07.txt").unwrap()
}

/// Simulate a single amplifier, given the phase setting, input value, and
/// program. Returns the output.
fn run_amp(phase: usize, inpval: isize, prog: &[isize]) -> isize {
    let mut c = Computer::new(prog.to_vec());
    c.push_input(phase.try_into().unwrap());
    c.push_input(inpval);
    c.run();
    let out = c.borrow_output();
    assert_eq!(out.len(), 1);
    out[0]
}

/// Pass values through the pipeline in accordance with phases, and
/// return the final output.
fn run_pipeline(phases: &[usize], prog: &[isize]) -> isize {
    let mut pow = 0;
    for phase in phases {
        pow = run_amp(*phase, pow, prog);
    }
    pow
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        assert_eq!(
            solve_type_a("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            (43210, vec![4, 3, 2, 1, 0])
        );

        assert_eq!(
            solve_type_a(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,
                23,23,1,24,23,23,4,23,99,0,0"
            ),
            (54321, vec![0, 1, 2, 3, 4])
        );

        assert_eq!(
            solve_type_a(
                "3,31,3,32,1002,32,10,32,1001,31,-2,
                31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,
                31,4,31,99,0,0,0"
            ),
            (65210, vec![1, 0, 4, 3, 2])
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a().0, 118_936);
    }
}
