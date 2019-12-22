#![allow(unused_variables)]

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt;

use mbp_aoc2019::{Matrix, Point};

type Map = Matrix<char>;

const WALL: char = '#';
const PASSAGE: char = '.';
const PLAYER: char = '@';

pub fn main() {
    println!("18a: {}", solve_a());
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Keys {
    u: u32,
}

impl Keys {
    fn new() -> Keys {
        Keys { u: 0 }
    }

    fn offset(c: char) -> u32 {
        assert!(c >= 'a' && c <= 'z');
        (c as u32) - ('a' as u32)
    }

    fn set(&mut self, c: char) {
        self.u |= 1 << Keys::offset(c)
    }

    fn contains(&self, c: char) -> bool {
        let c = c.to_ascii_lowercase();
        self.u & (1 << Keys::offset(c)) != 0
    }
}

impl fmt::Debug for Keys {
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
/// Search state is represented by the sorted list of keys we've collected,
/// the distance travelled, and the current position.
#[derive(Eq, PartialEq, Debug, Clone)]
struct State {
    dist: isize,
    // TODO: Could make it Rc; probably many of them are identitical...
    keys: Keys,
    p: Point,
}

fn solve_a() -> isize {
    let mat = Matrix::from_string_lines(&std::fs::read_to_string("input/input18.txt").unwrap());
    let mut me = None;
    let mut all_keys = Keys::new();
    for p in mat.iter_points() {
        let c = mat[p];
        if c == PLAYER {
            me = Some(p)
        } else if c.is_ascii_lowercase() {
            all_keys.set(c);
        }
    }

    let me = me.unwrap();
    let initial = State {
        keys: Keys::new(),
        p: me,
        dist: 0,
    };

    // Queue of candidate states to check, sorted with the shortest live paths to the end.
    let mut queue = Vec::<(isize, State)>::new();
    queue.push((0, initial));

    // Spaces we've already visited and the shortest known distance.
    let mut best = BTreeMap::<(Point, Keys), isize>::new();
    best.insert((me, Keys::new()), 0);

    while let Some((d, st)) = queue.pop() {
        // println!("pop from queue to visit {:?}", st);
        // Consider every possible neighbor; skip them if we already know a shorter
        // path there.
        for newstate in neighbors(&mat, &st) {
            // println!("can move from {:?} to {:?}", st.p, newstate);
            // Do we already know a shorter path there?
            match best.entry((newstate.p, newstate.keys.clone())) {
                Entry::Vacant(v) => {
                    v.insert(newstate.dist);
                }
                Entry::Occupied(mut o) => {
                    if *o.get() <= newstate.dist {
                        continue;
                    } else {
                        o.insert(newstate.dist);
                    }
                }
            }
            // Since we found a new or shorter path, we need to look at continuations from it
            // later.
            // println!("queue to visit {:?}", &newstate);
            if newstate.keys == all_keys {
                return newstate.dist;
            }
            queue.push((newstate.dist, newstate));
        }
        // Sort with the shortest explored paths to the end.
        queue.sort_by_key_unstable(|(dist, state)| -dist);
        //  println!("queue length {}", queue.len());
    }
    unreachable!()
}

fn neighbors(mat: &Map, state: &State) -> Vec<State> {
    let mut v = Vec::new();
    let p = state.p;
    for &p in &[p.left(), p.right(), p.up(), p.down()] {
        if let Some(c) = mat.try_get(p) {
            let mut keys = state.keys.clone();
            if c == WALL {
                continue;
            } else if c == PASSAGE || c == PLAYER {
            } else if c.is_ascii_uppercase() {
                // A door: can we pass it?
                if !state.keys.contains(c) {
                    continue;
                }
            } else if c.is_ascii_lowercase() {
                // println!("collect key {} at {:?}", c, &p);
                keys.set(c);
            } else {
                panic!("unexpected {:?} at {:?}", c, state);
            }
            v.push(State {
                p,
                keys,
                dist: state.dist + 1,
            })
        }
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 4204);
    }
}
