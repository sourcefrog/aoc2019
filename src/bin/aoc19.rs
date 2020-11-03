use std::cmp::max;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    // println!("19a: {}", solve_a());
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
                print!("#");
                tracted += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
    tracted
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
    let mut lower_bound = 1;
    let mut higher_bound = 10_000;
    let mut best_corner = None;
    while lower_bound + 1 < higher_bound {
        let guess = (lower_bound + higher_bound) / 2;
        if let Some((x, y)) = large_square(program, guess) {
            best_corner = Some((x, y));
            higher_bound = guess
        } else {
            lower_bound = guess
        }
    }
    let (x, y) = best_corner.unwrap();
    x * 10_000 + y
}

/// True if the cell at (x, y) is lit.
fn is_lit(program: &Computer, x: isize, y: isize) -> bool {
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

/// Calculate the inclusive range of lit squares on row Y.
// TODO: Memoize these calculations; they're likely to reoccur.
fn lit_range(program: &Computer, y: isize, min_x: isize) -> Option<(isize, isize)> {
    let mut first_lit = None;
    let mut last_lit = None;
    // Some rows have nothing lit, so we have a kinda kludgey assumption that we will find
    // nothing too far to the right of the unit slope.
    for x in min_x..max(10, 2 * y) {
        if is_lit(program, x, y) {
            if first_lit.is_none() {
                first_lit = Some(x)
            }
            last_lit = Some(x)
        } else {
            if first_lit.is_some() {
                break;
            }
        }
    }
    match (first_lit, last_lit) {
        (Some(first), Some(last)) => Some((first, last)),
        (None, None) => None,
        _ => panic!(),
    }
}

/// Find a full-size square starting on row y, if there is one, and return its top-left corner coordinate.
fn large_square(program: &Computer, y: isize) -> Option<(isize, isize)> {
    if let Some((top_left, top_right)) = lit_range(program, y, 0) {
        if top_right - top_left + 1 < SQUARE {
            None
        } else {
            // Look down SQUARE rows, and find the range there.
            if let Some((bottom_left, bottom_right)) = lit_range(program, y + SQUARE - 1, top_left)
            {
                // They both have some lit squares. Are there at least 'size' columns in common?
                // This all assumes the beam slopes down to the right.
                debug_assert!(bottom_left >= top_left);
                debug_assert!(bottom_right >= top_right);
                if top_right - bottom_left + 1 >= SQUARE {
                    Some((bottom_left, y))
                } else {
                    None
                }
            } else {
                None
            }
        }
    } else {
        None
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

    /// Check lit ranges from the early part of the map against visually
    /// inspected values.
    #[test]
    fn lit_ranges() {
        let program = load_program();
        assert_eq!(lit_range(&program, 0, 0), Some((0, 0)));
        assert_eq!(lit_range(&program, 9, 4), Some((8, 8)));
        assert_eq!(lit_range(&program, 10, 2), Some((8, 9)));
        assert_eq!(lit_range(&program, 40, 0), Some((32, 36)));
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 12201460);
    }
}
