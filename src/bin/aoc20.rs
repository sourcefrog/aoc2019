#![allow(unused_imports, dead_code)]

use mbp_aoc2019::shortest_path::shortest_distance;
use mbp_aoc2019::{point, Matrix, Point};
use std::collections::BTreeMap;

const PASSAGE: char = '.';

type AdjMap = BTreeMap<Point, Vec<Point>>;
type Label = String;
type LabelMap = BTreeMap<Label, Vec<Point>>;

pub fn main() {
    println!("20a: {}", solve_a());
}

fn solve_a() -> isize {
    solve_from_file("input/input20.txt")
}

fn solve_a_from_file(filename: &str) -> isize {
    shortest_aa_zz_path(&std::fs::read_to_string(filename).unwrap())
}

fn shortest_aa_zz_path(s: &str) -> isize {
    let mut mb = Matrix::<char>::from_rows();
    for l in s.lines() {
        let cv: Vec<char> = l.chars().collect();
        mb.add_row(&cv);
    }
    // Raw matrix of just uninterpreted chars.
    let mat = mb.finish();

    let mut adj = find_square_adjacencies(&mat);
    let labels = find_labels(&mat);
    add_portal_adjacencies(&labels, &mut adj);

    let aa = dbg!(find_single_portal(&labels, "AA"));
    let zz = dbg!(find_single_portal(&labels, "ZZ"));

    // Get all neighbors of p, and they're all distance 1.
    let mut neighb = |p| adj.get(&p).unwrap().iter().map(|p1| (*p1, 1)).collect();

    shortest_distance(aa, zz, &mut neighb)
}

#[allow(unused)]
fn dump_adjacencies(adj: &AdjMap) {
    for (k, v) in adj.iter() {
        println!("{:3}, {:3} => {:?}", k.x, k.y, v);
    }
}

fn find_single_portal(labels: &LabelMap, name: &str) -> Point {
    match labels.get(name).map(Vec::as_slice) {
        Some([p1]) => *p1,
        other => panic!("expected one point at {:?}, got {:?}", name, other),
    }
}

/// Transform a list of portal locations to adjacencies between their entry
/// squares. Entry/exit portals with only one square are skipped.
fn add_portal_adjacencies(labels: &LabelMap, map: &mut AdjMap) {
    for (label, ps) in labels.iter() {
        match ps.as_slice() {
            [p1, p2] => {
                // Both p1 and p2 should already be known.
                map.get_mut(&p1).unwrap().push(*p2);
                map.get_mut(&p2).unwrap().push(*p1);
            }
            [_] => { /* entry or exit label */ }
            _ => panic!("unexpected {:?} {:?}", label, ps),
        };
    }
}

/// Return a map from points that are open passages, to other points that are
/// neighboring open passages.
fn find_square_adjacencies(mat: &Matrix<char>) -> AdjMap {
    let mut v = BTreeMap::new();
    for p0 in mat.iter_points() {
        if mat[p0] == PASSAGE {
            let a: Vec<Point> = mat
                .neighbors4(p0)
                .into_iter()
                .filter(|(_, c1)| **c1 == PASSAGE)
                .map(|(p1, _)| p1)
                .collect();
            if !a.is_empty() {
                assert!(v.insert(p0, a).is_none());
            }
        }
    }
    v
}

/// Return a map from labels to the 1 or 2 points they label.
fn find_labels(mat: &Matrix<char>) -> LabelMap {
    let mut v = LabelMap::new();
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
        if let Some(&cright) = mat.try_get(pright) {
            if cright.is_ascii_uppercase() {
                let name: String = [c1, cright].iter().collect();
                // Is there a dot to the left of these, or to the right?
                if p.x > 0 && mat[p.left()] == PASSAGE {
                    found(name, p.left());
                } else if pright.x < mat.width() && mat[pright.right()] == PASSAGE {
                    found(name, pright.right());
                } else {
                    panic!("confused at {:?}", p);
                }
                continue;
            }
        }
        if let Some(&cdown) = mat.try_get(pdown) {
            if cdown.is_ascii_uppercase() {
                let name = [c1, cdown].iter().collect();
                if p.y > 0 && mat[p.up()] == PASSAGE {
                    found(name, p.up());
                } else if pdown.y < mat.height() && mat[pdown.down()] == PASSAGE {
                    found(name, pdown.down());
                } else {
                    panic!("confused at {:?}", p);
                }
            }
        }
    }
    // dump_portals(&mut std::io::stdout(), &v);
    v
}

#[allow(unused)]
fn dump_portals(w: &mut dyn std::io::Write, portals: &LabelMap) {
    for (name, ps) in portals.iter() {
        writeln!(w, "{}: {:?}", name, ps).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a_1() {
        assert_eq!(solve_a_from_file("input/example_20_1.txt"), 23);
    }

    #[test]
    fn example_a_2() {
        assert_eq!(solve_a_from_file("input/example_20_2.txt"), 58);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 454);
    }
}
