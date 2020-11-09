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

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("21a: {}", solve_a());
    println!("21b: {}", solve_b());
}

fn solve_a() -> isize {
    let cpu = &mut Computer::from_file("input/input21.txt");

    cpu.run();
    print_output(cpu);
    push_string_input(
        cpu,
        "\
NOT T T
AND A T
AND B T 
AND C T 
NOT T J
AND D J
WALK\n",
    );
    cpu.run();
    print_output(cpu).unwrap()
}

fn solve_b() -> isize {
    let cpu = &mut Computer::from_file("input/input21.txt");

    cpu.run();
    print_output(cpu);
    push_string_input(
        cpu,
        "\
NOT T T
AND A T
AND B T 
AND C T 
NOT T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN\n",
    );
    cpu.run();
    print_output(cpu).unwrap_or(0)
}

fn print_output(cpu: &mut Computer) -> Option<isize> {
    let (text, score) = cpu.drain_output_to_string_and_score();
    print!("{}", text);
    if let Some(score) = score {
        println!("score: {}", score);
    }
    score
}

fn push_string_input(cpu: &mut Computer, ins: &str) {
    println!(">> {}", ins);
    for ch in ins.chars() {
        cpu.push_input((ch as u32).try_into().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 19_357_290);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1_136_394_042);
    }
}
