// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::convert::TryInto;

use mbp_aoc2019::intcode::{parse_string, Computer};
use mbp_aoc2019::permute::permutations;

pub fn main() {
    println!("07a: {:?}", solve_a().0);
    println!("07b: {:?}", solve_b().0);
}

fn solve_a() -> (isize, Vec<usize>) {
    solve_type_a(&load_input())
}

fn solve_type_a(prog: &str) -> (isize, Vec<usize>) {
    let prog = parse_string(prog);
    let mut best_pow = 0;
    let mut best_phases = Vec::new();

    for phases in permutations(0..5) {
        let pow = run_pipeline(&phases, &prog);
        // println!("{:?} => {}", &phases, pow);
        if pow > best_pow {
            best_pow = pow;
            best_phases = phases.clone();
        }
    }
    (best_pow, best_phases)
}

fn solve_b() -> (isize, Vec<usize>) {
    solve_type_b(&load_input())
}

fn solve_type_b(prog: &str) -> (isize, Vec<usize>) {
    let prog = parse_string(prog);
    let mut best_pow = 0;
    let mut best_phases = Vec::new();

    for phases in permutations(5..=9) {
        let pow = run_feedback(&phases, &prog);
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
    let mut c = Computer::new(prog);
    c.push_input(phase.try_into().unwrap());
    c.push_input(inpval);
    c.run();
    let out = c.drain_output();
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

/// Type B problem: run all the amplifiers until they all halt;
/// then the last output from the final amplifier is the result.
fn run_feedback(phases: &[usize], prog: &[isize]) -> isize {
    let mut comps = vec![Computer::new(prog); 5];
    for i in 0..5 {
        comps[i].push_input(phases[i].try_into().unwrap())
    }
    // push initial input
    comps[0].push_input(0);
    let mut stopped = [false; 5];
    let mut i = 0;
    let mut e_out = 0;
    while stopped != [true; 5] {
        if !stopped[i] {
            if let Some(out) = comps[i].run_until_output() {
                comps[(i + 1) % 5].push_input(out);
                if i == 4 {
                    e_out = out;
                }
            // println!("{} emitted {}", i, out);
            } else {
                // println!("{} stopped", i);
                stopped[i] = true;
            }
        }
        i = (i + 1) % 5;
    }
    e_out
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

    #[test]
    fn examples_b() {
        assert_eq!(
            solve_type_b(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            ),
            (139_629_729, vec![9, 8, 7, 6, 5])
        );

        assert_eq!(
            solve_type_b(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            ),
            (18216, vec![9, 7, 8, 5, 6])
        );
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b().0, 57_660_948);
    }
}
