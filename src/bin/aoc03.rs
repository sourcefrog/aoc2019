#[macro_use]
extern crate itertools;

use std::convert::{identity, TryInto};

pub fn main() {
    println!("03a: {}", solve_a());
    println!("03b: {}", solve_b());
}

fn solve_a() -> usize {
    closest_intersection_from_strings(&load_input()).unwrap()
}

fn solve_b() -> usize {
    shortest_intersection_from_strings(&load_input()).unwrap()
}

fn closest_intersection_from_strings(s: &str) -> Option<usize> {
    let wss: Vec<_> = s.lines().map(str::trim).map(parse_path).collect();
    assert_eq!(wss.len(), 2);
    intersect_any(&wss[0], &wss[1])
}

fn shortest_intersection_from_strings(s: &str) -> Option<usize> {
    let wss: Vec<_> = s.lines().map(str::trim).map(parse_path).collect();
    assert_eq!(wss.len(), 2);
    intersect_shortest(&wss[0], &wss[1])
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
    Horz {
        xmin: isize,
        xmax: isize,
        y: isize,
        dist: isize,
        /// The entry point on the x axis
        ex: isize,
    },
    Vert {
        x: isize,
        ymin: isize,
        ymax: isize,
        dist: isize,
        /// The entry point on the y axis
        ey: isize,
    },
}
use Wire::*;

impl Wire {
    /// If these wires intersect, then return a tuple of
    /// the minimal Manhattan distance to origin of the intersection,
    /// and the total distance along both wires to this intersection.
    ///
    /// Only wires in opposite orientations can intersect.
    #[allow(clippy::if_same_then_else)]
    fn intersect(&self, w2: &Wire) -> Option<(usize, usize)> {
        match (self, w2) {
            (Horz { .. }, Horz { .. }) => None,
            (Vert { .. }, Vert { .. }) => None,
            (Horz { .. }, Vert { .. }) => w2.intersect(self),
            (
                Vert {
                    x: vx,
                    ymin,
                    ymax,
                    dist: vd,
                    ey,
                },
                Horz {
                    xmin,
                    xmax,
                    y: hy,
                    dist: hd,
                    ex,
                },
            ) => {
                if vx < xmin || vx > xmax || hy < ymin || hy > ymax {
                    // Don't intersect
                    None
                } else if *vx == 0 && *hy == 0 {
                    // Intersection at the origin doesn't count
                    None
                } else {
                    // dbg!((vx, hy));
                    let man_dist = (vx.abs() + hy.abs()).try_into().unwrap();
                    let path_dist = vd + hd + (hy - ey).abs() + (vx - ex).abs();
                    Some((man_dist, path_dist.try_into().unwrap()))
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
        .map(|(man_dist, _path_dist)| man_dist)
        .min()
}

/// Find the shortest total distance from the start (across both wires).
fn intersect_shortest(ws1: &[Wire], ws2: &[Wire]) -> Option<usize> {
    iproduct!(ws1, ws2)
        .map(|(w1, w2)| w1.intersect(w2))
        .filter_map(identity)
        .map(|(_man_dist, path_dist)| path_dist)
        .min()
}

/// Parse a comma-separated list of steps into a vec of wires.
fn parse_path(path_str: &str) -> Vec<Wire> {
    let mut x = 0;
    let mut y = 0;
    let mut path = Vec::new();
    let mut dist = 0;
    for seg in path_str.split(',') {
        let l = seg[1..].parse::<isize>().expect(seg);
        assert!(l > 0);
        let dir_char = seg.chars().next().unwrap();
        path.push(match dir_char {
            'U' => Vert {
                x,
                ymin: y - l,
                ymax: y,
                dist,
                ey: y,
            },
            'D' => Vert {
                x,
                ymin: y,
                ymax: y + l,
                dist,
                ey: y,
            },
            'R' => Horz {
                xmin: x,
                xmax: x + l,
                y,
                dist,
                ex: x,
            },
            'L' => Horz {
                xmin: x - l,
                xmax: x,
                y,
                dist,
                ex: x,
            },
            _ => panic!("error in path: {:?}", seg),
        });
        match dir_char {
            'U' => y -= l,
            'D' => y += l,
            'L' => x -= l,
            'R' => x += l,
            _ => panic!("error in path: {:?}", seg),
        };
        dist += l;
    }
    path
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let w1 = parse_path("R8,U5,L5,D3");
        let w2 = parse_path("U7,R6,D4,L4");
        assert_eq!(intersect_any(&w1, &w2), Some(6));

        assert_eq!(
            closest_intersection_from_strings(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Some(159)
        );
        assert_eq!(
            closest_intersection_from_strings(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Some(135)
        );
    }

    #[test]
    fn example_b() {
        let w1 = parse_path("R8,U5,L5,D3");
        let w2 = parse_path("U7,R6,D4,L4");
        assert_eq!(intersect_shortest(&w1, &w2), Some(30));

        assert_eq!(
            shortest_intersection_from_strings(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Some(610)
        );
        assert_eq!(
            shortest_intersection_from_strings(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Some(410)
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 248);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 28580);
    }
}
