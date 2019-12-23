use std::convert::TryInto;

use mbp_aoc2019::intcode::Computer;

const NCPU: usize = 50;

pub fn main() {
    println!("23a: {}", solve_a());
    println!("23b: {}", solve_b());
}

struct Net {
    cpus: Vec<Computer>,
    nat_mem: Vec<isize>,
    last_nat_y: Option<isize>,
}

impl Net {
    fn new() -> Net {
        let orig_cpu = Computer::from_file("input/input23.txt");
        let mut cpus = vec![orig_cpu; NCPU];
        for (i, cpu) in cpus.iter_mut().enumerate() {
            cpu.push_input(i as isize);
        }
        Net {
            cpus,
            nat_mem: vec![],
            last_nat_y: None,
        }
    }

    fn run_type_a(&mut self) -> isize {
        loop {
            self.run_all_cpus();
            self.dispatch_completed_packets();
            if self.nat_mem.len() == 2 {
                return self.nat_mem[1];
            }
        }
    }

    fn run_type_b(&mut self) -> isize {
        loop {
            self.run_all_cpus();
            self.dispatch_completed_packets();
            if self.all_are_idle() {
                if let Some(y) = self.dispatch_nat_mem() {
                    return y;
                }
            }
        }
    }

    fn dispatch_nat_mem(&mut self) -> Option<isize> {
        if self.nat_mem.is_empty() {
            // println!("all are idle but nothing in nat mem");
            return None;
        }
        // println!("all are idle: sending {:?} to cpu 0", self.nat_mem);
        assert_eq!(self.nat_mem.len(), 2);
        let x = self.nat_mem[0];
        let y = self.nat_mem[1];
        if self.last_nat_y == Some(y) {
            return Some(y);
        }
        self.last_nat_y = Some(y);
        self.cpus[0].push_input(x);
        self.cpus[0].push_input(y);
        None
    }

    fn run_all_cpus(&mut self) {
        // Run all the computers quasi-simultaneously, but stop them when
        // they either want input or produce output.
        // println!("run all");
        for (n, cpu) in self.cpus.iter_mut().enumerate() {
            // println!("run {}", n);
            let _ = n;
            if cpu.wants_input() && cpu.input_len() == 0 {
                cpu.push_input(-1);
            }
            cpu.run();
        }
    }

    fn all_are_idle(&self) -> bool {
        let mut all_idle = true;
        for (n, cpu) in self.cpus.iter().enumerate() {
            let _ = n;
            if cpu.output_len() == 0 && cpu.wants_input() && cpu.input_len() == 0 {
                // println!("{:2} is idle", n);
            } else {
                all_idle = false
            }
        }
        all_idle
    }

    fn dispatch_completed_packets(&mut self) {
        for n in 0..NCPU {
            while self.cpus[n].output_len() >= 3 {
                let dest: usize = self.cpus[n].pop_output().unwrap().try_into().unwrap();
                let x = self.cpus[n].pop_output().unwrap();
                let y = self.cpus[n].pop_output().unwrap();
                // println!("{:2} => {:2} : {}, {}", n, dest, x, y);
                if dest == 255 {
                    self.nat_mem = vec![x, y];
                } else {
                    self.cpus[dest].push_input(x);
                    self.cpus[dest].push_input(y);
                }
            }
        }
    }
}

fn solve_a() -> isize {
    Net::new().run_type_a()
}

fn solve_b() -> isize {
    Net::new().run_type_b()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 20665);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 13358);
    }
}
