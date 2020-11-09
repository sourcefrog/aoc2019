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


use std::convert::TryInto;

const REPEATS: usize = 10_000;
const PHASES: usize = 100;
const OFFSET_LEN: usize = 7;
const OUTPUT_LEN: usize = 8;

pub fn main() {
    println!("{}", solve_a());
    println!("{}", solve_b());
}

fn solve_b() -> String {
    let input = &std::fs::read_to_string("input/input16.txt").unwrap();
    solve_type_b(&input)
}

fn solve_type_b(input: &str) -> String {
    let s = str_to_vi8(input);
    let offset = input[..OFFSET_LEN].parse().unwrap();
    let total_len = s.len() * REPEATS;

    // For offsets in this range, every coefficient to the left of `offset`
    // is 0, and everything to the right is 1. Therefore in each phase,
    // for all i in offset..=total_len, new_a[i] is just the sum of
    // old_a[i..]. We never need to look at anything left of offset.
    assert!(offset < total_len && offset * 2 > total_len);

    // Calculate the expanded vector, omitting everything left of offset.
    let mut a: Vec<i8> = (offset..total_len).map(|i| s[i % s.len()]).collect();
    // show_vec(&a);

    let ol = total_len - offset;
    debug_assert_eq!(ol, a.len());

    for _phase in 0..PHASES {
        let mut last = 0;
        for i in (0..ol).rev() {
            last = (last + a[i]) % 10;
            a[i] = last;
        }
        // show_vec(&a);
    }

    vi8_to_str(&a[..OUTPUT_LEN])
}

fn show_vec(a: &[i8]) {
    println!(
        "{} ... {}",
        vi8_to_str(&a[..30]),
        vi8_to_str(&a[(a.len() - 30)..])
    )
}

fn vi8_to_str(a: &[i8]) -> String {
    let mut s = String::new();
    for i in a {
        s.push_str(&i.to_string());
    }
    s
}

fn str_to_vi8(s: &str) -> Vec<i8> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}

fn fft1(a: &[i8]) -> Vec<i8> {
    let mut o = Vec::with_capacity(a.len());
    for (i, _x) in a.iter().enumerate() {
        let mut s: isize = 0;
        // Take blocks of length (i+1) every (i+1)*4, starting at i.
        for (j, y) in a.iter().enumerate() {
            let phase = ((j + 1) / (i + 1)) % 4;
            let mul = [0, 1, 0, -1][phase];
            // print!("{}*{} ", y, mul,);
            s += mul * (*y as isize);
        }
        let s = s.abs() % 10;
        // println!("= {}", s);
        o.push(s.try_into().unwrap());
    }
    o
}

fn solve_a() -> String {
    vi8_to_str(
        &fftn(
            str_to_vi8(&std::fs::read_to_string("input/input16.txt").unwrap()),
            100,
        )[..8],
    )
}

fn fftn(mut a: Vec<i8>, n: usize) -> Vec<i8> {
    for _ in 0..n {
        a = fft1(&a);
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        // fft1(&str_to_vi8("12345678"));
        assert_eq!(fft1(&str_to_vi8("12345678")), str_to_vi8("48226158"));

        assert_eq!(fftn(str_to_vi8("12345678"), 4), str_to_vi8("01029498"));
    }

    #[test]
    fn examples_b() {
        assert_eq!(
            &solve_type_b("03036732577212944063491565474664"),
            "84462026"
        );
        assert_eq!(
            &solve_type_b("02935109699940807407585447034323"),
            "78725270"
        );
        assert_eq!(
            &solve_type_b("03081770884921959731165446850517"),
            "53553731"
        );
    }

    #[test]
    fn expected_answer_b() {
        assert_eq!(&solve_b(), "96099551");
    }
}
