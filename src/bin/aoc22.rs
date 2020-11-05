#![allow(dead_code)]
use std::{convert::TryInto, fmt, str::FromStr};

// 22b: The number of cards, and the number of iterations, are both clearly so high
// that we can't do them by brute force.
//
// However, all the operations on the deck actually look like arithmetic
// operations modulo N_CARDS. So, possibly we can form a polynomial from the
// instructions that says what original card is at position X. Then, possibly
// by raising that polynomial to B_ROUNDS we can find out, in closed form,
// the answer after many iterations.
//
// The "deal with increment" operations, which are effectively multiplications,
// seem to be the hard part. And, I'm still not sure how this whole thing will
// be raised to an astronomical power.
//
// Maybe instead of thinking of individual cards moving, we should think of the whole
// vector being stretched, reversed, and rotated.
//
// Perhaps really this is just all an `ax + b` type transformation, of additive shifts
// and multiplications, where reversals are just a multiplication by -1.

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
    Add(i64),
    Multiply(i64),
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
fn card_at(position: i64, transforms: &[Transform], n_cards: i64) -> i64 {
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

struct Collapsed {
    a: i64,
    b: i64,
}

impl Collapsed {
    /// Given a list of transforms, collapse it into a single linear transform
    /// `ax + b` describing the final position of card `x.`.
    fn collapse(transforms: &[Transform]) -> Collapsed {
        let mut a = 1;
        let mut b = 0;
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
                    b -= i;
                }
            }
        }
        Collapsed { a, b }
    }
    /// Given a collapsed transform, produce the deck
    fn to_deck(&self, n_cards: i64) -> Vec<i64> {
        let mut r = vec![-1; n_cards as usize];
        for i in 0..n_cards {
            let pos = (self.a * i + self.b).rem_euclid(n_cards);
            let pos: usize = pos.try_into().unwrap();
            assert!(r[pos] == -1);
            r[pos] = i;
        }
        assert!(!r.iter().any(|i| *i < 0));
        r
    }
}

/// Given a list of transforms, find in closed form the final card in each position.
fn eval_all_cards(transforms: &[Transform], n_cards: i64) -> Vec<i64> {
    (0..n_cards)
        .map(|i| card_at(i, transforms, n_cards))
        .collect()
}

fn cards_to_string(cards: &[i64]) -> String {
    let s: Vec<String> = cards.iter().map(|c| c.to_string()).collect();
    s.join(" ")
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
                Reverse => self.deal_into_new_stack(),
                Add(i) => self.cut(i.try_into().unwrap()),
                Multiply(i) => self.deal_with_increment(i.try_into().unwrap()),
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

    fn check_eval(input: &str, n_cards: i64, expected: &str) {
        let transforms = parse_input(input);
        // let result = cards_to_string(&eval_all_cards(&transforms, n_cards));
        let collapse = Collapsed::collapse(&transforms);
        let result = cards_to_string(&collapse.to_deck(n_cards));
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
}
