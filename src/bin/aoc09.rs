use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("09a: {}", solve_a());
    println!("09b: {}", solve_b());
}

fn solve_a() -> isize {
    let mut c = Computer::from_file("input/input09.txt");
    c.push_input(1);
    c.run();
    let o = c.drain_output();
    dbg!(&o);
    *o.last().unwrap()
}

fn solve_b() -> isize {
    let mut c = Computer::from_file("input/input09.txt");
    c.push_input(2);
    c.run();
    let o = c.drain_output();
    dbg!(&o);
    *o.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 2_789_104_029);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 32869);
    }
}
