#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

use mbp_aoc2019::{Matrix, Point};

type Map = Matrix<char>;

type DistanceMap = BTreeMap<(char, char), usize>;

const WALL: char = '#';
const PASSAGE: char = '.';
const PLAYER: char = '@';

pub fn main() {
    println!("18a: {}", solve_a());
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct KeySet {
    u: u32,
}

impl KeySet {
    fn new() -> KeySet {
        KeySet { u: 0 }
    }

    fn offset(c: char) -> u32 {
        assert!(c >= 'a' && c <= 'z');
        (c as u32) - ('a' as u32)
    }

    fn set(&mut self, c: char) {
        self.u |= 1 << KeySet::offset(c)
    }

    fn contains(&self, c: char) -> bool {
        let c = c.to_ascii_lowercase();
        self.u & (1 << KeySet::offset(c)) != 0
    }

    fn len(&self) -> usize {
        self.u.count_ones() as usize
    }

    fn key_chars(&self) -> String {
        let mut s = String::with_capacity(26);
        for i in 0..26 {
            if self.u & (1 << i) != 0 {
                s.push(std::char::from_u32('a' as u32 + i).unwrap())
            }
        }
        s
    }

    fn door_chars(&self) -> String {
        let mut s = String::with_capacity(26);
        for i in 0..26 {
            if self.u & (1 << i) != 0 {
                s.push(std::char::from_u32('A' as u32 + i).unwrap())
            }
        }
        s
    }
}

impl fmt::Debug for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for i in 0..26 {
            if self.u & (1 << i) != 0 {
                write!(f, "{}", std::char::from_u32('a' as u32 + i).unwrap())?;
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}

fn solve_a() -> usize {
    solve_type_a(&std::fs::read_to_string("input/input18.txt").unwrap())
}

fn solve_type_a(s: &str) -> usize {
    let mat = Matrix::from_string_lines(s);
    let mut start = None;
    let mut n_keys = 0;
    let mut all_keys = KeySet::new();
    for p in mat.iter_points() {
        let c = mat[p];
        if c.is_ascii_lowercase() {
            n_keys += 1;
            all_keys.set(c);
        } else if c == PLAYER {
            start = Some(p);
        }
    }
    let start = start.unwrap();

    // The best-known distance and terminal point for any given keyset.
    let mut best: usize = std::usize::MAX;

    let mut queue: Vec<(usize, KeySet, Point)> = vec![(0, KeySet::new(), start)];
    let mut i = 0;
    while let Some((dist0, ks0, p0)) = queue.pop() {
        i += 1;
        if i % 1000 == 0 {
            println!("considered {:8} paths, queue length {}", i, queue.len());
        }
        // println!( "looking at distance      {:5} path {:26?} {:?}", dist0, ks0, p0);
        for (dist1, ks1, p1) in reachable(&mat, dist0, &ks0, p0) {
            // println!( "  could move to distance {:5} path {:26?} {:?}", dist1, ks1, p1);
            if ks1 == all_keys {
                best = std::cmp::min(best, dist1);
            }
            queue.push((dist1, ks1, p1));
        }
    }
    best
}

/// Return a vec of (distance, keyset, pos) for every new key reachable from
/// p given current KeySet.
fn reachable(mat: &Map, dist0: usize, ks: &KeySet, p: Point) -> Vec<(usize, KeySet, Point)> {
    let mut queue = vec![p];
    let mut seen = BTreeSet::new();
    let mut dist = dist0 + 1;
    let mut result = Vec::new();
    while !queue.is_empty() {
        // Find every as-yet-unvisited neighbor at this distance.
        let mut new_queue: Vec<Point> = Vec::new();
        for p1 in queue.into_iter() {
            for (p2, &c) in mat.neighbors4(p1) {
                if c == WALL || !seen.insert(p2) {
                    continue;
                }
                if c == PASSAGE || c == PLAYER || ks.contains(c.to_ascii_lowercase()) {
                    // Either empty, or we've already collected this key, or we have
                    // the key for this door.
                    new_queue.push(p2);
                } else if c.is_ascii_lowercase() {
                    // Found a new key. Take this; can't go further.
                    let mut ks1 = ks.clone();
                    ks1.set(c);
                    result.push((dist, ks1, p2));
                } else {
                    // A door for which we don't have the key.
                    debug_assert!(c.is_ascii_uppercase());
                }
            }
        }
        queue = new_queue;
        dist += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 4204);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            solve_type_a(
                "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
"
            ),
            86
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_type_a(
                "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
"
            ),
            132
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            solve_type_a(
                "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
"
            ),
            136
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            solve_type_a(
                "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
"
            ),
            81
        );
    }
}
