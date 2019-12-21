#![allow(unused_imports, dead_code)]

use mbp_aoc2019::{point, Matrix, Point};
use std::collections::BTreeMap;

const PASSAGE: char = '.';

struct Maze {}

type AdjMap = BTreeMap<Point, Vec<Point>>;
type Label = [char; 2];
type LabelMap = BTreeMap<Label, Vec<Point>>;

pub fn main() {
    load("input/input20.txt");
}

fn parse(s: &str) -> Maze {
    let mut mb = Matrix::<char>::from_rows();
    for l in s.lines() {
        let cv: Vec<char> = l.chars().collect();
        mb.add_row(&cv);
    }
    // Raw matrix of just uninterpreted chars.
    let mat = mb.finish();

    let mut adj = find_square_adjacencies(&mat);
    let portals = find_labels(&mat);
    add_portal_adjacencies(&portals, &mut adj);

    for (k, v) in adj.iter() {
        println!("{:3}, {:3} => {:?}", k.x, k.y, v);
    }

    Maze {}
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
                let name = [c1, cright];
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
                let name = [c1, cdown];
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
    dump_portals(&mut std::io::stdout(), &v);
    v
}

#[allow(unused)]
fn dump_portals(w: &mut dyn std::io::Write, portals: &LabelMap) {
    for (name, ps) in portals.iter() {
        writeln!(w, "{}{}: {:?}", name[0], name[1], ps).unwrap();
    }
}

fn load(filename: &str) -> Maze {
    parse(&std::fs::read_to_string(filename).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let _maze = load("input/example_20_1.txt");
    }
}
