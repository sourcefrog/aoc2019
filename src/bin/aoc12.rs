#![allow(dead_code)]

use std::cmp::Ordering;
use std::fmt;

pub fn main() {
    dbg!(solve_a());
}

fn solve_a() -> isize {
    let mut ms = make_moons(&[(4, 1, 1), (11, -18, -1), (-2, -10, -4), (-7, -2, 14)]);
    for _ in 0..1000 {
        step(&mut ms);
    }
    total_energy(&ms)
}

struct Moon {
    p: (isize, isize, isize),
    v: (isize, isize, isize),
}

impl Moon {
    fn new(p: (isize, isize, isize)) -> Moon {
        Moon { p, v: (0, 0, 0) }
    }

    fn energy(&self) -> isize {
        (self.p.0.abs() + self.p.1.abs() + self.p.2.abs())
            * (self.v.0.abs() + self.v.1.abs() + self.v.2.abs())
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.p.0, self.p.1, self.p.2, self.v.0, self.v.1, self.v.2
        )
    }
}

fn ordering_to_int(ord: Ordering) -> isize {
    match ord {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

fn step(ms: &mut [Moon]) {
    // gravity
    // Use indexes to avoid multiply borrowing ms.
    for i in 0..ms.len() {
        for j in 0..ms.len() {
            if i != j {
                let pi = ms[i].p;
                let mut mj = &mut ms[j];

                mj.v.0 += ordering_to_int(pi.0.cmp(&mj.p.0));
                mj.v.1 += ordering_to_int(pi.1.cmp(&mj.p.1));
                mj.v.2 += ordering_to_int(pi.2.cmp(&mj.p.2));
            }
        }
    }

    // velocity
    for m in ms.iter_mut() {
        m.p.0 += m.v.0;
        m.p.1 += m.v.1;
        m.p.2 += m.v.2;
    }
}

fn make_moons(ms: &[(isize, isize, isize)]) -> Vec<Moon> {
    ms.iter().cloned().map(Moon::new).collect()
}

fn total_energy(ms: &[Moon]) -> isize {
    ms.iter().map(Moon::energy).sum::<isize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn example() {
        let mut ms = make_moons(&[(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)]);

        // after 1 step
        step(&mut ms);
        assert_eq!(ms[0].p, (2, -1, 1));
        assert_eq!(ms[0].v, (3, -1, -1));

        assert_eq!(ms[1].v, (1, 3, 3));
        assert_eq!(ms[1].p, (3, -7, -4));

        assert_eq!(ms[2].v, (-3, 1, -3));
        assert_eq!(ms[2].p, (1, -7, 5));

        assert_eq!(ms[3].p, (2, 2, 0));
        assert_eq!(ms[3].v, (-1, -3, 1));

        for _ in 2..=10 {
            step(&mut ms);
        }

        assert_eq!(ms.iter().map(Moon::energy).sum::<isize>(), 179);
    }

    #[test]
    fn example_a2() {
        let mut ms = make_moons(&[(-8, -10, 0), (5, 5, 10), (2, -7, 3), (9, -8, -3)]);
        for _i in 1..=100 {
            step(&mut ms);
        }
        assert_eq!(total_energy(&ms), 1940);
    }
}
