#![allow(dead_code)]

use std::collections::BTreeMap;
use std::collections::VecDeque;

pub fn main() {
    println!("06a: {}", solve_a());
}

fn solve_a() -> usize {
    Map::from_string(&std::fs::read_to_string("input/input06.txt").unwrap()).count_orbits()
}

#[derive(Debug)]
struct Map {
    /// Map from orbiting to orbited object. Each object only orbits one, except the CoM
    /// orbits nothing.
    orbits: BTreeMap<String, String>,

    /// Map from orbited to any number of orbiting objects.
    orbited: BTreeMap<String, Vec<String>>,
}

impl Map {
    pub fn from_string(s: &str) -> Map {
        let mut orbits: BTreeMap<String, String> = BTreeMap::new();
        let mut orbited: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for l in s.lines().map(str::trim) {
            let mut splitit = l.split(')');
            let a = splitit.next().unwrap();
            let b = splitit.next().unwrap();
            assert!(splitit.next().is_none());
            assert!(
                orbits.insert(b.to_string(), a.to_string()).is_none(),
                "value {:?} is already present",
                b
            );
            orbited
                .entry(a.to_string())
                .or_default()
                .push(b.to_string());
        }
        Map { orbits, orbited }
    }

    /// Find the one object that is orbited, but does not orbit anything itself.
    pub fn find_com(&self) -> String {
        let mut found = None;
        for a in self.orbits.values() {
            if !self.orbits.contains_key(a) {
                if let Some(already) = found {
                    panic!("two apparent CoMs: {:?}, {:?}", already, a);
                }
                found = Some(a.clone())
            }
        }
        found.unwrap()
    }

    /// Find the number of direct and indirect inputs.
    ///
    /// Basically: first find the one single object which orbits nothing else,
    /// (the CoM.) Then gradually build up a list of objects whose orbit counts
    /// are definitively known.
    fn count_orbits(&self) -> usize {
        let mut total = 0;
        let mut pend: VecDeque<(String, usize)> = VecDeque::new();
        pend.push_back((self.find_com(), 0));
        while let Some((b, depth)) = pend.pop_front() {
            total += depth;
            // Visit everything that orbits b
            if let Some(avec) = self.orbited.get(&b) {
                for a in avec {
                    pend.push_back((a.clone(), depth + 1))
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let m = Map::from_string(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
        );
        dbg!(&m);
        assert_eq!(m.find_com(), "COM");
        assert_eq!(m.count_orbits(), 42);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 322_508);
    }
}
