use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("15a: {}", solve_a());
}

fn solve_a() -> usize {
    // Do a breadth-first traversal from the starting point until we find
    // the oxygen. 
    //
    // By looking at all squares N positions away, before looking any deeper,
    // we should find the shortest path first.
    let mut c = Computer::from_file("input/input15.txt");

    0
}
