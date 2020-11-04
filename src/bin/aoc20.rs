//! AoC 2019-20: Find the shortest path through a (recursive) maze.

#![allow(unused_imports, dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

use mbp_aoc2019::shortest_path::shortest_distance;
use mbp_aoc2019::{point, Matrix, Point};

const PASSAGE: char = '.';

type Label = String;

pub fn main() {
    println!("20a: {}", solve_a());
    println!("20b: {}", solve_b());
}

fn solve_a() -> isize {
    Maze::from_input_file().single_level_path()
}

fn solve_b() -> isize {
    Maze::from_input_file().multi_level_path()
}

/// Describes one level of the possibly-recursive map.
struct Maze {
    /// Character-matrix representation.
    matrix: Matrix<char>,

    /// Map from the two-letter labels to the labelled positions/s on the maze.
    /// (For AA and ZZ there should be only one labelled point; for everything else
    /// there should be two.)
    labels: HashMap<Label, Vec<Point>>,

    /// For twinned portals, a symmetric map from the entry to the exit.
    warps: HashMap<Point, Point>,

    /// Points on the inside that connect downward to a smaller maze.
    warp_down: HashMap<Point, Point>,
    warp_up: HashMap<Point, Point>,

    /// Memoized same-level neighbors.
    memo_neighbors: RefCell<HashMap<Point, Vec<Point>>>,
}

/// A point in 3d-space
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point3 {
    /// Depth into the maze, where the entrance and exit are at 0
    depth: isize,
    x: isize,
    y: isize,
}

impl Point3 {
    fn at_depth(p: Point, depth: isize) -> Point3 {
        Point3 {
            x: p.x,
            y: p.y,
            depth,
        }
    }
}

impl Maze {
    pub fn from_string(s: &str) -> Maze {
        let matrix = Matrix::from_string_lines(s);
        let labels = find_labels(&matrix);
        let mut warps = HashMap::new();
        for (label, points) in &labels {
            if label == "AA" || label == "ZZ" {
                assert_eq!(points.len(), 1);
                continue;
            }
            assert_eq!(points.len(), 2);
            let exists = warps.insert(points[0], points[1]).is_some();
            assert!(!exists);
            let exists = warps.insert(points[1], points[0]).is_some();
            assert!(!exists);
        }
        let mut warp_down = HashMap::new();
        let mut warp_up = HashMap::new();
        // Coordinates for the outer ring
        let top = 2;
        let bottom = (matrix.height() - 3) as isize;
        let left = 2;
        let right = (matrix.width() - 3) as isize;
        let is_outside = |p: &Point| p.y == top || p.y == bottom || p.x == left || p.x == right;
        for (p1, p2) in &warps {
            assert!(is_outside(p1) != is_outside(p2));
            if is_outside(p1) {
                warp_up.insert(*p1, *p2);
            } else {
                warp_down.insert(*p1, *p2);
            }
        }
        Maze {
            matrix,
            labels,
            warps,
            warp_down,
            warp_up,
            memo_neighbors: RefCell::new(HashMap::new()),
        }
    }

    pub fn from_file(path: &str) -> Maze {
        Maze::from_string(&std::fs::read_to_string(path).unwrap())
    }

    pub fn from_input_file() -> Maze {
        Maze::from_file("input/input20.txt")
    }

    /// Find the entry or exit portal.
    fn find_single_portal(&self, name: &str) -> Point {
        debug_assert!(name == "AA" || name == "ZZ");
        match self.labels.get(name).map(Vec::as_slice) {
            Some([p1]) => *p1,
            other => panic!("expected one point at {:?}, got {:?}", name, other),
        }
    }

    fn entrance(&self) -> Point {
        self.find_single_portal("AA")
    }

    fn entrance3(&self) -> Point3 {
        let p = self.entrance();
        Point3 {
            depth: 0,
            x: p.x,
            y: p.y,
        }
    }

    fn exit(&self) -> Point {
        self.find_single_portal("ZZ")
    }

    fn exit3(&self) -> Point3 {
        let p = self.exit();
        Point3 {
            depth: 0,
            x: p.x,
            y: p.y,
        }
    }

    /// Return the same-level neighbors, without portals.
    fn flat_neighbors(&self, p: Point) -> Vec<Point> {
        self.memo_neighbors
            .borrow_mut()
            .entry(p)
            .or_insert_with(|| {
                self.matrix
                    .neighbors4(p)
                    .into_iter()
                    .filter(|(_, c1)| **c1 == PASSAGE)
                    .map(|(p1, _)| p1)
                    .collect()
            })
            .clone()
    }

    /// Return the neighbors of point `p` in a single-level maze.
    ///
    /// `p` must be a passage square.
    ///
    /// That is: immediately neighboring other passage squares, or if there is a neighboring portal
    /// that has a twin, you can warp to the square outside its twin. All of these are one step.
    fn single_level_neighbors(&self, p: Point) -> Vec<(Point, isize)> {
        debug_assert_eq!(self.matrix.try_get(p).unwrap(), PASSAGE);
        let mut n: Vec<(Point, isize)> =
            self.flat_neighbors(p).into_iter().map(|p| (p, 1)).collect();
        if let Some(out_p) = self.warps.get(&p) {
            n.push((*out_p, 1))
        }
        n
    }

