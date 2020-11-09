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

extern crate bytecount;

use std::convert::TryInto;

pub fn main() {
    println!("08a: {}", solve_a());
    println!("08b:\n{}", solve_b());
}

fn solve_a() -> usize {
    let im = Image::from_string(&load_input(), 25, 6);
    // Find the layer with the fewest zeros.
    let least_zeros_layer = (0..(im.l))
        .map(|i| (im.count_in_layer(0, i), i))
        .min()
        .unwrap()
        .1;
    im.count_in_layer(1, least_zeros_layer) * im.count_in_layer(2, least_zeros_layer)
}

fn solve_b() -> String {
    Image::from_string(&load_input(), 25, 6).composite()
}

struct Image {
    digits: Vec<u8>,
    w: usize,
    h: usize,
    /// Number of layers
    l: usize,
}

impl Image {
    pub fn from_string(s: &str, w: usize, h: usize) -> Image {
        let s = s.trim();
        // Check it's an even number of planes
        assert_eq!(s.len() % (w * h), 0);
        Image {
            digits: s
                .chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect(),
            w,
            h,
            l: s.len() / (w * h),
        }
    }

    /// Return a reference to a slice of the image in the given layer.
    pub fn layer(&self, p: usize) -> &[u8] {
        assert!(p < self.l);
        let sz = self.w * self.h;
        let a = sz * p;
        &self.digits[a..(a + sz)]
    }

    /// Return the count of digits equal to `d` within layer `l`.
    pub fn count_in_layer(&self, d: u8, l: usize) -> usize {
        bytecount::count(self.layer(l), d)
    }

    /// Combine all the layers of this image, from the bottom up, to
    /// produce a single result, as a text string.
    pub fn composite(&self) -> String {
        let sz = self.w * self.h;
        let mut r = vec![2; sz];
        // Neat little hack here: we can just walk all the values in order,
        // applying them to the corresponding modulus address, if they are not
        // transparent.
        for (i, v) in self.digits.iter().enumerate().rev() {
            if *v != 2 {
                r[i % sz] = *v
            }
        }

        let mut s = String::with_capacity(sz + self.h);
        let mut i = 0;
        for _y in 0..(self.h) {
            for _x in 0..(self.w) {
                s.push(match r[i] {
                    0 => ' ',
                    1 => '*',
                    _ => '.',
                });
                i += 1;
            }
            s.push('\n');
        }
        s
    }
}

fn load_input() -> String {
    std::fs::read_to_string("input/input08.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 2032);
    }

    #[test]
    fn solution_b() {
        solve_b();
        // No assertion about the content; let's just make sure it completes.
    }
}
