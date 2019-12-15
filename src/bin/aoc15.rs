#![allow(dead_code, unused_mut, unused_variables)]

use std::collections::BTreeMap;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("15a: {}", solve_a());
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

    fn to_char(&self) -> char {
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

    loop {
        queue.sort_by_key(|a| a.0);
        println!("queue len {}", queue.len());
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

            println!(
                "move {:?} from {:?} to {:?} at depth {} => {:?}",
                m, oldpos, newpos, depth, sq
            );
            match sq {
                Oxygen => {
                    println!("found oxygen at {:?}", newpos);
                    return depth + 1;
                }
                Wall => {}
                Empty => {
                    let newdepth = depth + 1;
                    queue.push((newdepth, newpos, newcpu));
                }
            }

            seen.insert(newpos, sq);
        }

        println!("{}", draw(&seen, &oldpos));
    }
}

fn draw(map: &BTreeMap<Point, Square>, pos: &Point) -> String {
    if map.is_empty() {
        return String::new();
    }
    let xmin = map.keys().map(|p| p.0).min().unwrap();
    let xmax = map.keys().map(|p| p.0).max().unwrap();
    let ymin = map.keys().map(|p| p.1).min().unwrap();
    let ymax = map.keys().map(|p| p.1).max().unwrap();
    let mut s = String::new();
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if *pos == (x, y) {
                s.push('@')
            } else if let Some(sq) = map.get(&(x, y)) {
                s.push(sq.to_char());
            } else {
                s.push(' ')
            }
        }
        s.push('\n');
    }
    s
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
}
