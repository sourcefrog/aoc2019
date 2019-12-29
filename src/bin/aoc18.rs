#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt;

use mbp_aoc2019::{Matrix, Point};

type Map = Matrix<char>;

const WALL: char = '#';
const PASSAGE: char = '.';
const PLAYER: char = '@';

pub fn main() {
    // println!("18a: {}", solve_a());
    println!("18b: {}", solve_b());
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

    fn clear(&mut self, c: char) {
        self.u &= !(1 << KeySet::offset(c))
    }

    fn contains(&self, c: char) -> bool {
        let c = c.to_ascii_lowercase();
        self.u & (1 << KeySet::offset(c)) != 0
    }

    fn subtract(&mut self, other: &KeySet) {
        self.u &= !other.u;
    }

    fn is_empty(&self) -> bool {
        self.u == 0
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

    fn is_subset_of(&self, other: &KeySet) -> bool {
        self.u & other.u == self.u
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

fn solve_b() -> usize {
    solve_type_b(&std::fs::read_to_string("input/input18.txt").unwrap())
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

    // Best seen distance to collect all keys.
    let mut best_overall: usize = std::usize::MAX;

    // Process things in generations that discover all shortest paths of the same
    // length, going through any *gen* keys, and ending at each distinct key.
    let mut queue: BTreeMap<(KeySet, Point), usize> = BTreeMap::new();
    queue.insert((KeySet::new(), start), 0);
    for gen in 1..=n_keys {
        let mut next_queue: BTreeMap<(KeySet, Point), usize> = BTreeMap::new();
        for ((ks0, p0), dist0) in queue.into_iter() {
            for (dist1, ks1, _c1, p1) in reachable(&mat, dist0, &ks0, p0, &all_keys) {
                debug_assert_eq!(gen, ks1.len());
                if ks1.len() == n_keys {
                    best_overall = std::cmp::min(best_overall, dist1);
                }
                let newk = (ks1.clone(), p1);
                match next_queue.entry(newk) {
                    Entry::Vacant(v) => {
                        v.insert(dist1);
                    }
                    Entry::Occupied(mut o) => {
                        o.insert(std::cmp::min(*o.get(), dist1));
                    }
                };
            }
        }
        println!("generation {}: next queue len {}", gen, next_queue.len());
        queue = next_queue;
    }
    best_overall
}

fn solve_type_b(s: &str) -> usize {
    let mut mat = Matrix::from_string_lines(s);
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
    edit_for_b(&mut mat, start);
    let n_keys = all_keys.len();

    // Best seen distance to collect all keys.
    let mut best_overall: usize = std::usize::MAX;

    // Process things in generations that discover all shortest paths of the same
    // length, going through any *gen* keys, and ending at each distinct key.
    let mut queue: BTreeMap<(KeySet, [Point; 4]), usize> = BTreeMap::new();
    let starts = [
        start.left().up(),
        start.right().up(),
        start.left().down(),
        start.right().down(),
    ];
    queue.insert((KeySet::new(), starts), 0);
    for gen in 1..=n_keys {
        let mut next_queue = BTreeMap::new();
        for ((ks0, robots), dist0) in queue.into_iter() {
            for (i, &p0) in robots.iter().enumerate() {
                for (dist1, ks1, _c1, p1) in reachable(&mat, dist0, &ks0, p0, &all_keys) {
                    debug_assert_eq!(gen, ks1.len());
                    if ks1.len() == n_keys {
                        best_overall = std::cmp::min(best_overall, dist1);
                    }
                    let mut newbots = robots;
                    newbots[i] = p1;
                    let newk = (ks1.clone(), newbots);
                    match next_queue.entry(newk) {
                        Entry::Vacant(v) => {
                            v.insert(dist1);
                        }
                        Entry::Occupied(mut o) => {
                            o.insert(std::cmp::min(*o.get(), dist1));
                        }
                    };
                }
            }
        }
        println!("generation {}: next queue len {}", gen, next_queue.len());
        queue = next_queue;
    }
    best_overall
}

fn edit_for_b(mat: &mut Map, start: Point) {
    mat[start] = WALL;
    mat[start.left()] = WALL;
    mat[start.right()] = WALL;
    mat[start.up()] = WALL;
    mat[start.down()] = WALL;
    // We don't bother for now adding the individual markers.
}

/// Return a vec of (distance, keyset, key) for every new key reachable from
/// p given current KeySet.
fn reachable(
    mat: &Map,
    dist0: usize,
    ks: &KeySet,
    p: Point,
    all_keys: &KeySet,
) -> Vec<(usize, KeySet, char, Point)> {
    let mut queue = vec![p];
    let mut seen = Matrix::new(mat.width(), mat.height(), false);
    let mut dist = dist0 + 1;
    let mut result = Vec::new();
    let mut remaining_keys = all_keys.clone();
    remaining_keys.subtract(&ks);
    while !queue.is_empty() && !remaining_keys.is_empty() {
        // Find every as-yet-unvisited neighbor at this distance.
        let mut new_queue: Vec<Point> = Vec::new();
        for p1 in queue.into_iter() {
            for (p2, &c) in mat.neighbors4(p1) {
                if c == WALL || seen[p2] {
                    continue;
                }
                seen[p2] = true;
                if c == PASSAGE || c == PLAYER || ks.contains(c) {
                    // Either empty, or we've already collected this key, or we have
                    // the key for this door.
                    new_queue.push(p2);
                } else if c.is_ascii_lowercase() {
                    // Found a new key. Take this; can't go further.
                    let mut ks1 = ks.clone();
                    ks1.set(c);
                    result.push((dist, ks1, c, p2));
                    remaining_keys.clear(c);
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