    /// Return the neighbors of point `p` in a multi-level maze.
    ///
    /// They are: every direct neighbor at the same depth, plus traversal
    /// downwards through the inner warps, and upward through the outer warps.
    fn multi_level_neighbors(&self, p3: Point3) -> Vec<(Point3, isize)> {
        let flatp = point(p3.x, p3.y);
        let depth = p3.depth;
        let mut n: Vec<(Point3, isize)> = self
            .flat_neighbors(flatp)
            .into_iter()
            .map(|p1| (Point3::at_depth(p1, depth), 1))
            .collect();
        // if let Some(out_p) = self.warps.get(&p) { n.push((*out_p, 1)) }
        if depth > 0 {
            if let Some(up) = self.warp_up.get(&flatp) {
                n.push((Point3::at_depth(*up, depth - 1), 1));
            }
        }
        if let Some(down) = self.warp_down.get(&flatp) {
            // println!("Down to {}", depth + 1);
            n.push((Point3::at_depth(*down, depth + 1), 1));
        }
        n
    }

    /// Find the length of the shortest path from AA to ZZ in a single-level maze.
    fn single_level_path(&self) -> isize {
        shortest_distance(self.entrance(), self.exit(), &mut |p| {
            self.single_level_neighbors(p)
        })
    }

    /// Find the shortest path in a recursive multi-level maze.
    fn multi_level_path(&self) -> isize {
        shortest_distance(self.entrance3(), self.exit3(), &mut |p3| {
            self.multi_level_neighbors(p3)
        })
    }
}

/// Return a map from labels to the 1 or 2 points they label.
fn find_labels(mat: &Matrix<char>) -> HashMap<Label, Vec<Point>> {
    let mut v: HashMap<Label, Vec<Point>> = HashMap::new();
    // Trick here is not to be confused by finding the second character
    // of the labels...
    //
    // Walk over the matrix looking for letters.
    //
    // If we find one, look below it, or to the right of it, for another
    // letter. If there is one, that tells us there's a marker, and its
    // orientation. Then look at one square more, on either side, to find
    // a '.' marker, which tells us the location of the actual portal.
    let mut found = |name: Label, p: Point| v.entry(name).or_default().push(p);

    for p in mat.iter_points() {
        let c1 = mat[p];
        if !c1.is_ascii_uppercase() {
            continue;
        }
        let pright = p.right();
        let pdown = p.down();
        if let Some(cright) = mat.try_get(pright) {
            if cright.is_ascii_uppercase() {
                let name: String = [c1, cright].iter().collect();
                // Is there a dot to the left of these, or to the right?
                if mat.try_get(p.left()) == Some(PASSAGE) {
                    found(name, p.left());
                } else if mat.try_get(pright.right()) == Some(PASSAGE) {
                    found(name, pright.right());
                } else {
                    panic!("confused at {:?}", p);
                }
                continue;
            }
        }
        if let Some(cdown) = mat.try_get(pdown) {
            if cdown.is_ascii_uppercase() {
                let name = [c1, cdown].iter().collect();
                if mat.try_get(p.up()) == Some(PASSAGE) {
                    found(name, p.up());
                } else if mat.try_get(pdown.down()) == Some(PASSAGE) {
                    found(name, pdown.down());
                } else {
                    panic!("portal at {:?} does not seem to be near a passage", p);
                }
            }
        }
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_map() {
        let map = Maze::from_input_file();
        assert_eq!(map.entrance(), point(49, 2));
        assert_eq!(map.exit(), point(37, 106));
    }

    #[test]
    fn example_a_1_without_warps() {
        let mut map = Maze::from_file("input/example_20_1.txt");
        map.warps.clear();
        assert_eq!(map.single_level_path(), 26);
    }

    #[test]
    fn example_a_1_new() {
        let map = Maze::from_file("input/example_20_1.txt");
        assert_eq!(
            *map.labels.get("BC").unwrap(),
            vec![point(9, 6), point(2, 8)]
        );
        // Warps through BC exist
        assert_eq!(*map.warps.get(&point(2, 8)).unwrap(), point(9, 6));
        assert_eq!(*map.warps.get(&point(9, 6)).unwrap(), point(2, 8));
        assert_eq!(map.single_level_path(), 23);
    }

    #[test]
    fn example_a_2() {
        let map = Maze::from_file("input/example_20_2.txt");
        assert_eq!(map.single_level_path(), 58);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 454);
    }

    #[test]
    fn example_b_1() {
        let map = Maze::from_file("input/example_20_1.txt");
        assert_eq!(map.multi_level_path(), 26);
    }

    #[test]
    fn example_b_3() {
        let map = Maze::from_file("input/example_20_3.txt");
        assert_eq!(map.multi_level_path(), 396);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 5744);
    }
}
