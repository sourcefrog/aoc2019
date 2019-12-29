#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
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
    let mut all_keys = KeySet::new();
    let mut key_pos: BTreeMap<char, Point> = BTreeMap::new();
    for p in mat.iter_points() {
        let c = mat[p];
        if c.is_ascii_lowercase() {
            all_keys.set(c);
            key_pos.insert(c, p);
        } else if c == PLAYER {
            key_pos.insert(c, p);
        }
    }
    let start = *key_pos.get(&'@').unwrap();
    let n_keys = all_keys.len();

    // The best-known distance for any given keyset and terminal point. We might
    // be interested to continue searching from the same point for various
    // different prior keysets, or different final points (at different
    // distances) for the same keyset. But there would be no point ever using 
    // anything but the shortest path to get to a given point with a given set of
    // keys collected. 
    // 
    // We don't care what path or order we took on the way to get there - in fact
    // it's desirable to collapse them, to reduce the search space.
    //
    // We could go one step further, too: there's also no point searching from
    // that point for any smaller keyset that can be achieved with the same 
    // distance.
    let mut besties: BTreeMap<Point, BTreeMap<KeySet, usize>> = BTreeMap::new();
    let mut p0best = BTreeMap::new();
    p0best.insert(KeySet::new(), 0);
    besties.insert(start.clone(), p0best);

    // Best seen distance to collect all keys.
    let mut best_overall: usize = std::usize::MAX;

    // Points/keysets that we want to do another search from.
    let mut queue: VecDeque<(Point, KeySet)> = vec![(start, KeySet::new())].into();
    let mut i = 0;

    while let Some((p0, ks0)) = queue.pop_front() {
        let dist0 = besties[&p0][&ks0];
        i += 1;
        if i % 1000 == 0 {
            println!("considered {:8} paths, queue length {}", i, queue.len());
        }
        // println!( "looking at distance      {:5} path {:26?} {:?}", dist0, ks0, p0);
        for (dist1, ks1, c1, p1) in reachable(&mat, dist0, &ks0, p0) {
            // println!( "  could move to {} at distance {:5} path {:26?} {:?}", c1, dist1, ks1, p1);
            // Is this a good and novel way to get to c1?
            let cbest = besties.entry(p1).or_default();
            if *cbest.get(&ks1).unwrap_or(&std::usize::MAX) > dist1 {
                // println!("    new better route to {} with {:30} of length {:6}", c1, ks1.key_chars(), dist1);
                if ks1 == all_keys {
                    best_overall = std::cmp::min(best_overall, dist1);
                }
                cbest.insert(ks1.clone(), dist1);
                let newqent = (p1, ks1);
                if !queue.contains(&newqent) {
                    queue.push_back(newqent);
                }
            }
        }
    }
    println!("final best result: {}", best_overall);
    best_overall
}

/// Return a vec of (distance, keyset, key) for every new key reachable from
/// p given current KeySet.
fn reachable(mat: &Map, dist0: usize, ks: &KeySet, p: Point) -> Vec<(usize, KeySet, char, Point)> {
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
                } else if c == PASSAGE || c == PLAYER || ks.contains(c.to_ascii_lowercase()) {
                    // Either empty, or we've already collected this key, or we have
                    // the key for this door.
                    new_queue.push(p2);
                } else if c.is_ascii_lowercase() {
                    // Found a new key. Take this; can't go further.
                    let mut ks1 = ks.clone();
                    ks1.set(c);
                    result.push((dist, ks1, c, p2));
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
