#![allow(unused_imports, dead_code)]

use mbp_aoc2019::{point, Matrix, Point};

struct Maze {
    portals: Vec<([char; 2], Point)>,
}

pub fn main() {}

fn parse(s: &str) -> Maze {
    let mut mb = Matrix::<char>::from_rows();
    for l in s.lines() {
        let cv: Vec<char> = l.chars().collect();
        mb.add_row(&cv);
    }
    // Raw matrix of just uninterpreted chars.
    let mat = mb.finish();

    // Find labels
    let portals = find_labels(&mat);

    Maze { portals }
}

fn find_labels(mat: &Matrix<char>) -> Vec<([char; 2], Point)> {
    let mut v = Vec::new();
    // Trick here is not to be confused by finding the second character
    // of the labels...
    //
    // Walk over the matrix looking for letters.
    //
    // If we find one, look below it, or to the right of it, for another
    // letter. If there is one, that tells us there's a marker, and its
    // orientation. Then look at one square more, on either side, to find
    // a '.' marker, which tells us the location of the actual portal.
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
                if p.x > 0 && mat[p.left()] == '.' {
                    v.push((name, p.left()));
                } else if pright.x < mat.width() && mat[pright.right()] == '.' {
                    v.push((name, pright.right()));
                } else {
                    panic!("confused at {:?}", p);
                }
                continue;
            }
        }
        if let Some(&cdown) = mat.try_get(pdown) {
            if cdown.is_ascii_uppercase() {
                let name = [c1, cdown];
                if p.y > 0 && mat[p.up()] == '.' {
                    v.push((name, p.up()));
                } else if pdown.y < mat.height() && mat[pdown.down()] == '.' {
                    v.push((name, pdown.down()));
                } else {
                    panic!("confused at {:?}", p);
                }
            }
        }
    }
    dump_portals(&mut std::io::stdout(), &v);
    v
}

fn dump_portals(w: &mut dyn std::io::Write, portals: &[([char; 2], Point)]) {
    for (name, p) in portals {
        writeln!(w, "{}{}: {:3}, {:3}", name[0], name[1], p.x, p.y).unwrap();
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
