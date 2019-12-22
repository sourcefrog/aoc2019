use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("{}", solve_a());
}

fn solve_a() -> usize {
    let orig_cpu = Computer::from_file("input/input19.txt");

    let mut tracted = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut cpu = orig_cpu.clone();
            cpu.push_input(x);
            cpu.push_input(y);
            cpu.run();
            assert_eq!(cpu.output_len(), 1);
            match cpu.pop_output().unwrap() {
                1 => {
                    print!("#");
                    tracted += 1;
                }
                0 => {
                    print!(".");
                }
                _ => panic!(),
            }
        }
        println!();
    }
    tracted
}
