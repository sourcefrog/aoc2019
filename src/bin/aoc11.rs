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

use std::collections::BTreeMap;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("{}", solve_a());
    solve_b();
}

fn solve_a() -> usize {
    // contains squares that have been painted, and their color.
    // false or unset is black; true is white.
    let mut painted = BTreeMap::new();
    paint_it(&mut painted);
    painted.len()
}

fn solve_b() {
    let mut painted = Map::new();
    painted.insert((0, 0), true);
    paint_it(&mut painted);
    for y in -10..10 {
        for x in -20..40 {
            let p = painted.get(&(x, y)).copied().unwrap_or_default();
            print!("{}", if p { "*" } else { " " });
        }
        println!();
    }
}

type Map = BTreeMap<(isize, isize), bool>;

fn paint_it(painted: &mut Map) {
    let mut pos = (0, 0);
    // 0=up, 1=left, etc.
    let mut dir = 0;
    let mut c = Computer::from_file("input/input11.txt");
    loop {
        c.clear_input(); // in case it wasn't read
        c.push_input(painted.get(&pos).cloned().unwrap_or_default().into());
        if let Some(paint_this) = c.run_until_output() {
            // println!("paint {}", paint_this);
            assert!(paint_this == 0 || paint_this == 1);
            painted.insert(pos, paint_this != 0);
        } else {
            break;
        }
        if let Some(turn) = c.run_until_output() {
            // println!("turn {}", turn);
            dir = match turn {
                0 => (dir + 3) % 4,
                1 => (dir + 1) % 4,
                _ => panic!(),
            };
            pos = match dir {
                0 => (pos.0, pos.1 - 1),
                1 => (pos.0 + 1, pos.1),
                2 => (pos.0, pos.1 + 1),
                3 => (pos.0 - 1, pos.1),
                _ => panic!("bad dir {}", dir),
            };
        } else {
            break;
        }
    }
}
