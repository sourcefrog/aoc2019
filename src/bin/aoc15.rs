#![allow(dead_code, unused_mut, unused_variables)]
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


use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("15a: {}", solve_a());
    println!("15b: {}", solve_b());
}

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
enum Square {
    Empty,
    Wall,
    Oxygen,
}
use Square::*;

impl Square {
    fn from_output(i: isize) -> Self {
        match i {
            0 => Wall,
            1 => Empty,
            2 => Oxygen,
            _ => panic!("invalid square content {}", i),
        }
    }

    fn to_char(self) -> char {
        match self {
            Empty => '.',
            Wall => 'x',
            Oxygen => 'O',
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Move {
    North,
    South,
    West,
    East,
}
use Move::*;

impl Move {
    fn all_moves() -> &'static [Move] {
        &[North, South, West, East]
    }

    fn command(&self) -> isize {
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }

    fn move_from(&self, p: &Point) -> Point {
        match self {
            North => (p.0, p.1 - 1),
            South => (p.0, p.1 + 1),
            West => (p.0 - 1, p.1),
            East => (p.0 + 1, p.1),
        }
    }
}

type Point = (isize, isize);

fn solve_a() -> usize {
    Map::load().oxygen_depth
}

fn solve_b() -> usize {
    Map::load().flood_oxygen()
}

struct Map {
    seen: BTreeMap<Point, Square>,
    oxygen_depth: usize,
    oxygen_pos: Point,
}

impl Map {
    fn load() -> Map {
        // Naively, we'd want to do a breadth-first search from the starting point
        // until we find the oxygen.
        //
        // This is slightly complicated by not being able to jump to any arbitrary
        // point - except, we could store the computer state and jump there.
        //
        // We need to simultaneously discover the contents of the squares,
        // which squares are reachable, and also the length of the shorest path
        // to get there...
        //
        // By looking at all squares N positions away, before looking any deeper,
        // we should find the shortest path first.
        let orig_c = Computer::from_file("input/input15.txt");

        // Contents of all the squares we've seen.
        let mut seen: BTreeMap<Point, Square> = BTreeMap::new();

        // Squares we want to visit, preceded by their depth, and including
        // the computer state when in them.
        let mut queue: Vec<(usize, Point, Computer)> = Vec::new();
        queue.push((0, (0, 0), orig_c));

        let mut oxygen_depth = None;
        let mut oxygen_pos = None;

        while !queue.is_empty() {
            queue.sort_by_key(|a| a.0);
            // println!("queue len {}", queue.len());
            let (depth, oldpos, cpu) = queue.remove(0);

            // Try to visit every square neighboring `oldpos`, starting from `cpu`.
            for m in Move::all_moves() {
                let newpos = m.move_from(&oldpos);
                if seen.contains_key(&newpos) {
                    // println!("have already seen {:?}, skipping", &newpos);
                    continue;
                }

                let mut newcpu = cpu.clone();
                newcpu.push_input(m.command());
                newcpu.run();
                assert_eq!(newcpu.output_len(), 1);
                let sq = Square::from_output(newcpu.pop_output().unwrap());
                // println!(".. result: {:?}", sq);

                // println!( "move {:?} from {:?} to {:?} at depth {} => {:?}", m, oldpos, newpos, depth, sq);
                match sq {
                    Oxygen => {
                        // println!("found oxygen at {:?}", newpos);
                        oxygen_depth = Some(depth + 1);
                        oxygen_pos = Some(newpos);
                    }
                    Wall => {}
                    Empty => {
                        let newdepth = depth + 1;
                        queue.push((newdepth, newpos, newcpu));
                    }
                }

                seen.insert(newpos, sq);
            }
        }
        let map = Map {
            seen,
            oxygen_depth: oxygen_depth.unwrap(),
            oxygen_pos: oxygen_pos.unwrap(),
        };
        println!("{}", map.draw(&(0, 0)));
        map
    }

    fn draw(&self, pos: &Point) -> String {
        if self.seen.is_empty() {
            return String::new();
        }
        let xmin = self.seen.keys().map(|p| p.0).min().unwrap();
        let xmax = self.seen.keys().map(|p| p.0).max().unwrap();
        let ymin = self.seen.keys().map(|p| p.1).min().unwrap();
        let ymax = self.seen.keys().map(|p| p.1).max().unwrap();
        let mut s = String::new();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if *pos == (x, y) {
                    s.push('@')
                } else if let Some(sq) = self.seen.get(&(x, y)) {
                    s.push(sq.to_char());
                } else {
                    s.push(' ')
                }
            }
            s.push('\n');
        }
        s
    }

    fn flood_oxygen(&mut self) -> usize {
        // Locations of as-yet-incomplete oxygen.
        let mut oxy = vec![self.oxygen_pos];
        for i in 0.. {
            let mut new_oxy = Vec::new();
            for oxypos in oxy.into_iter() {
                for m in Move::all_moves() {
                    let qxy = m.move_from(&oxypos);
                    if let Entry::Occupied(mut o) = self.seen.entry(qxy) {
                        if *o.get() == Empty {
                            new_oxy.push(qxy);
                            o.insert(Oxygen);
                        }
                    }
                }
            }
            if new_oxy.is_empty() {
                return i;
            }
            oxy = new_oxy;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_and_return() {
        let mut cpu = Computer::from_file("input/input15.txt");
        cpu.push_input(1);
        cpu.push_input(2);
        cpu.run();
        assert_eq!(cpu.drain_output(), &[1, 1]);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 232);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 320);
    }
}
