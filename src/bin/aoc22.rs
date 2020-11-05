#![allow(dead_code)]
use std::{fmt, str::FromStr};

// 22b: The number of cards, and the number of iterations, are both clearly so high
// that we can't do them by brute force.
//
// However, all the operations on the deck actually look like arithmetic
// operations modulo N_CARDS. So, possibly we can form a polynomial from the
// instructions that says what original card is at position X. Then, possibly
// by raising that polynomial to B_ROUNDS we can find out, in closed form,
// the answer after many iterations.

const B_CARDS: u64 = 119315717514047;
const B_ROUNDS: u64 = 101741582076661;

pub fn main() {
    println!("22a: {}", solve_a());
}

fn solve_a() -> usize {
    let mut d = Deck::new(10007);
    d.eval(&std::fs::read_to_string("input/input22.txt").unwrap());
    d.d.iter().position(|x| *x == 2019).unwrap()
}

#[derive(Debug, Eq, PartialEq)]
enum Transform {
    Reverse,
    Add(isize),
    Multiply(usize),
}

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

struct Deck {
    d: Vec<usize>,
}

impl Deck {
    fn new(len: usize) -> Deck {
        Deck {
            d: (0..len).collect(),
        }
    }

    fn eval(&mut self, r: &str) {
        for instr in parse_input(r) {
            match instr {
                Transform::Reverse => self.deal_into_new_stack(),
                Transform::Add(i) => self.cut(i),
                Transform::Multiply(i) => self.deal_with_increment(i),
            }
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.d.reverse()
    }

    fn deal_with_increment(&mut self, incr: usize) {
        assert!(incr > 0);
        let len = self.d.len();
        let mut space: Vec<Option<usize>> = vec![None; len];
        let mut pos = 0;
        for &x in self.d.iter() {
            debug_assert!(space[pos].is_none());
            space[pos] = Some(x);
            pos = (pos + incr) % len;
        }
        self.d = space.into_iter().map(Option::unwrap).collect();
    }

    fn cut(&mut self, cutsz: isize) {
        let orig_len = self.d.len();
        let cutpt = if cutsz >= 0 {
            cutsz as usize
        } else {
            self.d.len() - ((-cutsz) as usize)
        };
        let mut bottom = self.d.split_off(cutpt);
        bottom.extend_from_slice(&self.d);
        debug_assert_eq!(orig_len, bottom.len());
        self.d = bottom;
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ds = self
            .d
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        write!(f, "{}", ds.join(" "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut d = Deck::new(10);
        d.eval(
            "deal with increment 7
            deal into new stack
            deal into new stack",
        );
        assert_eq!(d.to_string(), "0 3 6 9 2 5 8 1 4 7");

        let mut d = Deck::new(10);
        d.eval(
            "cut 6
            deal with increment 7
            deal into new stack",
        );
        assert_eq!(d.to_string(), "3 0 7 4 1 8 5 2 9 6");

        let mut d = Deck::new(10);
        d.eval(
            "\
            deal with increment 7
            deal with increment 9
            cut -2",
        );
        assert_eq!(d.to_string(), "6 3 0 7 4 1 8 5 2 9");
        let mut d = Deck::new(10);
        d.eval(
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
        );
        assert_eq!(d.to_string(), "9 2 5 8 1 4 7 0 3 6");
    }
}
