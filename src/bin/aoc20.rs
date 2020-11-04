//! AoC 2019-20: Find the shortest path through a (recursive) maze.

#![allow(unused_imports, dead_code)]

use std::collections::BTreeMap;
use std::path::Path;

use mbp_aoc2019::shortest_path::shortest_distance;
use mbp_aoc2019::{point, Matrix, Point};

const PASSAGE: char = '.';

type Label = String;

pub fn main() {
    println!("20a: {}", solve_a());
}

fn solve_a() -> isize {
    MapLevel::from_input_file().single_level_path()
}

/// Describes one level of the possibly-recursive map.
struct MapLevel {
    /// Character-matrix representation.
    matrix: Matrix<char>,

    /// Map from the two-letter labels to the labelled positions/s on the map.
    /// (For AA and ZZ there should be only one labelled point; for everything else
    /// there should be two.)
    labels: BTreeMap<Label, Vec<Point>>,

    /// For twinned portals, a symmetric map from the entry to the exit.
    warps: BTreeMap<Point, Point>,
}

impl MapLevel {
    pub fn from_string(s: &str) -> MapLevel {
        let matrix = Matrix::from_string_lines(s);
        let labels = find_labels(&matrix);
        let mut warps = BTreeMap::new();
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
        MapLevel {
            matrix,
            labels,
            warps,
        }
    }

    pub fn from_file(path: &str) -> MapLevel {
        MapLevel::from_string(&std::fs::read_to_string(path).unwrap())
    }

    pub fn from_input_file() -> MapLevel {
        MapLevel::from_file("input/input20.txt")
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

    fn exit(&self) -> Point {
        self.find_single_portal("ZZ")
    }

    /// Return the neighbors of point `p` in a single-level map.
    ///
    /// `p` must be a passage square.
    ///
    /// That is: immediately neighboring other passage squares, or if there is a neighboring portal
    /// that has a twin, you can warp to the square outside its twin. All of these are one step.
    fn single_level_neighbors(&self, p: Point) -> Vec<(Point, isize)> {
        debug_assert_eq!(self.matrix.try_get(p).unwrap(), PASSAGE);
        let mut n: Vec<(Point, isize)> = self
            .matrix
            .neighbors4(p)
            .into_iter()
            .filter(|(_, c1)| **c1 == PASSAGE)
            .map(|(p1, _)| (p1, 1))
            .collect();
        if let Some(out_p) = self.warps.get(&p) {
            n.push((*out_p, 1))
        }
        n
    }

    /// Find the length of the shortest path from AA to ZZ in a single-level map.
    fn single_level_path(&self) -> isize {
        shortest_distance(self.entrance(), self.exit(), &mut |p| {
            self.single_level_neighbors(p)
        })
    }
}

/// Return a map from labels to the 1 or 2 points they label.
fn find_labels(mat: &Matrix<char>) -> BTreeMap<Label, Vec<Point>> {
    let mut v: BTreeMap<Label, Vec<Point>> = BTreeMap::new();
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
    fn example_a_1_without_warps() {
        let mut map = MapLevel::from_file("input/example_20_1.txt");
        map.warps.clear();
        assert_eq!(map.single_level_path(), 26);
    }

    #[test]
    fn example_a_1_new() {
        let map = MapLevel::from_file("input/example_20_1.txt");
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
        let map = MapLevel::from_file("input/example_20_2.txt");
        assert_eq!(map.single_level_path(), 58);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 454);
    }

    #[test]
    fn load_map() {
        let map = MapLevel::from_input_file();
        assert_eq!(map.entrance(), point(49, 2));
        assert_eq!(map.exit(), point(37, 106));
    }
}
