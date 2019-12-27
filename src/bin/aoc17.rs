#![allow(dead_code, unused_imports)]

use std::convert::TryFrom;

use mbp_aoc2019::intcode::Computer;
use mbp_aoc2019::Matrix;

pub fn main() {
    println!("17a: {}", solve_a());
    println!("17b: {}", solve_b());
}

fn is_scaffold(ch: Option<char>) -> bool {
    if let Some(ch) = ch {
        match ch {
            '#' | '^' | 'v' | '<' | '>' => true,
            '.' | 'X' => false,
            _ => panic!("unexpected char {:?}", ch),
        }
    } else {
        false
    }
}

fn solve_a() -> isize {
    let mat = load_map();
    let mut marked = mat.clone();
    let mut align = 0;
    // Count neighbors by looking at every point to see if it is a beam
    // and there is a beam above, below, left, and right.
    for p in mat.iter_points() {
        if is_scaffold(mat.try_get(p))
            && is_scaffold(mat.try_get(p.left()))
            && is_scaffold(mat.try_get(p.right()))
            && is_scaffold(mat.try_get(p.up()))
            && is_scaffold(mat.try_get(p.down()))
        {
            marked[p] = '$';
            align += p.x * p.y
        }
    }
    println!("{}", marked.to_string_lines());
    align
}

fn solve_b() -> isize {
    let mut cpu = Computer::from_file("input/input17.txt");
    cpu.poke_at(0, 2);
    let score = cpu.interact();
    println!("score: {}", score.unwrap_or(0));
    score.unwrap()
}

fn load_map() -> Matrix<char> {
    let mut c = Computer::from_file("input/input17.txt");
    c.run();
    let map_str: String = c
        .drain_output()
        .iter()
        .map(|i| char::try_from(*i as u32).unwrap())
        .collect();
    // println!("{}", map_str);
    Matrix::from_string_lines(&map_str)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 3888);
    }
}
