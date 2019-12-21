use std::convert::TryFrom;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    let mut c = Computer::from_file("input/input17.txt");
    c.run();
    let map: String = c
        .drain_output()
        .iter()
        .map(|i| char::try_from(*i as u32).unwrap())
        .collect();
    println!("{}", map);
}
