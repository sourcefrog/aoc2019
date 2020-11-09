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

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("09a: {}", solve_a());
    println!("09b: {}", solve_b());
}

fn solve_a() -> isize {
    let mut c = Computer::from_file("input/input09.txt");
    c.push_input(1);
    c.run();
    let o = c.drain_output();
    dbg!(&o);
    *o.last().unwrap()
}

fn solve_b() -> isize {
    let mut c = Computer::from_file("input/input09.txt");
    c.push_input(2);
    c.run();
    let o = c.drain_output();
    dbg!(&o);
    *o.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 2_789_104_029);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 32869);
    }
}
