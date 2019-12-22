#![allow(dead_code)]

use std::convert::TryInto;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("21a: {}", solve_a());
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

fn print_output(cpu: &mut Computer) -> Option<isize> {
    let (text, score) = output_to_string_and_score(cpu);
    print!("{}", text);
    if let Some(score) = score {
        println!("score: {}", score);
    }
    score
}

fn output_to_string_and_score(cpu: &mut Computer) -> (String, Option<isize>) {
    let mut s = String::with_capacity(cpu.output_len());
    let mut score = None;
    for c in cpu.drain_output() {
        match std::char::from_u32(c.try_into().unwrap()) {
            Some(ch) => s.push(ch),
            None => score = Some(c),
        }
    }
    (s, score)
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
}
