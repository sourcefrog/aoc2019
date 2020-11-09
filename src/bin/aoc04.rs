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

pub fn main() {
    println!("04a: {}", solve_a());
    println!("04b: {}", solve_b());
}

/// Return digit d (counting from the left) of i.
pub fn digit(i: usize, d: usize) -> usize {
    (match d {
        0 => i / 100_000,
        1 => i / 10_000,
        2 => i / 1_000,
        3 => i / 100,
        4 => i / 10,
        5 => i,
        _ => panic!("invalid d"),
    }) % 10
}

fn solve_a() -> usize {
    iter_a().count()
}

fn iter_a() -> impl Iterator<Item = usize> {
    let (amin, amax) = load_input();
    // It is a six-digit number.
    // The value is within the range given in your puzzle input.
    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease;
    // they only ever increase or stay the same (like 111123 or 135679).
    (amin..=amax).filter(|&i| {
        // TODO: We could probably skip whole ranges on numbers based on these
        // constraints...
        let d = (
            digit(i, 0),
            digit(i, 1),
            digit(i, 2),
            digit(i, 3),
            digit(i, 4),
            digit(i, 5),
        );
        (d.0 <= d.1 && d.1 <= d.2 && d.2 <= d.3 && d.3 <= d.4 && d.4 <= d.5)
            && (d.0 == d.1 || d.1 == d.2 || d.2 == d.3 || d.3 == d.4 || d.4 == d.5)
    })
}

fn solve_b() -> usize {
    iter_b().count()
}

fn iter_b() -> impl Iterator<Item = usize> {
    let (amin, amax) = load_input();
    // It is a six-digit number.
    // The value is within the range given in your puzzle input.
    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease;
    // they only ever increase or stay the same (like 111123 or 135679).
    (amin..=amax).filter(|&i| filter_b(i))
}

/// True if i meets the criteria for part b.
fn filter_b(i: usize) -> bool {
    let mut prev = 0;
    let mut c = [0; 10];
    for j in 0..=5 {
        let di = digit(i, j);
        if di < prev {
            return false;
        }
        c[di] += 1;
        prev = di;
    }
    c.iter().any(|&j| j == 2)
}

fn load_input() -> (usize, usize) {
    let v: Vec<usize> = std::fs::read_to_string("input/input04.txt")
        .unwrap()
        .trim()
        .split('-')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    (v[0], v[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(load_input(), (178_416, 676_461));
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 1650);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1129);
    }
}
