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

use std::iter::FromIterator;
use std::iter::IntoIterator;

/// Generate all permutations of the elements in l.
pub fn permutations<T, I>(l: I) -> Vec<Vec<T>>
where
    T: Clone,
    I: IntoIterator<Item = T>,
{
    let mut l = l.into_iter();
    if let Some(first) = l.next() {
        let rest = Vec::from_iter(l);
        if rest.is_empty() {
            vec![vec![first]]
        } else {
            // The last element inserted at every possible position,
            // in every possible permutation of the other members.
            let mut r = Vec::new();
            for p in permutations(rest) {
                for i in 0..=(p.len()) {
                    let mut q = p.clone();
                    q.insert(i, first.clone());
                    r.push(q);
                }
            }
            r
        }
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test {
    fn pts(n: usize) -> String {
        super::permutations(0..n)
            .iter()
            .map(|e| {
                e.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn permute() {
        assert_eq!(pts(0), "");
        assert_eq!(pts(1), "0");
        assert_eq!(pts(2), "01,10");
        assert_eq!(pts(3), "012,102,120,021,201,210");

        assert_eq!(super::permutations(0..6).len(), 6 * 5 * 4 * 3 * 2);
    }
}
