use mbp_aoc2019::intcode::Computer;

pub fn main() {
    let mut cpu = Computer::from_file("input/input25.txt");
    cpu.interact();
}