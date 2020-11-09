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

use std::{convert::TryInto, str::FromStr};

use modinverse::modinverse;
use num_integer::gcd;

// 22b: The number of cards, and the number of iterations, are both clearly so
// high that we can't do them by brute force.
//
// However, all the operations on the deck actually look like arithmetic
// operations modulo N_CARDS. So, possibly we can form a polynomial from the
// instructions that says what original card is at position X. Then, possibly by
// raising that polynomial to B_ROUNDS we can find out, in closed form, the
// answer after many iterations.
//
// The "deal with increment" operations, which are effectively multiplications,
// seem to be the hard part. And, I'm still not sure how this whole thing will
// be raised to an astronomical power.
//
// Maybe instead of thinking of individual cards moving, we should think of the
// whole vector being stretched, reversed, and rotated.
//
// In fact really this is just all an `ax + b` affine transformation, of
// additive shifts and multiplications, where reversals are just a
// multiplication by -1.
//
// Furthermore, repeating the instructions is also just a multiplication of the
// transformations by the number of repetitions.o
//
// Both the number of cards and the number of rounds are prime, which can't be a
// coincidence.
//
// However interestingly the computed a and b do have factors, but no common
// factors:
//
// a = 2453562856896 = 2^6×3×23×281×1977251 (10 prime factors, 5 distinct)
// b = 26048896043585 = 5×2053×2537642089 (3 distinct prime factors)
//
// (Obviously they will have factors, as they're the product of some of the input
// values.)

const B_CARDS: i128 = 119315717514047;
const B_ROUNDS: i128 = 101741582076661;

pub fn main() {
    println!("22a: {}", solve_a());
    println!("22b: {}", solve_b());
}

fn load_input() -> String {
    std::fs::read_to_string("input/input22.txt").unwrap()
}

fn solve_a() -> i128 {
    let transforms = parse_input(&load_input());
    let collapsed = Fold::new(&transforms, 10007);
    collapsed.position_of_card(2019)
}

fn solve_b() -> i128 {
    let transforms = parse_input(&load_input());
    let fold = Fold::new(&transforms, B_CARDS);
    let bigfold = fold.exponent(B_ROUNDS);
    bigfold.card_in_position(2020)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Transform {
    Reverse,
    Add(i128),
    Multiply(i128),
}
use Transform::*;

#[derive(Debug)]
struct ParseInstructionError {}

impl FromStr for Transform {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "deal into new stack" {
            Ok(Transform::Reverse)
        } else if s.starts_with("deal with increment ") {
            s[20..]
                .parse()
                .map_err(|_| ParseInstructionError {})
                .map(|i| Transform::Multiply(i))
        } else if s.starts_with("cut ") {
            s[4..]
                .parse()
                .map_err(|_| ParseInstructionError {})
                .map(|i| Transform::Add(i))
        } else {
            Err(ParseInstructionError {})
        }
    }
}

fn parse_input(s: &str) -> Vec<Transform> {
    s.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(Transform::from_str)
        .map(Result::unwrap)
        .collect()
}

/// Given a list of transforms, find which card number ends up in a given position.
fn card_at(position: i128, transforms: &[Transform], n_cards: i128) -> i128 {
    // Originally, every card is in the position with its own number.
    let mut c = position;
    println!("eval card_at {}", position);
    for t in transforms.iter().rev() {
        let old_c = c;
        assert!(c >= 0);
        assert!(c < n_cards, "{} is too big before evaluating {:?}", c, t);
        c = match *t {
            Reverse => n_cards - 1 - c,
            Add(i) => (c + n_cards + i) % n_cards,
            Multiply(_i) => todo!(),
        };
        println!("eval {:?}, {} => {}", t, old_c, c);
        assert!(c >= 0);
    }
    c
}

/// Transforms applied to the original deck so that card `i` ends up in position
/// `(a * i + b).rem_euclid(n)`,
#[derive(Debug, Eq, PartialEq, Clone)]
struct Fold {
    a: i128,
    b: i128,
    n: i128,
}

impl Fold {
    /// Given a list of transforms, collapse it into a single linear transform
    /// `ax + b` describing the final position of card `x.`.
    fn new(transforms: &[Transform], n: i128) -> Fold {
        let mut a: i128 = 1;
        let mut b: i128 = 0;
        for t in transforms {
            match t {
                Reverse => {
                    a = -a;
                    b = -b - 1;
                }
                Multiply(i) => {
                    a *= i;
                    b *= i
                }
                Add(i) => {
                    b -= *i;
                }
            }
            a = a.rem_euclid(n);
            b = b.rem_euclid(n);
        }
        Fold { a, b, n }
    }

    fn identity(n: i128) -> Fold {
        Fold { a: 1, b: 0, n }
    }

    /// Given a collapsed transform, produce the deck
    fn to_deck(&self) -> Vec<i128> {
        let mut r = vec![-1; self.n as usize];
        for i in 0..(self.n) {
            let pos = self.position_of_card(i);
            let pos: usize = pos.try_into().unwrap();
            assert!(r[pos] == -1);
            r[pos] = i;
        }
        assert!(!r.iter().any(|i| *i < 0));
        r
    }

    fn position_of_card(&self, card: i128) -> i128 {
        (self.a * card + self.b).rem_euclid(self.n)
    }

