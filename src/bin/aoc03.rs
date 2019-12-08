#[macro_use]
extern crate itertools;

use std::convert::{identity, TryInto};

pub fn main() {
    println!("03a: {}", solve_a())
}

fn solve_a() -> usize {
    closest_intersection_from_strings(&load_input()).unwrap()
}

fn closest_intersection_from_strings(s: &str) -> Option<usize> {
    let wss: Vec<_> = s.lines().map(str::trim).map(parse_path).collect();
    assert_eq!(wss.len(), 2);
    intersect_any(&wss[0], &wss[1])
}

fn load_input() -> String {
    std::fs::read_to_string("input/input03.txt").unwrap()
}

// x left-to-right, y top-to-bottom
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum Wire {
    Horz { xmin: isize, xmax: isize, y: isize },
    Vert { x: isize, ymin: isize, ymax: isize },
}
use Wire::*;

impl Wire {
    /// Return the minimal Manhattan distance to origin of the intersection
    /// of these two wires, if any.
    ///
    /// Only wires in opposite orientations can intersect.
    fn intersect(&self, w2: &Wire) -> Option<usize> {
        match (self, w2) {
            (Horz { .. }, Horz { .. }) => None,
            (Vert { .. }, Vert { .. }) => None,
            (Horz { .. }, Vert { .. }) => w2.intersect(self),
            (Vert { x: vx, ymin, ymax }, Horz { xmin, xmax, y: hy }) => {
                if vx < xmin || vx > xmax || hy < ymin || hy > ymax {
                    None
                } else if *vx == 0 && *hy == 0 {
                    // Intersection at the origin doesn't count
                    None
                } else {
                    // dbg!((vx, hy));
                    Some((vx.abs() + hy.abs()).try_into().unwrap())
                }
            }
        }
    }
}

/// Find the closest-to-origin intersection between the two sets of
/// wires.
fn intersect_any(ws1: &[Wire], ws2: &[Wire]) -> Option<usize> {
    iproduct!(ws1, ws2)
        .map(|(w1, w2)| w1.intersect(w2))
        .filter_map(identity)
        .min()
}

/// Parse a comma-separated list of steps into a vec of wires.
fn parse_path(s: &str) -> Vec<Wire> {
    let mut p = Point { x: 0, y: 0 };
    let mut w = Vec::new();
    for seg in s.split(',') {
        let l = seg[1..].parse::<isize>().expect(seg);
        let x = p.x;
        let y = p.y;
        let dir_char = seg.chars().next().unwrap();
        w.push(match dir_char {
            'U' => Vert {
                x,
                ymin: y - l,
                ymax: y,
            },
            'D' => Vert {
                x,
                ymin: y,
                ymax: y + l,
            },
            'R' => Horz {
                xmin: x,
                xmax: x + l,
                y,
            },
            'L' => Horz {
                xmin: x - l,
                xmax: x,
                y,
            },
            _ => panic!("error in path: {:?}", seg),
        });
        p = match dir_char {
            'U' => Point { x, y: y - l },
            'D' => Point { x, y: y + l },
            'L' => Point { x: x - l, y },
            'R' => Point { x: x + l, y },
            _ => panic!("error in path: {:?}", seg),
        };
        // println!("p={:?}, w.last()={:?}", &p, &w.last().unwrap());
    }
    w
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let w1 = parse_path("R8,U5,L5,D3");
        let w2 = parse_path("U7,R6,D4,L4");
        assert_eq!(intersect_any(&w1, &w2), Some(6));
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 248);
    }
}
