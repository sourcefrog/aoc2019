#![allow(dead_code)]
use std::fmt;

pub fn main() {
    println!("22a: {}", solve_a());
}

fn solve_a() -> usize {
    let mut d = Deck::new(10007);
    d.eval(&std::fs::read_to_string("input/input22.txt").unwrap());
    d.d.iter().position(|x| *x == 2019).unwrap()
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
        for l in r.lines().map(str::trim) {
            if l == "deal into new stack" {
                self.deal_into_new_stack()
            } else if l.starts_with("deal with increment ") {
                self.deal_with_increment(l[20..].parse().unwrap())
            } else if l.starts_with("cut ") {
                self.cut(l[4..].parse().unwrap())
            } else if l.is_empty() {
            } else {
                panic!("huh? {:?}", l);
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
