#![allow(dead_code)]
// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


// NOTE: This representation apparently does a lot of string copying. It could perhaps be
// eliminated by keeping the names just once in the struct, but that might not be worth it unless
// this's too slow.
use std::collections::BTreeMap;
use std::collections::VecDeque;

pub fn main() {
    println!("06a: {}", solve_a());
    println!("06b: {}", solve_b());
}

fn solve_a() -> usize {
    Map::from_string(&std::fs::read_to_string("input/input06.txt").unwrap()).count_orbits()
}

fn solve_b() -> usize {
    Map::from_string(&std::fs::read_to_string("input/input06.txt").unwrap()).transfer_distance()
}

#[derive(Debug)]
struct Map {
    /// Map from orbiting to orbited object. Each object only orbits one, except the CoM
    /// orbits nothing.
    orbits: BTreeMap<String, String>,

    /// Map from orbited to any number of orbiting objects.
    orbited: BTreeMap<String, Vec<String>>,

    com: String,
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
        // Find CoM: the orbited thing that does not orbit anything.
        let mut found = None;
        for a in orbited.keys() {
            if !orbits.contains_key(a) {
                if let Some(already) = found {
                    panic!("two apparent CoMs: {:?}, {:?}", already, a);
                }
                found = Some(a.clone())
            }
        }
        Map {
            orbits,
            orbited,
            com: found.unwrap(),
        }
    }

    /// Find the number of direct and indirect orbits.
    ///
    /// Basically: first find the one single object which orbits nothing else,
    /// (the CoM.) Then gradually build up a list of objects whose orbit counts
    /// are definitively known.
    fn count_orbits(&self) -> usize {
        let mut total = 0;
        let mut pend: VecDeque<(String, usize)> = VecDeque::new();
        pend.push_back((self.com.clone(), 0));
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

    /// Return a path from the COM to this object.
    pub fn path(&self, target: &str) -> Vec<String> {
        let mut v = vec![target.to_string()];
        while *v.last().unwrap() != self.com {
            v.push(self.orbits[v.last().unwrap()].clone());
        }
        v.reverse();
        v
    }

    pub fn transfer_distance(&self) -> usize {
        let you_path = self.path("YOU");
        let san_path = self.path("SAN");
        let common_len = common_prefix_len(you_path.iter(), san_path.iter());
        // Steps from you down to the common prefix, not counting your current position,
        // and then from the common prefix up to SAN, not counting san
        you_path.len() + san_path.len() - (2 * common_len) - 2
    }
}

/// Return the length of the common prefix between two slices.
///
/// This assumes there is a common prefix and a difference before the ends.
fn common_prefix_len<IA, IB, E>(mut ia: IA, mut ib: IB) -> usize
where
    IA: Iterator<Item = E>,
    IB: Iterator<Item = E>,
    E: Eq,
{
    let mut l = 0;
    while ia.next() == ib.next() {
        l += 1;
    }
    l
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
        assert_eq!(m.com, "COM");
        assert_eq!(m.count_orbits(), 42);
    }

    #[test]
    fn example_b() {
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
            K)L
            K)YOU
            I)SAN",
        );
        assert_eq!(
            m.path("YOU"),
            vec!["COM", "B", "C", "D", "E", "J", "K", "YOU"]
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 322_508);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 496);
    }
}
