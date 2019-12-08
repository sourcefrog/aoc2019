use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("{}", solve_a());
}

fn solve_a() -> u32 {
    load_input().map(fuel_for_mass).sum()
}

fn fuel_for_mass(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn load_input() -> impl Iterator<Item = u32> {
    let f = BufReader::new(File::open("input/input01.txt").unwrap());
    f.lines().map(|l| l.unwrap().parse::<u32>().unwrap())
}
