// use std::cmp::max;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// Track angles as minimal ratios?

/// x, y; origin at top left
type Point = (isize, isize);

pub fn main() {
    println!("10a: {}", solve_a());
    dbg!(solve_b());
}

fn solve_a() -> usize {
    solve_type_a(&std::fs::read_to_string("input/input10.txt").unwrap())
}

fn solve_type_a(s: &str) -> usize {
    let asts = parse(s);
    asts.iter()
        .map(|obs| (visible(&asts, *obs), *obs))
        .max()
        .unwrap()
        .0
}

fn solve_b() -> isize {
    solve_type_b(&std::fs::read_to_string("input/input10.txt").unwrap())
}

#[allow(clippy::many_single_char_names)]
fn solve_type_b(s: &str) -> isize {
    let asts = parse(s);
    let obs = asts
        .iter()
        .map(|obs| (visible(&asts, *obs), *obs))
        .max()
        .unwrap()
        .1;
    println!("from obs {:?}", obs);

    let mut q = BTreeMap::<NF64, BTreeSet<(NF64, Point)>>::new();
    for p in asts {
        let d = (p.0 - obs.0, p.1 - obs.1);
        if d == (0, 0) {
            continue;
        }
        let (a, r) = to_polar(d);
        q.entry(a).or_default().insert((r, p));
    }

    let mut zz = vec![];
    while !q.is_empty() {
        // Take the next element where the first part of the key, the angle,
        // is greater than ang.  But take care one directly above is taken
        // first.
        let mut zapped = false;
        for (k, v) in q.iter_mut() {
            if let Some((r, p)) = v.iter().cloned().next() {
                println!("zap #{} {:?} angle {:?} range {:?}", zz.len() + 1, p, k, r);
                v.remove(&(r, p));
                zz.push(p);
                zapped = true;
            }
        }
        if !zapped {
            break;
        }
    }
    let p200 = zz[199];
    p200.0 * 100 + p200.1
}

#[derive(Debug, Clone, Copy)]
struct NF64(f64);

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

impl Ord for NF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for NF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for NF64 {}

impl PartialEq for NF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

/// Given a coordinate in integers, return angle clockwise from up
/// in radians, and distance in units.
fn to_polar(p: Point) -> (NF64, NF64) {
    let x = p.0 as f64;
    let y = p.1 as f64;
    let mut ang = NF64(x.atan2(-y));
    if ang.0 < 0f64 {
        ang.0 += 2f64 * std::f64::consts::PI;
    }
    let radius = NF64((x * x + y * y).sqrt());
    (ang, radius)
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
    let uniq = asts
        .iter()
        .map(|a| (a.0 - obs.0, a.1 - obs.1))
        .filter(|d| *d != (0, 0))
        .map(|d| to_polar(d).0)
        .collect::<BTreeSet<_>>();
    // println!("from {:?} can see {}: {:?}", obs, uniq.len(), &uniq);
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
    // println!("asts={:?}", &v);
    v
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_a() {
        assert_eq!(
            solve_type_a(
                "\
                .#..#
                .....
                #####
                ....#
                ...##"
            ),
            8
        );

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

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 278);
    }

    #[test]
    fn b_large() {
        assert_eq!(
            solve_type_b(
                "\
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
            ",
            ),
            802
        );
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1417);
    }
}
