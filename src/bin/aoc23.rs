#![allow(dead_code)]

use std::collections::VecDeque;
use std::convert::TryInto;

use mbp_aoc2019::intcode::Computer;

const NCPU: usize = 50;

pub fn main() {
    println!("23a: {}", solve_a());
}

struct Net {
    /// Output bytes produced by this computer and not yet dispatched.
    /// They're stored here rather than being pushed to the input
    /// queue directly because we might not have the whole packet
    /// when the computer stops.
    outqs: Vec<VecDeque<isize>>,
    cpus: Vec<Computer>,
}

impl Net {
    fn new() -> Net {
        let orig_cpu = Computer::from_file("input/input23.txt");
        let mut cpus = vec![orig_cpu; NCPU];
        for (i, cpu) in cpus.iter_mut().enumerate() {
            cpu.push_input(i as isize);
        }
        Net {
            outqs: vec![VecDeque::new(); NCPU],
            cpus,
        }
    }

    fn run(&mut self) -> isize {
        // Run all the computers quasi-simultaneously, but stop them when
        // they either want input or produce output.
        loop {
            for n in 0..NCPU {
                println!("run {}", n);
                if self.cpus[n].wants_input() && self.cpus[n].input_len() == 0 {
                    self.cpus[n].push_input(-1)
                }
                self.cpus[n].run();

                let outq = &mut self.outqs[n];
                outq.extend(self.cpus[n].drain_output());

                // See if there are any whole packets and if so dispatch.
                while outq.len() >= 3 {
                    let dest: usize = outq.pop_front().unwrap().try_into().unwrap();
                    let x = outq.pop_front().unwrap();
                    let y = outq.pop_front().unwrap();

                    println!("{:2} => {:2} : {}, {}", n, dest, x, y);

                    if dest == 255 {
                        return y;
                    }
                    self.cpus[dest].push_input(x);
                    self.cpus[dest].push_input(y);
                }
            }
        }
    }
}

fn solve_a() -> isize {
    Net::new().run()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 20665);
    }
}
