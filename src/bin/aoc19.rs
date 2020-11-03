use std::cmp::max;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("19a: {}", solve_a());
    println!("19b: {}", solve_b());
}

fn load_program() -> Computer {
    Computer::from_file("input/input19.txt")
}

fn solve_a() -> usize {
    let program = load_program();
    let mut tracted = 0;
    for y in 0..50 {
        for x in 0..50 {
            if is_lit(&program, x, y) {
                tracted += 1;
            }
        }
    }
    tracted
}

#[allow(dead_code)]
fn show_map(program: &Computer) {
    for y in 0..50 {
        for x in 0..50 {
            if is_lit(&program, x, y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_b() -> isize {
    solve_type_b(&load_program())
}

const SQUARE: isize = 100;

/// Find the combined top-left coordinates of the closest SQUARE size that fits
/// entirely within the beam.
///
/// Bisect through rows, finding the size of the square starting on that row
/// that fits in the beam, until we find the smallest adequate answer.
fn solve_type_b(program: &Computer) -> isize {
    let mut min_first = 0;
    let mut min_last = 0;
    for y in 0.. {
        // dbg!(y);
        // Is anything lit on this row, and if so where does it start?
        if let Some(top_first) = first_lit(program, min_first, y) {
            let top_last = last_lit(program, max(top_first, min_last), y);
            min_first = top_first; // No need to look left of this on later rows
            min_last = top_last;
            let square_x = top_last - SQUARE + 1;
            if square_x < top_first {
                // Not even enough room on this row
                continue;
            }
            // Now look down from here to see if there is a full square.
            if is_lit(program, square_x, y + SQUARE - 1) {
                // dbg!(square_x, y);
                debug_assert!(is_lit(program, square_x + SQUARE - 1, y));
                debug_assert!(is_lit(program, square_x + SQUARE - 1, y + SQUARE - 1));
                return square_x * 10_000 + y;
            }
        }
    }
    unreachable!()
}

/// Return the x coordinate of the first lit cell in row Y, at at least min_x, if any.
fn first_lit(program: &Computer, min_x: isize, y: isize) -> Option<isize> {
    // Some rows have nothing lit, so we have a kinda kludgey assumption that we will find
    // nothing too far to the right of the unit slope.
    for x in min_x..max(10, 2 * y) {
        if is_lit(program, x, y) {
            return Some(x);
        }
    }
    None
}

/// Find the x coordinate of the last lit cell in row Y, guaranteed to be at least
/// min_x, and where min_x is guaranteed to be lit.
fn last_lit(program: &Computer, min_x: isize, y: isize) -> isize {
    debug_assert!(is_lit(program, min_x, y), "{}, {} is not lit", min_x, y);
    for x in (min_x + 1).. {
        if !is_lit(program, x, y) {
            debug_assert!(x > min_x);
            return x - 1;
        }
    }
    unreachable!()
}

/// True if the cell at (x, y) is lit.
fn is_lit(program: &Computer, x: isize, y: isize) -> bool {
    // println!("is_lit ({}, {})", x, y);
    let mut cpu = program.clone();
    assert!(x >= 0);
    assert!(y >= 0);
    cpu.push_input(x);
    cpu.push_input(y);
    cpu.run();
    assert_eq!(cpu.output_len(), 1);
    match cpu.pop_output().unwrap() {
        1 => true,
        0 => false,
        other => panic!("unexpected output {:?}", other),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 150);
    }

    #[test]
    fn is_lit_samples() {
        let program = load_program();
        assert_eq!(is_lit(&program, 0, 0), true);
        assert_eq!(is_lit(&program, 0, 1), false);
        assert_eq!(is_lit(&program, 0, 10), false);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 12201460);
    }
}
