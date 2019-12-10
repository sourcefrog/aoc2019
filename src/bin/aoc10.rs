// 294 is too high...

use std::cmp::max;
use std::collections::BTreeSet;

// Track angles as minimal ratios?

/// x, y; origin at top left
type Point = (isize, isize);

pub fn main() {
    println!("10a: {}", solve_a());
}

fn solve_a() -> usize {
    solve_type_a(&std::fs::read_to_string("input/input10.txt").unwrap())
}

fn solve_type_a(s: &str) -> usize {
    let asts = parse(s);
    let mut best_vis = 0;
    for obs in &asts {
        best_vis = max(best_vis, visible(&asts, *obs));
    }
    best_vis
}

fn angle_lcd(mut p: Point) -> Point {
    for i in (2..50).rev() {
        if (p.0 % i == 0) && (p.1 % i == 0) {
            p = (p.0 / i, p.1 / i)
        }
    }
    p
}

fn visible(asts: &[Point], obs: Point) -> usize {
    // Find the angle, expressed as a ratio, to every asteroid.
    //
    // Reduce them to lowest-common denominators, preserving the sign of both the x and y
    // component.
    // 
    // Don't count the one we're sitting on
    //
    // How many are unique?
    let uniq = asts.iter()
        .map(|a| (a.0 - obs.0, a.1 - obs.1))
        .filter(|d| *d != (0,0))
        .map(angle_lcd)
        .collect::<BTreeSet<_>>();
    println!("from {:?} can see {}: {:?}", obs, uniq.len(), &uniq);
    uniq.len()
}

fn parse(s: &str) -> Vec<Point> {
    let mut v = Vec::new();
    for (y, l) in s.lines().map(str::trim).enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => v.push((x as isize, y as isize)),
                '.' => (),
                _ => panic!("unexpected char {:?}", c),
            }
        }
    }
    println!("asts={:?}", &v);
    v
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lcd() {
        assert_eq!(angle_lcd((6, 6)), (1, 1));
        assert_eq!(angle_lcd((6, 1)), (6, 1));
        assert_eq!(angle_lcd((-6, -1)), (-6, -1));
        assert_eq!(angle_lcd((7, -2)), (7, -2));
        assert_eq!(angle_lcd((-10, 2)), (-5, 1));
        assert_eq!(angle_lcd((4, 0)), (1, 0));
    }

    #[test]
    fn example_a() {
        assert_eq!(solve_type_a(
            "\
.#..#
.....
#####
....#
...##"), 8);

        assert_eq!(
            solve_type_a(
                "\
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####"
            ),
            33
        );
    }
}
