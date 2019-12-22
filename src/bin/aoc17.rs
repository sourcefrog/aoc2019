// #![allow(dead_code, unused_imports)]

use std::convert::TryFrom;

use mbp_aoc2019::intcode::Computer;
use mbp_aoc2019::Matrix;

pub fn main() {
    println!("17a: {}", solve_a());
}

fn is_scaffold(ch: char) -> bool {
    match ch {
        '#' | '^' | 'v' | '<' | '>' => true,
        '.' | 'X' => false,
        _ => panic!("unexpected char {:?}", ch),
    }
}

fn solve_a() -> usize {
    let mat = load_map();
    let mut marked = mat.clone();
    let mut align = 0;
    // Count neighbors by looking at every point to see if it is a beam
    // and there is a beam above, below, left, and right.
    for p in mat.iter_points() {
        if is_scaffold(mat[p])
            && (p.x > 0 && is_scaffold(mat[p.left()]))
            && (p.x < mat.width() - 1 && is_scaffold(mat[p.right()]))
            && (p.y > 0 && is_scaffold(mat[p.up()]))
            && (p.y < mat.height() - 1 && is_scaffold(mat[p.down()]))
        {
            marked[p] = '$';
            align += p.x * p.y
        }
    }
    // println!("{}", marked.to_string_lines());
    align
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