    /// Given a collapsed transform, what card is in a given position?
    fn card_in_position(&self, pos: i128) -> i128 {
        assert!(pos >= 0);
        assert!(pos < self.n);
        // dbg!(pos, self);
        let x = (pos - self.b).rem_euclid(self.n);
        assert_eq!(gcd(self.a, self.n), 1);
        // dbg!(gcd(self.a, self.n));
        // dbg!(self.a / self.n, self.a % self.n);
        debug_assert_eq!(self.a, self.a.rem_euclid(self.n));
        let inv = modinverse(self.a, self.n).expect("no modular inverse");
        // Now we need to find: what number, multiplied by a, equals pos, modulo self.n?
        (inv * x).rem_euclid(self.n)
    }

    // Calculate the fold that would result from applying this twice
    fn double(&self) -> Fold {
        // pos = (a * c + b)
        // pos^2 = (a * (a * c + b) + b)
        // === a^2*c + ab + b
        Fold {
            a: (self.a * self.a).rem_euclid(self.n),
            b: ((self.a + 1) * self.b).rem_euclid(self.n),
            n: self.n,
        }
    }

    // Calculate the fold that results from applying two folds successively.
    fn multiply(&self, other: &Fold) -> Fold {
        // pos = (a1 * (a2 * c + b2) + b1)
        // === a1*a2*c + a1*b2 + b1
        assert_eq!(self.n, other.n);
        Fold {
            a: (self.a * other.a).rem_euclid(self.n),
            b: (self.a * other.b + self.b).rem_euclid(self.n),
            n: self.n,
        }
    }

    // Raise this fold to an exponent.
    fn exponent(&self, mut exp: i128) -> Fold {
        let mut result = Fold::identity(self.n);
        assert!(exp > 0);
        let mut base = self.clone();
        while exp > 0 {
            if (exp & 1) == 1 {
                result = result.multiply(&base);
            }
            base = base.double();
            exp = exp >> 1;
        }
        result
    }
}

fn cards_to_string(cards: &[i128]) -> String {
    let s: Vec<String> = cards.iter().map(|c| c.to_string()).collect();
    s.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_eval(input: &str, n_cards: i128, expected: &str) {
        let transforms = parse_input(input);
        let collapse = Fold::new(&transforms, n_cards);
        let result = cards_to_string(&collapse.to_deck());
        assert_eq!(result, expected, "wrong result for {}", input);
    }

    #[test]
    fn simple_reverse() {
        check_eval("deal into new stack", 10, "9 8 7 6 5 4 3 2 1 0");
    }

    #[test]
    fn double_reverse() {
        check_eval(
            "deal into new stack
            deal into new stack",
            10,
            "0 1 2 3 4 5 6 7 8 9",
        );
    }

    #[test]
    fn cut_positive() {
        check_eval("cut 2", 10, "2 3 4 5 6 7 8 9 0 1")
    }

    #[test]
    fn cut_negative() {
        check_eval("cut -4", 10, "6 7 8 9 0 1 2 3 4 5")
    }

    #[test]
    fn deal() {
        check_eval("deal with increment 3", 10, "0 7 4 1 8 5 2 9 6 3")
    }

    #[test]
    fn more_collapsed_examples() {
        check_eval(
            "deal with increment 7
            deal into new stack
            deal into new stack",
            10,
            "0 3 6 9 2 5 8 1 4 7",
        );

        check_eval(
            "cut 6
            deal with increment 7
            deal into new stack",
            10,
            "3 0 7 4 1 8 5 2 9 6",
        );

        check_eval(
            "\
            deal with increment 7
            deal with increment 9
            cut -2",
            10,
            "6 3 0 7 4 1 8 5 2 9",
        );

        check_eval(
            "\
            deal into new stack
            cut -2
            deal with increment 7
            cut 8
            cut -4
            deal with increment 7
            cut 3
            deal with increment 9
            deal with increment 3
            cut -1",
            10,
            "9 2 5 8 1 4 7 0 3 6",
        );
    }

    #[test]
    fn solve_a_collapsed() {
        let fold = Fold::new(&parse_input(&load_input()), 10007);
        assert_eq!(fold.position_of_card(2019), 3749);
        let deck = fold.to_deck();
        // what is the position of card 2019?
        let pos = deck.iter().position(|c| *c == 2019).unwrap();
        assert_eq!(pos, 3749);
        assert_eq!(fold.card_in_position(3749), 2019);
    }

    #[test]
    fn multiplying_folds() {
        let fold = Fold::new(&parse_input(&load_input()), B_CARDS);
        assert_eq!(fold.card_in_position(2020), 36005810730586);
        let fold2 = fold.double();
        assert_eq!(fold2.card_in_position(2020), 73091068072643);
        let fold4 = fold2.double();
        assert_eq!(fold4.card_in_position(2020), 46845718844529);

        // try multiply
        let fold3 = fold.multiply(&fold2);
        assert_eq!(fold3.card_in_position(2020), 48659237259576);

        let fold3b = fold2.multiply(&fold);
        assert_eq!(fold3b.card_in_position(2020), 48659237259576);

        assert_eq!(fold3, fold3b);

        // Can calculate directly as an exponent
        assert_eq!(fold.exponent(3), fold3);
        let fold8 = fold4.double();
        let fold10 = fold8.multiply(&fold2);
        assert_eq!(fold.exponent(8), fold8);
        assert_eq!(fold.exponent(10), fold10);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 77225522112241);
    }
}
