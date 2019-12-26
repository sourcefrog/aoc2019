// use std::io::BufRead;
use std::io::prelude::*;

use mbp_aoc2019::intcode::Computer;

// Avoid: giant electromagnet, photons, infinite loop, escape pod

// S: engineering / infinite loop
// SS :

pub fn main() {
    let mut cpu = Computer::from_file("input/input25.txt");
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut in_lines = stdin.lock().lines();
    let mut out = stdout.lock();
    loop {
        cpu.run();
        out.write_all(&cpu.drain_output_string().as_bytes()).unwrap();
        if cpu.is_halted() {
            break
        } else if cpu.wants_input() {
            let mut l: String = in_lines.next().unwrap().unwrap();
            l.push('\n');
            cpu.push_input_string(&l);
        }
    }
}