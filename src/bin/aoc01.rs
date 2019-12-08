use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("A: {}", solve_a());
    println!("B: {}", solve_b());
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

fn solve_b() -> u32 {
    recursive_fuel(&load_input().collect::<Vec<_>>())
}

fn recursive_fuel(m: &[u32]) -> u32 {
    let mut m = m.to_vec();
    let mut total = 0;
    loop {
        m = m.into_iter().map(fuel_for_mass).collect();
        match m.iter().sum() {
            0 => return total,
            ns => total += ns,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 3334297);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 4998565);
    }

    #[test]
    fn recursion_examples() {
        assert_eq!(recursive_fuel(&[14]), 2);
        assert_eq!(recursive_fuel(&[1969]), 966);
        assert_eq!(recursive_fuel(&[100756]), 50346);
    }
}
