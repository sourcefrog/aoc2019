use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("02a: {}", solve_a());
    println!("02b: {}", solve_b());
}

fn solve_a() -> isize {
    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?
    let mut c = Computer::from_string(&load_input());
    c.poke_at(1, 12);
    c.poke_at(2, 2);
    c.run();
    c.borrow_mem()[0]
}

fn solve_b() -> isize {
    // Find values a and b, each between 0 and 99 inclusive, stored in
    // address 1 and 2, that lead to noun final output value of 19690720 in
    // address 0.
    //
    // NOTE: In principle we could cause an infinite loop, and so the
    // tests might need to cap processing cycles. However this doesn't seem
    // to actually happen on this input, so I won't worry.
    let orig_ic = Computer::from_string(&load_input());
    let desired = 19_690_720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            // dbg!(noun, verb);
            let mut ic = orig_ic.clone();
            ic.poke_at(1, noun);
            ic.poke_at(2, verb);
            ic.run();
            let output = ic.borrow_mem()[0];
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 3_790_689);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 6533);
    }
}
